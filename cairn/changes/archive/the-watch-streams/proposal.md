---
cairn: change
id: the-watch-streams
status: landed
created: 2026-11-15
---

# The watch streams

## Why

pimalaya-cli master 51ce8b1 (pushed 2026-11-15) adds the family requirement "A long-lived command streams events" (cli cairn/spec/output.md): a command running until stopped prints one event line per event, then exactly one line without `event`, its result or the failure object; a termination signal stops it cleanly with its result, a watch exiting 0, through `signal::Shutdown`; its text events reach a pipe through `StdoutPrinter::streaming`. Family item 10 applies it to Comodoro's `watch`, which printed a `TimerOutput` result per change (the one command printing several results), died on a signal with nothing printed, and exited 0 when the server went away.

The same pin brings `pimalaya_cli::error::with_code` (family item 13), the chain rebuild Comodoro carried as `error::coded`.

## What

- Cargo.toml: pimalaya-cli pinned at 51ce8b17714ee7445bd59874d0cfd988612d9a08, with its `signal` feature (ctrlc joins Cargo.lock).
- src/cli/error.rs: `coded` gone; `code(&Error) -> Option<ErrorCode>` reads the code off a `TimerClientError` (and an `AccountError`, default-is-a-name), and src/cli.rs calls `with_code(err, error::code)`.
- src/cli/client/watch.rs: `TimerWatchEvent::TimerChanged` (`{"event":"timer-changed", ...TimerOutput}`, the timer's line as text) through `printer.event`; `Shutdown::install` before connecting; the client reads on a thread, the command polls the shutdown every 100 ms and prints `TimerWatchOutput` (`{"stoppedBy":"signal"}`, `Watch stopped` as text); the connection closing fails with `TimerClientError::Closed`, coded `server-closed`.
- src/main.rs: the printer streams for `watch`.
- src/cli/json_schema.rs: `comodoro-watch` describes `TimerWatchOutput`, `comodoro-watch.events` `TimerWatchEvent`.
- tests/watch.rs: JSON events then the result on SIGINT, exit 0; text events reaching a pipe then `Watch stopped` on SIGTERM, exit 0; a server closing the connection, the event then the `server-closed` failure, exit 1. A schema test.
- cairn/spec/output.md, README, CHANGELOG (**Breaking:**), MIGRATION.md.

## Scope / non-goals

- `server start` runs until interrupted too but prints nothing; it keeps its own handling.
- The REPL still refuses `watch`: pimalaya-cli's REPL installs no shutdown.
- The `timer-changed` variant flattens the timer as a struct variant (`TimerChanged { timer }`): as a newtype variant schemars writes `$ref` beside the `event` property, and pimalaya-cli's registry check follows the `$ref` alone and reports "event without an event key". The JSON is the same either way.
