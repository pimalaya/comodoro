---
cairn: tasks
change: the-contract-audit-rulings
---

# Tasks

- [x] Point Cargo.toml at the unreleased pimalaya-cli through `[patch.crates-io]` (a release pin before commit)
- [x] COM-X1: src/cli/account.rs `AccountCommand`, src/cli/account/list.rs, src/cli/account/check.rs; `AccountConfig::transports`, `default_transport`; `Account::new`; registry keys; tests/account.rs, seen failing first
- [x] COM-X2: src/cli/server/start.rs `Shutdown`, socket removal, `TimerServerStartOutput`; registry key, `server start` out of `SILENT`; tests/server.rs signal tests, seen failing first
- [x] A5 / COM-X3: refusal fields and types out of src/cli/config.rs, `on-<cycle>-running` out of src/cli/hook.rs; tests/config.rs rewritten to assert the generic refusal, seen failing first
- [x] COM-X5: src/cli/repl.rs refusal coded `usage-invalid`; tests/repl.rs asserts it, seen failing first
- [x] COM-X6: src/transport.rs `TimerTransportError`, src/cli/error.rs codes, src/client/std.rs keeps the transport error; "Cannot ..." contexts; tests/server.rs busy and too-long tests, seen failing first; tests/timer.rs asserts the typed error
- [x] COM-X9: src/timer/rfc3339.rs, `Timer::started_at` through it, `TimerOutput` through it, jiff dropped; unit tests, the `Timer` one seen failing first
- [x] Kit item B: main calls `pimalaya_cli::clap::meet_bare_invocation`, `cli::meet_bare_invocation` deleted; tests/first_run.rs `a_bare_invocation_under_json_is_a_refused_command_line`, seen failing first
- [x] COM-X7, COM-X8: CHANGELOG markers and lines, MIGRATION.md entries, "From v0.1 to v1" removed
- [x] README: account group, server stop, codes
- [x] Fold the delta into spec/commands.md (new), output.md, protocol.md, config.md, hooks.md, repl.md, mcp.md; log
- [x] `cargo fmt`, clippy, the offline test suite, the `json-schema` diff
