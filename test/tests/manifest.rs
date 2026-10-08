use std::{fs, path::PathBuf, process::Command};

fn read_toml(relative: &str) -> toml::Table {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative);
    let text =
        fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    text.parse()
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn get<'a>(table: &'a toml::Table, path: &[&str]) -> &'a toml::Value {
    path[1..]
        .iter()
        .fold(&table[path[0]], |value, key| &value[*key])
}

/// The directory Cargo unpacked the tree-sitter-bats crate into.
fn grammar_crate_dir() -> PathBuf {
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--locked"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let package = metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|package| package["name"] == "tree-sitter-bats")
        .unwrap();
    PathBuf::from(package["manifest_path"].as_str().unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
}

/// crates.io keeps the commit a crate was published from in
/// `.cargo_vcs_info.json`, so the harness tests the grammar Zed builds.
#[test]
fn harness_tests_the_grammar_revision_the_extension_pins() {
    let extension = read_toml("../extension.toml");
    let pinned = get(&extension, &["grammars", "bats"]);
    let path = grammar_crate_dir().join(".cargo_vcs_info.json");
    let text =
        fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let vcs_info: serde_json::Value = serde_json::from_str(&text).unwrap();

    assert_eq!(
        pinned["repository"].as_str(),
        Some("https://github.com/nertzy/tree-sitter-bats")
    );
    assert_eq!(pinned["rev"].as_str(), vcs_info["git"]["sha1"].as_str());
}

#[test]
fn language_uses_the_pinned_grammar() {
    let extension = read_toml("../extension.toml");
    let language = read_toml("../languages/bats/config.toml");
    let grammar = language["grammar"].as_str().unwrap();

    assert!(get(&extension, &["grammars"]).get(grammar).is_some());
}

#[test]
fn grammar_parses_a_bats_file() {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_bats::LANGUAGE.into())
        .unwrap();
    let tree = parser
        .parse(
            "@test \"adds\" {\n  run expr 1 + 1\n  [ \"$output\" -eq 2 ]\n}\n",
            None,
        )
        .unwrap();

    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
    assert_eq!(tree.root_node().child(0).unwrap().kind(), "test_block");
}
