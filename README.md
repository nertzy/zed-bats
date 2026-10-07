# zed-bats

[![CI][ci]](https://github.com/nertzy/zed-bats/actions/workflows/ci.yml)

[Bats](https://github.com/bats-core/bats-core) (Bash Automated Testing System) support for [Zed](https://zed.dev): run each `@test` from the gutter, with outline and highlighting, built on [tree-sitter-bats](https://github.com/nertzy/tree-sitter-bats).

## Status

Early development. The extension registers a Bats language for `.bats` files and files with a `bats` shebang; highlighting, outline, and test runnables land with the first implementation pull request.

## Install

The extension isn't in Zed's extension registry yet. Install it from a checkout:

1. `git clone https://github.com/nertzy/zed-bats`
2. In Zed, run `zed: install dev extension` and choose the checkout.

Zed compiles the grammar on install; it downloads the WASI SDK it needs the first time.

## Scope and limitations

Zed's built-in Shell Script language also claims `.bats` files. Zed prefers the most recently registered language when two match equally well, so with this extension installed, `.bats` files open as Bats. To keep a file as Shell Script, map it in the `file_types` setting.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT. See [LICENSE](LICENSE).

[ci]: https://github.com/nertzy/zed-bats/actions/workflows/ci.yml/badge.svg
