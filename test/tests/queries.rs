mod support;

use std::fs;

use support::{FIXTURE, matches, query, read, render, root};

#[test]
fn every_query_compiles_against_the_grammar() {
    let queries: Vec<_> = fs::read_dir(root().join("languages/bats"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "scm"))
        .collect();

    assert!(queries.len() >= 9, "{queries:?}");
    for path in queries {
        query(path.file_stem().unwrap().to_str().unwrap());
    }
}

#[test]
fn highlights_are_the_grammars_own() {
    assert!(
        read("languages/bats/highlights.scm") == tree_sitter_bats::HIGHLIGHTS_QUERY,
        "languages/bats/highlights.scm differs from queries/highlights.scm at the pinned tree-sitter-bats rev; copy it over"
    );
}

#[test]
fn the_fixture_parses_cleanly() {
    let tree = support::parse(&read(FIXTURE));
    assert!(
        !tree.root_node().has_error(),
        "{}",
        tree.root_node().to_sexp()
    );
}

#[test]
fn runnables() {
    insta::assert_snapshot!(render(&matches("runnables", &read(FIXTURE))));
}

/// Zed drops a runnable whose @run capture reaches the end of the buffer
/// (`MultiBufferSnapshot::runnable_ranges` keeps `run_range.end < range.end`),
/// so its button never appears.
#[test]
fn every_run_capture_ends_before_the_end_of_the_file() {
    let source = read(FIXTURE);
    for found in matches("runnables", &source) {
        for capture in found
            .captures
            .iter()
            .filter(|capture| capture.name == "run")
        {
            assert!(
                capture.end < source.len(),
                "{:?} @run at {}:{} reaches the end of the file",
                found.properties,
                capture.row + 1,
                capture.column + 1
            );
        }
    }
}

#[test]
fn outline() {
    insta::assert_snapshot!(render(&matches("outline", &read(FIXTURE))));
}

#[test]
fn textobjects() {
    insta::assert_snapshot!(render(&matches("textobjects", &read(FIXTURE))));
}

#[test]
fn brackets() {
    insta::assert_snapshot!(render(&matches("brackets", &read(FIXTURE))));
}

#[test]
fn indents() {
    insta::assert_snapshot!(render(&matches("indents", &read(FIXTURE))));
}

#[test]
fn overrides() {
    insta::assert_snapshot!(render(&matches("overrides", &read(FIXTURE))));
}

#[test]
fn injections() {
    insta::assert_snapshot!(render(&matches("injections", &read(FIXTURE))));
}

/// Catches upstream highlight changes when the grammar is re-pinned.
/// The highlights snapshot only guards what the fixture exercises, so it
/// must exercise every pattern.
#[test]
fn the_fixture_exercises_every_highlight_pattern() {
    let query = query("highlights");
    let hit: Vec<usize> = matches("highlights", &read(FIXTURE))
        .iter()
        .map(|found| found.pattern)
        .collect();
    let missed: Vec<String> = (0..query.pattern_count())
        .filter(|pattern| !hit.contains(pattern))
        .map(|pattern| {
            let start = query.start_byte_for_pattern(pattern);
            let end = query.end_byte_for_pattern(pattern);
            read("languages/bats/highlights.scm")[start..end]
                .trim()
                .to_string()
        })
        .collect();
    assert!(
        missed.is_empty(),
        "unmatched highlight patterns:\n{}",
        missed.join("\n")
    );
}

#[test]
fn highlights() {
    insta::assert_snapshot!(render(&matches("highlights", &read(FIXTURE))));
}
