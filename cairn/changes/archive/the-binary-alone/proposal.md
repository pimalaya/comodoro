---
cairn: change
id: the-binary-alone
status: landed
created: 2026-11-21
---

# The binary alone

## Why

Comodoro shipped a library beside its binary: a no_std contract (timer, JSON-RPC 2.0 envelope, protocol), a blocking client and server behind `client` and `server`, and the CLI itself behind `cli`. Nothing in the family consumes it, ortie and neverest are binaries only, and every CLI change became a public library break (`cli::account::Account`, `AccountConfig::socket`, `TimerServerCommand::execute`). The owner ruled on 2026-11-21 to remove lib.rs and the `cli` feature, keep `notify` and `vendored`, and keep the CLI only. The contract third parties implement is the wire protocol, specified in spec/protocol.md for any language.

## What

- src/lib.rs goes; src/main.rs declares every module and becomes the architecture header. The crate is std: `#![no_std]`, `extern crate alloc` and the `core`/`alloc` imports go.
- `client::std` and `server::std` flatten into src/client.rs and src/server.rs, the runtime-named module waiting for a second runtime that never came.
- Features: `default = ["notify"]`, `notify` and `vendored`; `client`, `server`, `cli` and `schemars` go, their dependencies becoming plain ones.
- Cargo.toml follows cargo-009: no `documentation`, no docs.rs block, no `no-std` category, no `[[bin]]` or `[[test]]` blocks.
- examples/ goes. The tests that used the library move into the modules they test or drive the binary; tests/common serves a timer by running `comodoro server start`.
- Code only the library's consumers used goes.
- CI clippy runs the default set and `--no-default-features`; CONTRIBUTING, the spec (packaging.md) and the docs follow.
