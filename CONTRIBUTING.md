# Contributing

Contributions are welcome. Please open an issue for larger changes before
implementing them.

Before submitting a pull request, run:

```sh
cargo fmt
cargo clippy -- -D warnings
cargo test
```

Keep changes focused, add tests for new behavior, and ensure the CLI remains
usable on Windows, macOS, and Linux.
