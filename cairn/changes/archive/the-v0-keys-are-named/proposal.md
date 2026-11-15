---
cairn: change
id: the-v0-keys-are-named
status: landed
created: 2026-11-15
---

# The 0.x keys are named

## Why

cairn/spec/config.md, "Unknown keys are refused at every level", has a key Comodoro used to read and removed refused naming what replaced it and MIGRATION.md. unknown-keys-are-refused did so for the 2.x `on-<cycle>-running` and the 1.x `cmd`, and left the 0.x keys refused as any unknown key, serde's generic error naming neither the replacement nor the guide. Owner request, 2026-11-15: refuse them by name too.

The 0.x keys, from the v0.1.2 sources (src/config.rs, src/preset/config.rs) and config.sample.toml: the root table `presets`, which 1.0 renamed `accounts`; an account's `preset` (`pomodoro` or `52/17`), which 1.0 removed in favour of `cycles`; and an account's `timer-precision`, which 1.0 renamed `precision`. Their other keys, `cycles`, `cycles-count`, `tcp.host`, `tcp.port` and `hooks`, still load, and `cmd` is refused already.

## What

- src/cli/config.rs: `Config::presets`, `AccountConfig::preset` and `AccountConfig::timer_precision`, each typed by an uninhabited enum whose deserializer refuses the key naming its replacement and MIGRATION.md, as `CmdRenamedConfig` does (src/cli/hook.rs). src/cli/account.rs ignores the two account fields, never set.
- tests/config.rs, through the binary: each key refused naming its replacement and MIGRATION.md.
- cairn/spec/config.md, CHANGELOG (**Breaking:**), MIGRATION.md (a v0.1 to v1 section, which the messages point at).

## Scope / non-goals

- The 1.0 CHANGELOG line "Prefixed preset configs with `presets.`" says the opposite of what 1.0 did (`presets` became `accounts`); a released entry is left as it is.
- No version bump; no release.
