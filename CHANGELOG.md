# Changelog

Notable changes to `zed-bats` are documented here.

## [Unreleased]

## [0.1.0] - 2026-10-08

### Added

- Added a Bats language for `.bats` files and files with a `bats` shebang, using the tree-sitter-bats 0.1.0 grammar.
- Added run buttons that run a single test, by its name, or the whole file with `bats`, including functions Bats runs as tests because of a `# @test` comment.
- Added highlighting (tree-sitter-bats's own query), an outline of tests and functions, Vim text objects, brackets, indentation, comment injections, and redactions.
