# hat-specifications-cli

Read and validate an explicitly supplied HAT declaration or package and evaluate
its fitting against a profile. Every response is bounded JSON with
`external_actions: false`; validation never grants authority or executes an effect.

```sh
cargo run --locked -p hat-specifications-cli -- doctor
cargo run --locked -p hat-specifications-cli -- validate examples/source-curator.hat.toml
```

This is a standalone application built from the owning workspace. The reusable
contracts are the separate `hat-specifications` library; this executable is not
published to crates.io. [Product documentation](../../README.md) · [License](LICENSE)
