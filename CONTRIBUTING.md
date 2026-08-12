# Contributing guide

Thank you for investing your time in contributing to Comodoro.

Whether you are a human or an AI agent, read these in order before touching the code:

1. the [Pimalaya README](https://github.com/pimalaya) for what the project is and how its repositories stack;
2. the [Pimalaya CONTRIBUTING](https://github.com/pimalaya/.github/blob/master/CONTRIBUTING.md) guide (Nix environment, build and check commands, dependency overrides, commit style), which chains to the shared architecture and guidelines;
3. the inline header documentation in src/lib.rs: it is the architecture document of this crate, covering the three layers, the two I/O axes and where each module sits;
4. the cairn/ folder for the development history and living plans (the Cairn convention: spec/, changes/, log/).

Everything below documents only what differs from the Pimalaya standards.

## Where changes belong

Comodoro is self-contained. Unlike the rest of the Pimalaya binaries it drives no io- library: the timer state machine, the wire protocol and the blocking client and server all live in this repository. A protocol or timer fix therefore lands here, not upstream.

Only three shared crates are consumed, all from crates.io. The clap arguments, printer, logger and error reporting come from [pimalaya/cli](https://github.com/pimalaya/cli), the TOML loader and the command serde adapter from [pimalaya/config](https://github.com/pimalaya/config), and desktop notifications from [notify-rust](https://crates.io/crates/notify-rust). The `[patch.crates-io]` table is empty. To build against a local checkout of a Pimalaya crate, add a `<crate>.path = "../<repo>"` entry there.

## Feature matrix

The features stack rather than branch: `cli` implies `client` and `server`, and `notify` implies `cli`. The contract layer (src/timer.rs, src/jsonrpc20.rs, src/protocol.rs) is always compiled and is the only no_std layer, so a default-features-off build has to keep compiling under `#![no_std]`. Build the reduced sets when touching the gates:

```sh
cargo build --no-default-features
cargo build --no-default-features --features client
cargo build --no-default-features --features cli
```

The `notify` feature is the only one pulling a system dependency (D-Bus, through notify-rust). Release builds for platforms without a system D-Bus use `vendored`, which forwards to notify-rust.

## Layers and where code goes

New code belongs in the lowest layer that can hold it, and the lowest layer is a contract other people implement. A change to what travels on the wire belongs in src/protocol.rs, is documented in cairn/spec/protocol.md, and only then grows a client method and a CLI command. Nothing in src/timer.rs, src/jsonrpc20.rs or src/protocol.rs may perform I/O or read the clock: the timer takes the current time as a parameter, which is what keeps its behaviour testable without waiting for time to pass.

The protocol is not private. Treat a change to a method name, a parameter or a result shape as breaking for third-party clients, even when every caller in this repository is updated in the same commit.
