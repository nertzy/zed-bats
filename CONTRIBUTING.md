# Contributing

Thanks for helping improve zed-bats.

## Development

The extension itself has no Rust code: Zed reads `extension.toml` and the query and config files under `languages/bats/`. The tests live in a separate Cargo package under `test/`, outside the extension root, because Zed builds any `Cargo.toml` at the root as a Rust extension. They need a stable Rust toolchain and a C compiler to build the grammar.

```sh
cd test
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
shellcheck --shell=bash ../tasks/run-test.bash
```

`cargo fmt` fixes formatting findings.

The query tests snapshot each Zed query's matches against `test/fixtures/example.bats` with [insta](https://insta.rs). After an intended change, review new snapshots with `cargo insta review`, or accept them with `INSTA_UPDATE=always cargo test`. The task tests run the real `bats` (1.5 or later) on the fixture, so it must be on your `PATH`.

`languages/bats/highlights.scm` is a verbatim copy of `queries/highlights.scm` from the pinned tree-sitter-bats commit; change highlighting there, then re-pin and copy. The tests fail when the copy differs.

The single-test task embeds `tasks/run-test.bash` as its `command` string. Zed replaces `$ZED_` variable references in a command before running it, so the script reads them with `printenv` and must not mention them; the tests fail if substitution would change it. Edit the script, check it with `shellcheck --shell=bash tasks/run-test.bash`, then regenerate the task:

```sh
jq --rawfile script tasks/run-test.bash \
  '(.[] | select(.tags == ["bats-test"]) | .command) = $script' \
  languages/bats/tasks.json > tasks.json.new && mv tasks.json.new languages/bats/tasks.json
```

The grammar is pinned by commit in two places that must match: `[grammars.bats]` in `extension.toml` and the `tree-sitter-bats` dependency in `test/Cargo.toml`. The tests fail when they differ.

To try a change in Zed, run `zed: install dev extension` and choose your checkout. After editing a query, run `zed: rebuild dev extension` or reinstall it.

Add or update tests for behavior changes. Pull requests should explain the problem, the chosen behavior, and any user-visible documentation changes.

## Reporting bugs and proposing changes

Use [GitHub issues](https://github.com/nertzy/zed-bats/issues) for reproducible bugs and focused proposals. For vulnerabilities, follow [SECURITY.md](SECURITY.md) instead of opening a public issue.
