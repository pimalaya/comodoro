---
cairn: log
change: the-binary-alone
landed: 2026-11-21
---

# The binary alone

Comodoro ships a binary only, by the owner's ruling of 2026-11-21.

- src/lib.rs is gone; src/main.rs declares the modules and carries the architecture header. The crate is std: no `#![no_std]`, no `extern crate`, no `core` or `alloc` imports.
- `client::std` and `server::std` flattened into src/client.rs and src/server.rs.
- Features: `default = ["notify"]`, `notify`, `vendored`. `client`, `server`, `cli` and `schemars` are gone; schemars derives unconditionally. Cargo.toml follows GUIDELINES cargo-009: no `documentation`, no docs.rs block, no `no-std` category or keyword, no `[[bin]]`, `[[test]]` or `[[example]]` blocks; build.rs is no longer gated.
- examples/ is gone. tests/timer.rs moved to src/client/tests.rs, the library half of tests/config.rs to src/cli/account/tests.rs (the binary half stays in tests/config.rs), tests/hook.rs to src/cli/hook/tests.rs. tests/common and tests/output.rs serve the timer by spawning `comodoro server start`, killed on drop.
- Dead code the library alone used is gone: `TimerClient::unsubscribe` (the unsubscribe test sends the request through the client's private `send`), `Jsonrpc20Error::INTERNAL`, `SERVER_MIN`, `SERVER_MAX` and `internal`.
- CI clippy runs the default set, `--all-features` and `--no-default-features`, and tests the last. CONTRIBUTING.md rewritten for the binary; cairn/verify.sh vendored from ortie.
- CHANGELOG: the library lines of [Unreleased] folded into one breaking removal; MIGRATION.md: the v2 to v3 Library section says the library is gone.

Spec moved: packaging.md (Feature layers, Every feature set builds clean), protocol.md (no_std wording).
