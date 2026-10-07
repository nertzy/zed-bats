# Contributing

Thanks for helping improve zed-bats.

## Development

The extension itself has no Rust code: Zed reads `extension.toml` and the query and config files under `languages/bats/`. The tests live in a separate Cargo package under `test/`, outside the extension root, because Zed builds any `Cargo.toml` at the root as a Rust extension. They need a stable Rust toolchain and a C compiler to build the grammar.

```sh
cd test
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

`cargo fmt` fixes formatting findings.

The grammar is pinned by commit in two places that must match: `[grammars.bats]` in `extension.toml` and the `tree-sitter-bats` dependency in `test/Cargo.toml`. The tests fail when they differ.

To try a change in Zed, run `zed: install dev extension` and choose your checkout. After editing a query, run `zed: rebuild dev extension` or reinstall it.

Add or update tests for behavior changes. Pull requests should explain the problem, the chosen behavior, and any user-visible documentation changes.

## Reporting bugs and proposing changes

Use [GitHub issues](https://github.com/nertzy/zed-bats/issues) for reproducible bugs and focused proposals. For vulnerabilities, follow [SECURITY.md](SECURITY.md) instead of opening a public issue.
