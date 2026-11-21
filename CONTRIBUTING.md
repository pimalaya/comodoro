# Contributing guide

Thank you for investing your time in contributing to Comodoro.

Whether you are a human or an AI agent, read these in order before touching the code:

1. the [Pimalaya README](https://github.com/pimalaya) for what the project is and how its repositories stack;
2. the [Pimalaya CONTRIBUTING](https://github.com/pimalaya/.github/blob/master/CONTRIBUTING.md) guide, which chains to the shared architecture and guidelines;
3. the inline header documentation, starting with src/main.rs: it is the architecture document of this crate;
4. the cairn/ folder, which follows [Cairn](https://github.com/pimalaya/cairn): `spec/` is current truth, `changes/` holds in-flight proposals (landed ones move to `changes/archive/`), and `log/` is the dated history; `sh cairn/verify.sh` checks the structure.

Everything below documents only what differs from the Pimalaya standards.

## Where changes belong

Comodoro is self-contained. Unlike the rest of the Pimalaya binaries it drives no io- library: the timer state machine, the wire protocol and the blocking client and server all live in this repository. A protocol or timer fix therefore lands here, not upstream.

Only two shared crates are consumed, both from crates.io. The clap arguments, printer, logger, error reporting, first-run welcome, REPL and MCP server come from [pimalaya/cli](https://github.com/pimalaya/cli), the TOML loader, the account resolution, the sample variants and the hook command and notification templates from [pimalaya/config](https://github.com/pimalaya/config), which brings desktop notifications through notify-rust. To build against a local checkout of either, add a `[patch.crates-io]` table with a `<crate>.path = "../<repo>"` entry, and drop it before committing.

## Feature matrix

Comodoro ships a binary and no library. The only features are `notify` (default), which pulls the notification backend the hooks use and the one system dependency (D-Bus, through notify-rust), and `vendored`, which release builds for platforms without a system D-Bus use. Build without notifications when touching them:

```sh
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --no-default-features
```

CI runs clippy over every feature set (.github/workflows/tests.yml).

## Layers and where code goes

New code belongs in the lowest layer that can hold it. A change to what travels on the wire belongs in src/protocol.rs, is documented in cairn/spec/protocol.md, and only then grows a client method and a CLI command. Nothing in src/timer.rs, src/jsonrpc20.rs or src/protocol.rs may perform I/O or read the clock: the timer takes the current time as a parameter, which is what keeps its behaviour testable without waiting for time to pass.

The protocol is not private. Treat a change to a method name, a parameter or a result shape as breaking for third-party clients, even when every caller in this repository is updated in the same commit.
