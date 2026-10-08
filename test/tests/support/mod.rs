#![allow(dead_code, reason = "each test file uses a different subset")]

use std::{fs, path::PathBuf};

use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor, Tree};

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

pub fn read(relative: &str) -> String {
    let path = root().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

pub fn language() -> tree_sitter::Language {
    tree_sitter_bats::LANGUAGE.into()
}

pub fn parse(source: &str) -> Tree {
    let mut parser = Parser::new();
    parser.set_language(&language()).unwrap();
    parser.parse(source, None).unwrap()
}

pub fn query(name: &str) -> Query {
    let source = read(&format!("languages/bats/{name}.scm"));
    Query::new(&language(), &source).unwrap_or_else(|error| panic!("{name}.scm: {error}"))
}

pub const FIXTURE: &str = "test/fixtures/example.bats";

/// One query match: the `#set!` properties of its pattern and its captures.
#[derive(Debug)]
pub struct Match {
    pub pattern: usize,
    pub properties: Vec<String>,
    pub captures: Vec<Capture>,
}

#[derive(Debug)]
pub struct Capture {
    pub name: String,
    pub row: usize,
    pub column: usize,
    /// Byte offset just past the captured node.
    pub end: usize,
    pub text: String,
}

pub fn matches(query_name: &str, source: &str) -> Vec<Match> {
    let query = query(query_name);
    let tree = parse(source);
    let mut cursor = QueryCursor::new();
    let mut found = Vec::new();
    let mut iter = cursor.matches(&query, tree.root_node(), source.as_bytes());
    while let Some(found_match) = iter.next() {
        let properties = query
            .property_settings(found_match.pattern_index)
            .iter()
            .map(|property| {
                format!(
                    "{} {}",
                    property.key,
                    property.value.as_deref().unwrap_or("")
                )
            })
            .collect();
        let captures = found_match
            .captures()
            .iter()
            .map(|capture| {
                let start = capture.node.start_position();
                Capture {
                    name: query.capture_names()[capture.index as usize].to_string(),
                    row: start.row,
                    column: start.column,
                    end: capture.node.end_byte(),
                    text: capture
                        .node
                        .utf8_text(source.as_bytes())
                        .unwrap()
                        .to_string(),
                }
            })
            .collect();
        found.push(Match {
            pattern: found_match.pattern_index,
            properties,
            captures,
        });
    }
    found
}

/// A readable listing of every match, for snapshots. Multi-line capture text
/// shows only its first line.
pub fn render(matches: &[Match]) -> String {
    matches
        .iter()
        .map(|found| {
            let header = match found.properties.as_slice() {
                [] => String::new(),
                properties => format!("({})\n", properties.join(", ")),
            };
            let captures = found
                .captures
                .iter()
                .map(|capture| {
                    let mut lines = capture.text.lines();
                    let first = lines.next().unwrap_or("");
                    let more = if lines.next().is_some() { " …" } else { "" };
                    format!(
                        "  @{} {}:{} {first}{more}",
                        capture.name,
                        capture.row + 1,
                        capture.column + 1
                    )
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("{header}{captures}")
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
