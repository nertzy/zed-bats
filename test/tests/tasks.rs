mod support;

use std::{
    collections::HashMap,
    process::{Command, Output, Stdio},
};

use serde_json::Value;
use support::{FIXTURE, matches, read, root};

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
fn substitute(template: &str, variables: &HashMap<&str, String>) -> String {
    shellexpand::env_with_context(template, |name: &str| {
        let variable = &name[..name.find(':').unwrap_or(name.len())];
        match variables.get(variable) {
            Some(value) => Ok(Some(value.clone())),
            None if variable.starts_with("ZED_") => Err(format!("unknown {variable}")),
            None if variable.len() < name.len() => Ok(Some(format!("${{{name}}}"))),
            None => Ok(None),
        }
    })
    .unwrap_or_else(|error| panic!("{template}: {error}"))
    .into_owned()
}

/// Spawns a task the way Zed does on macOS and Linux: substitute variables
/// in the command, paste it unquoted into `<shell> <shell args> -i -c`
/// (`ShellBuilder::build_no_quote`), and export the task variables.
fn run(task: &Value, variables: &HashMap<&str, String>) -> Output {
    assert!(
        task.get("args").is_none(),
        "Zed pastes args into the command unquoted"
    );
    let shell = &task["shell"]["with_arguments"];
    let command = substitute(task["command"].as_str().unwrap(), variables);
    let shell_args: Vec<&str> = shell["args"]
        .as_array()
        .unwrap()
        .iter()
        .map(|arg| arg.as_str().unwrap())
        .collect();

    Command::new(shell["program"].as_str().unwrap())
        .args(shell_args)
        .args(["-i", "-c", &command])
        .envs(variables)
        .env("HOME", "/home/zed-bats")
        .stdin(Stdio::null())
        .output()
        .expect("bash and bats on PATH")
}

#[test]
fn the_test_task_runs_the_checked_in_script() {
    assert!(
        task("bats-test")["command"] == read("tasks/run-test.bash"),
        "tasks.json is stale; see CONTRIBUTING.md to regenerate it from tasks/run-test.bash"
    );
}

#[test]
fn substitution_leaves_the_scripts_untouched() {
    let variables = HashMap::from([
        ("ZED_FILE", "/a b/$x.bats".to_string()),
        ("ZED_ROW", "7".to_string()),
    ]);
    for task in tasks() {
        let command = task["command"].as_str().unwrap();
        assert_eq!(substitute(command, &variables), command);
    }
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
    let source = read(FIXTURE);
    let file = root().join(FIXTURE).canonicalize().unwrap();
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

    let rows: Vec<usize> = matches("runnables", &source)
        .iter()
        .filter(|found| found.properties == ["tag bats-test"])
        .map(|found| {
            found
                .captures
                .iter()
                .find(|capture| capture.name == "run")
                .unwrap()
                .row
                + 1
        })
        .collect();
    assert_eq!(rows.len(), expected.len());

    for (row, name) in rows.iter().zip(expected) {
        let variables = HashMap::from([
            ("ZED_FILE", file.display().to_string()),
            ("ZED_ROW", row.to_string()),
        ]);
        let output = run(&task, &variables);

        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            format!("1..1\nok 1 {name}\n"),
            "row {row}; stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn the_test_task_reports_a_row_above_every_test() {
    let file = root().join(FIXTURE).canonicalize().unwrap();
    let variables = HashMap::from([
        ("ZED_FILE", file.display().to_string()),
        ("ZED_ROW", "2".to_string()),
    ]);
    let output = run(&task("bats-test"), &variables);

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("No Bats test starts on or above line 2"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn the_file_task_runs_the_whole_file() {
    let file = root().join(FIXTURE).canonicalize().unwrap();
    let variables = HashMap::from([("ZED_FILE", file.display().to_string())]);
    let output = run(&task("bats-file"), &variables);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("1..8\n"));
}
