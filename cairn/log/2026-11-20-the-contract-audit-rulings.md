---
cairn: log
change: the-contract-audit-rulings
landed: 2026-11-20
---

# The contract audit rulings

The owner's rulings on the CLI contract audit of 2026-11-20 (comodoro findings COM-X1 to COM-X9, family ruling A5) landed as an uncommitted diff, Cargo.toml pointing at the unreleased pimalaya-cli working tree through `[patch.crates-io]` until the kit's release.

- `account list` and `account check` (src/cli/account.rs, src/cli/account/), on the kit's `AccountListOutput` and `AccountCheckOutput` (a list of accounts since the kit's the-contract-audit-rulings). A transport is declared by its table or by being the default one, so `AccountConfig::socket` and `tcp` became `Option`s and `Account` gained `name`, `transports` and `Account::new`. Not MCP tools: himalaya, ortie, cardamum and calendula list none but `account capability`.
- `server start` stops on SIGINT, SIGTERM or SIGHUP through `signal::Shutdown`, removes its socket files and prints `{"stoppedBy":"signal"}`, exit 0, where it died with 130 or 143 and left comodoro.sock behind.
- The by-name refusals of `presets`, `preset`, `timer-precision` and `on-<cycle>-running`, with `Config::presets`, `AccountConfig::preset`, `AccountConfig::timer_precision` and their three refusing types, are gone: serde and the hook-name check refuse them as anything unknown.
- Codes: a REPL's refusal of `watch` and `server start` is `usage-invalid`; `TimerTransportError` gives `address-busy` (a live server on the socket, `EADDRINUSE` on either transport) and `socket-path-invalid` (the platform's `InvalidInput` on a Unix socket path, which a client used to report as `server-unreachable`); the transport, client and watch contexts read "Cannot ...".
- The wire's `startedAt` is RFC 3339 in UTC, `YYYY-MM-DDTHH:MM:SSZ`, written and read by src/timer/rfc3339.rs (no_std, Hinnant's civil-day arithmetic, a read checked by writing it back); `TimerOutput` uses the same writer, so jiff left the dependencies.
- A bare `comodoro` goes through pimalaya-cli's `clap::meet_bare_invocation` (src/main.rs), `cli::meet_bare_invocation` deleted: under `--json` it is the `usage-invalid` failure object, ``A command is required, see --help``, exit 2, the configuration left unread, where it printed the help's text (exit 0) or the missing configuration (exit 1); the text run is unchanged.
- CHANGELOG: breaking markers on the template `$$`, the several-defaults refusal and the `--dir` move; one pimalaya-cli line; the by-name refusal line removed; new lines for the account group, `server start`, the wire instant and the library changes. MIGRATION.md: "From v0.1 to v1" removed (owner rule: no 0.x guide), a v2 to v3 entry per breaking change.

## What reaches users

`comodoro account list` and `comodoro account check`; `server start` exiting 0 with a result line on a signal, its socket removed; the codes `address-busy` and `socket-path-invalid`; a bare `comodoro --json` refused coded `usage-invalid`, exit 2; a v2 config holding a 0.x key or `on-<cycle>-running` refused with the generic message; JSON-RPC clients of their own reading `startedAt` as an RFC 3339 string.

## Capabilities moved

- commands (new): *An account group lists and checks the accounts*.
- output: *`server start` stops on a signal* added; the code table, the exit-3 sentence, the registered keys and *The timer prints as one line* modified.
- protocol: *The timer is one object, and it says what it runs* and *Transport is a Unix domain socket or TCP* modified.
- config: *Unknown keys are refused at every level* and *A bare invocation shows the help* modified.
- hooks: *A removed hook spelling is refused by name* removed, its case a scenario of *A hook is named after the event that fires it*.
- repl: *A session runs the CLI over one account* modified. mcp: *Writing is opt-in* modified.

## Verified

`cargo fmt`, `cargo clippy --all-targets -- -D warnings` over the default, `--all-features` and `--no-default-features` feature sets, the offline `cargo test` suite; every new regression test (tests/account.rs, tests/server.rs, the rewritten tests/config.rs refusals, the coded REPL refusal, the `Timer` RFC 3339 unit test, tests/first_run.rs `a_bare_invocation_under_json_is_a_refused_command_line`) failed against HEAD 9a9bd1e in a scratch snapshot.
