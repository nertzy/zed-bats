# Contributing

Thanks for helping improve zed-bats.

## Development

The extension itself has no Rust code: Zed reads `extension.toml` and the query and config files under `languages/bats/`. The tests live in a separate Cargo package under `test/`, outside the extension root, because Zed builds any `Cargo.toml` at the root as a Rust extension. They need a stable Rust toolchain and a C compiler to build the grammar.

```sh
cd test
cargo test
cargo fmt --check
cargo clippy --all-targets -- -D warnings
bats tasks.bats
```

`cargo fmt` fixes formatting findings.

The query tests snapshot each Zed query's matches against `test/fixtures/example.bats` with [insta](https://insta.rs). After an intended change, review new snapshots with `cargo insta review`, or accept them with `INSTA_UPDATE=always cargo test`. The task tests run the real `bats` (1.5 or later) on the fixture, so it must be on your `PATH`, along with `jq`.

`languages/bats/highlights.scm` is a verbatim copy of `queries/highlights.scm` from the pinned tree-sitter-bats commit; change highlighting there, then re-pin and copy. The tests fail when the copy differs.

The task commands live in `languages/bats/tasks.json`. `test/tasks.bats` runs them with the variables Zed exports to a task, and the Rust task tests check how Zed fills those variables in from each run button and spawns the shell. Zed replaces `$ZED_` references in a command before the shell sees it, so the commands read Zed's variables with `printenv`; the tests fail if substitution would change a command. Check an edited command with shellcheck:

```sh
jq -r '.[0].command' languages/bats/tasks.json | shellcheck --shell=bash -
```

Zed builds the grammar from the commit `[grammars.bats]` pins in `extension.toml`; the tests use the `tree-sitter-bats` release from crates.io that `test/Cargo.toml` names. Bump both together: the tests fail unless the crate was published from the pinned commit, as its `.cargo_vcs_info.json` records.

To try a change in Zed, run `zed: install dev extension` and choose your checkout. After editing a query, run `zed: rebuild dev extension` or reinstall it.

Add or update tests for behavior changes. Pull requests should explain the problem, the chosen behavior, and any user-visible documentation changes.

## Releasing

Merging to `main` does not reach Zed users. Zed's [extension registry](https://github.com/zed-industries/extensions) pins this repository as the `extensions/bats` submodule and lists its version under `[bats]` in `extensions.toml`, so a release takes two steps:

1. Release here. Bump `version` in `extension.toml`, date the CHANGELOG's Unreleased section, and merge. Then push a signed `vX.Y.Z` tag on the merge commit and create a GitHub release from the CHANGELOG entry.
2. Publishing the release runs the [Zed registry workflow](.github/workflows/registry.yml), which uses [zed-extension-action](https://github.com/huacnlee/zed-extension-action) to open a pull request to `zed-industries/extensions` that [updates the extension](https://zed.dev/docs/extensions/publishing/updating-and-maintenance): it moves the `extensions/bats` submodule to the tagged commit and sets `[bats] version` from the tag. It pushes the `bats-vX.Y.Z` branch to the `nertzy/extensions` fork with the `COMMITTER_TOKEN` secret, a classic personal access token with the `repo` and `workflow` scopes. Prereleases are skipped.

   If the workflow fails, check `nertzy/extensions` for a `bats-vX.Y.Z` branch before rerunning its job from the Actions tab. The action always creates that branch, so a rerun fails while it exists; delete it first unless a pull request was already opened from it.

   That repository's [AI policy](https://github.com/zed-industries/extensions/blob/main/AI_POLICY.md) bans pull requests from autonomous agents and requires a human to write pull request text and review replies.

Zed publishes the new version once that pull request merges.

## Reporting bugs and proposing changes

Use [GitHub issues](https://github.com/nertzy/zed-bats/issues) for reproducible bugs and focused proposals. For vulnerabilities, follow [SECURITY.md](SECURITY.md) instead of opening a public issue.
