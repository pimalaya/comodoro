---
cairn: change
id: the-wizard-leaves
status: landed
created: 2026-11-12
---

# The wizard leaves

## Why

The owner decided on 2026-11-08 that the Pimalaya CLIs drop their setup wizards, and Himalaya and Ortie removed theirs (himalaya cairn/log/2026-11-11-the-wizard-leaves.md). Comodoro's asks two questions (log 2026-08-14-first-run-wizard), but it owns for them a hand-written TOML renderer (`AccountConfig::render`, src/cli/config.rs:121), the save, append and print paths of src/cli/configure.rs, a `Serialize` derive on every configuration type, the `comodoro-configure` schema and pimalaya-cli's `prompt` feature: all for a flow the shared welcome replaces family-wide in the-shared-kit.

## What

- Delete src/cli/configure.rs and the `configure` command with its `wizard` alias (src/cli.rs:97).
- Drop `offer_configuration` and the offer inside `take_account` (src/cli.rs:244, :267). Bare `comodoro` prints the help. A command needing an account and finding no configuration fails naming the path it looked at and pointing at config.sample.toml, where it used to offer the wizard on a terminal; the welcome replaces that message in the-shared-kit.
- Drop `AccountConfig::render` and the `Serialize` derives on `Config`, `AccountConfig`, `SocketConfig`, `TcpConfig` and `TimerHook`, which only rendered a generated account. The wire types (`Timer`, `TimerCycle`, `TimerLoop`, `TimerPrecision`, `TimerEvent`) keep theirs.
- Drop `GeneratedConfig` and the `comodoro-configure` schema from src/cli/json_schema.rs.
- Drop the pimalaya-cli `prompt` feature, the wizard being its only user.
- The README Configuration section and the config.sample.toml header stop naming `configure`; the CHANGELOG gets one Removed entry, BREAKING.

## Scope / non-goals

It stays on pimalaya-cli 0.2 and pimalaya-config 0.1, so it compiles and lands alone. The welcome, `-c` taking one path per flag and the typed account resolution are the-shared-kit's. B4 and B9 of cli-alignment-leftovers lapse with the wizard.
