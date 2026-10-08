# zed-bats

[![CI][ci]](https://github.com/nertzy/zed-bats/actions/workflows/ci.yml)

[Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System) support for [Zed](https://zed.dev): run each `@test` from the gutter, with outline and highlighting, built on [tree-sitter-bats](https://github.com/nertzy/tree-sitter-bats).

## Features

- **Run a test from the gutter.** Each `@test`, and each function Bats runs as a test (`name() { # @test`), gets a run button. It runs exactly that test: the button hands the task the test's name as written, and the task strips its quotes as Bats does and passes it to `bats --filter` as an escaped, anchored regex.
- **Run the file.** A run button on the first line runs `bats` on the whole file.
- **Highlighting** from tree-sitter-bats's own `highlights.scm`: Bats keywords, helpers (`run`, `load`, `skip`, …), hooks (`setup`, `teardown`, …), and variables (`$status`, `$output`, `$lines`, `$BATS_*`) on top of Bash.
- **Outline** of tests, hooks, and functions, for the outline panel and `editor: toggle outline`. Functions show by bare name, as bash-language-server lists them; one Bats runs as a test because of a `# @test` comment shows as `name # @test`.
- **Vim text objects and motions**: tests and functions are functions (`af`, `if`, `]m`), adjacent comments are one comment.
- **Brackets, indentation, comment toggling, and redaction** of assigned values while screen sharing.

The run buttons need `bats` on the `PATH` Zed loads from your login shell. Tasks run from the project root, in `bash --norc --noprofile` whatever your shell is, so your shell startup files don't run first.

### Use a different `bats`

Zed binds run buttons to tasks by tag: `bats-test` for a test, `bats-file` for a file. When your project's `.zed/tasks.json` or your global tasks has a task with the same tag, the run button uses yours instead of the extension's. For a project that vendors Bats as a submodule:

```json
[
  {
    "label": "bats $ZED_FILENAME",
    "command": "test/bats/bin/bats \"$ZED_FILE\"",
    "tags": ["bats-file"]
  }
]
```

The single-test task gets the test's name, as written between `@test` and `{`, in `ZED_CUSTOM_BATS_TEST_NAME`. To change it, copy the `bats-test` task from [`languages/bats/tasks.json`](languages/bats/tasks.json) and edit its `bats` command.

## Install

Open Zed's Extensions page (`zed: extensions`), search for Bats, and install it.

To try unreleased changes, install it from a checkout instead:

1. `git clone https://github.com/nertzy/zed-bats`
2. In Zed, run `zed: install dev extension` and choose the checkout.

Zed compiles the grammar on install; it downloads the WASI SDK it needs the first time. A dev extension overrides the published one until you uninstall it.

## Scope and limitations

Zed's built-in Shell Script language also claims `.bats` files. Zed prefers the most recently registered language when two match equally well, so with this extension installed, `.bats` files open as Bats. To keep a file as Shell Script, map it in the `file_types` setting.

The extension has no language server. Bash's language server attaches only to Shell Script files.

Parsing comes from [tree-sitter-bats](https://github.com/nertzy/tree-sitter-bats); its README lists the Bash constructs it doesn't parse yet.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. See [LICENSE](LICENSE).

[ci]: https://github.com/nertzy/zed-bats/actions/workflows/ci.yml/badge.svg
