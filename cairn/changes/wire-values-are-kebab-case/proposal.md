---
cairn: change
id: wire-values-are-kebab-case
status: active
created: 2026-11-12
---

# The wire values are kebab-case

## Why

json-keys-are-camel-case (2026-10-27) moved the keys and left the values on serde's defaults: the timer travels as `"state": "Running"` and `"loops": "Infinite"` or `{"Fixed": 4}`, the PascalCase variant names and an externally tagged object. The family spells a value in kebab-case and an unbounded count as a number or `null` (ortie and himalaya output spec, "Shared shapes and spellings"), and the CLI's `--json` prints the same timer from the-outputs-become-data on, so the two would disagree or the CLI would translate. The owner decided on 2026-11-12 that the wire moves in the same 3.0 break: third-party clients read `"running"` and `4` with no lookup table, and jq reads `.schedule.loops` without a type test.

## What

- `TimerState` takes `#[serde(rename_all = "kebab-case")]` (src/timer.rs:62): `running`, `paused`, `stopped`.
- `TimerLoop` travels as a number or `null` through `#[serde(from = "Option<usize>", into = "Option<usize>")]` (src/timer.rs:20), `null` meaning unbounded, with a matching schemars attribute. Still no_std.
- `TimerEvent` is already tagged `event` in camelCase, and `TimerPrecision` already kebab-case: both unchanged.
- The wire keeps `startedAt` in epoch seconds: a peer computes the remaining time from it, and the RFC 3339 instant is the CLI's output (the-outputs-become-data).
- A 3.0 client meeting a 2.x server still running, or a TCP server not yet upgraded, cannot read its answer. It fails with "Cannot read the timer server's answer, it may run another Comodoro version: restart it with this one" instead of "Deserialize timer response error" (src/client/std.rs:163); json-error-code gives it the code `server-mismatch`. MIGRATION.md (release-polish) tells users to restart the server, and any status bar running a 2.x `watch`, after upgrading.

## Scope / non-goals

The keys, the method names and the notification names do not move. No compatibility reader for 2.x values: a server and its clients upgrade together, and the error says so.
