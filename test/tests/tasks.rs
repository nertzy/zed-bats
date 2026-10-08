mod support;

use std::{
    collections::HashMap,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

use serde_json::Value;
use support::{FIXTURE, Match, matches, read, root};

fn tasks() -> Vec<Value> {
    serde_json::from_str::<Vec<Value>>(&read("languages/bats/tasks.json")).unwrap()
}

fn task(tag: &str) -> Value {
    tasks()
        .into_iter()
        .find(|task| task["tags"].as_array().unwrap().iter().any(|t| t == tag))
        .unwrap_or_else(|| panic!("no task tagged {tag}"))
}

/// Substitutes task variables the way Zed does: `ZED_` variables resolve or
/// fail, and any other `$variable` is left for the shell.
fn substitute(template: &str, variables: &HashMap<String, String>) -> Result<String, String> {
    shellexpand::env_with_context(template, |name: &str| {
        let variable = &name[..name.find(':').unwrap_or(name.len())];
        match variables.get(variable) {
            Some(value) => Ok(Some(value.clone())),
            None if variable.starts_with("ZED_") => Err(format!("unknown {variable}")),
            None if variable.len() < name.len() => Ok(Some(format!("${{{name}}}"))),
            None => Ok(None),
        }
    })
    .map(|expanded| expanded.into_owned())
    .map_err(|error| error.to_string())
}

/// Spawns a task the way Zed does on macOS and Linux: substitute variables
/// in the command, paste it unquoted into `<shell> <shell args> -i -c`
/// (`ShellBuilder::build_no_quote`), and export the task variables. Zed
/// offers a task only when its label resolves.
fn run(task: &Value, variables: &HashMap<String, String>) -> Output {
    substitute(task["label"].as_str().unwrap(), variables).unwrap();
    assert!(
        task.get("args").is_none(),
        "Zed pastes args into the command unquoted"
    );
    let shell = &task["shell"]["with_arguments"];
    let command = substitute(task["command"].as_str().unwrap(), variables).unwrap();
    let shell_args: Vec<&str> = shell["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|arg| arg.as_str().unwrap())
        .collect();

    let mut bash = Command::new(shell["program"].as_str().unwrap());
    // Detach from the terminal, or an interactive bash that shares it with
    // an interactive cargo test can stop itself with SIGTTIN.
    // SAFETY: setsid is async-signal-safe.
    unsafe {
        bash.pre_exec(|| match libc::setsid() {
            -1 => Err(std::io::Error::last_os_error()),
            _ => Ok(()),
        });
    }
    // Zed exports its task variables to every task, so a test run from a Zed
    // terminal inherits them; each case sets only its own.
    for (name, _) in std::env::vars_os() {
        if name.to_string_lossy().starts_with("ZED_") {
            bash.env_remove(name);
        }
    }
    bash.args(shell_args)
        .args(["-i", "-c", &command])
        .envs(variables)
        .env("HOME", "/home/zed-bats")
        .stdin(Stdio::null())
        .output()
        .expect("bash and bats on PATH")
}

/// The variables Zed gives a task run from `runnable`: the file and row, and
/// each capture other than @run as `ZED_CUSTOM_<capture>`.
fn variables(file: &Path, runnable: &Match) -> HashMap<String, String> {
    let row = runnable
        .captures
        .iter()
        .find(|capture| capture.name == "run")
        .unwrap()
        .row
        + 1;
    let mut variables = HashMap::from([
        ("ZED_FILE".to_string(), file.display().to_string()),
        (
            "ZED_FILENAME".to_string(),
            file.file_name().unwrap().to_string_lossy().into_owned(),
        ),
        ("ZED_ROW".to_string(), row.to_string()),
    ]);
    for capture in runnable
        .captures
        .iter()
        .filter(|capture| capture.name != "run")
    {
        variables.insert(format!("ZED_CUSTOM_{}", capture.name), capture.text.clone());
    }
    variables
}

fn fixture() -> PathBuf {
    root().join(FIXTURE).canonicalize().unwrap()
}

fn runnables(tag: &str) -> Vec<Match> {
    matches("runnables", &read(FIXTURE))
        .into_iter()
        .filter(|found| found.properties == [format!("tag {tag}")])
        .collect()
}

#[test]
fn substitution_leaves_the_commands_untouched() {
    let variables = HashMap::from([
        ("ZED_FILE".to_string(), "/a b/$x.bats".to_string()),
        (
            "ZED_CUSTOM_BATS_TEST_NAME".to_string(),
            "\"$y\"".to_string(),
        ),
    ]);
    for task in tasks() {
        let command = task["command"].as_str().unwrap();
        assert_eq!(substitute(command, &variables).unwrap(), command);
    }
}

#[test]
fn the_test_task_needs_a_test_name() {
    let variables = HashMap::from([("ZED_FILENAME".to_string(), "example.bats".to_string())]);
    assert!(substitute(task("bats-test")["label"].as_str().unwrap(), &variables).is_err());
}

#[test]
fn tasks_run_in_bash_without_rc_files_whatever_the_users_shell() {
    for task in tasks() {
        assert_eq!(task["shell"]["with_arguments"]["program"], "bash");
        assert_eq!(
            task["shell"]["with_arguments"]["args"],
            serde_json::json!(["--norc", "--noprofile"])
        );
    }
}

/// Runs the bats-test task from each runnable in the fixture, as Zed would,
/// and checks that exactly that test ran.
#[test]
fn each_runnable_runs_exactly_its_test() {
    let task = task("bats-test");
    let expected = [
        "greets by name",
        r"matches a.b* (literally) [x] {y} ^$ | +? \ too",
        "unquoted name with words",
        "greets",
        "matches /home/zed-bats literally",
        "one line",
        "empty",
        "marked_by_comment",
    ];
    let runnables = runnables("bats-test");
    assert_eq!(runnables.len(), expected.len());

    for (runnable, name) in runnables.iter().zip(expected) {
        let output = run(&task, &variables(&fixture(), runnable));

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            format!("1..1\nok 1 {name}\n"),
            "{runnable:?}; stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn the_file_task_runs_the_whole_file() {
    let [runnable] = &runnables("bats-file")[..] else {
        panic!("expected one bats-file runnable");
    };
    let output = run(&task("bats-file"), &variables(&fixture(), runnable));

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("1..8\n"));
}
