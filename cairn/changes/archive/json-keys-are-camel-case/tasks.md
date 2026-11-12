---
cairn: tasks
change: json-keys-are-camel-case
---

# Tasks

Landed on 2026-10-27 under [Unreleased], marked breaking, ahead of the 3.0 version bump.

- [x] `#[serde(rename_all = "camelCase")]` on `Timer` in src/timer.rs, renaming `started_at` to `startedAt`
- [x] Same attribute on `ConfigureOutput` in src/cli/configure.rs, whose payload does not move (the type is still named `GeneratedConfig`, its rename being cli-alignment-leftovers B9)
- [x] Update the timer object example and the `started_at` prose in cairn/spec/protocol.md, the wire contract moving with the payload
- [x] Leave src/cli/config.rs kebab-case, and leave `TimerPrecision` alone: its rename governs values read from the configuration, not keys
- [x] Regenerate the JSON Schemas and check no key kept an underscore
- [x] CHANGELOG under `Changed`, as breaking for both `--json` and the JSON-RPC protocol, in the 3.0 section (no such section yet: under [Unreleased], marked **Breaking:**)
