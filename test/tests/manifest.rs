use std::{fs, path::PathBuf};

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

#[test]
fn harness_tests_the_grammar_revision_the_extension_pins() {
    let extension = read_toml("../extension.toml");
    let harness = read_toml("Cargo.toml");
    let pinned = get(&extension, &["grammars", "bats"]);
    let tested = get(&harness, &["dev-dependencies", "tree-sitter-bats"]);

    assert_eq!(pinned["repository"].as_str(), tested["git"].as_str());
    assert_eq!(pinned["rev"].as_str(), tested["rev"].as_str());
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
