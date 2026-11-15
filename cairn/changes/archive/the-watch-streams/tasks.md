---
cairn: tasks
change: the-watch-streams
---

# Tasks

- [x] Pin pimalaya-cli 51ce8b1 with `signal`; Cargo.lock moves its source line and adds ctrlc, nix and cfg_aliases (dispatch2 gains two dependencies), no version moved.
- [x] Replace `error::coded` with `pimalaya_cli::error::with_code` and a `code` function; the server-code binary tests (tests/output.rs) unchanged and passing.
- [x] `watch`: `timer-changed` events, `Shutdown`, the `stoppedBy` result, `server-closed` on a closed connection; streaming printer in main.
- [x] Register `TimerWatchOutput` and `TimerWatchEvent` for `comodoro-watch`; schema test.
- [x] tests/watch.rs: SIGINT under JSON, SIGTERM as text through a pipe, a closing server.
- [x] README, CHANGELOG `[Unreleased]` **Breaking:** line, MIGRATION.md v2 to v3 lines and jq example.
- [x] Delta folded into cairn/spec/output.md; log written; status landed; archived.
