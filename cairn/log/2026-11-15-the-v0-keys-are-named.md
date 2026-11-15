---
cairn: log
change: the-v0-keys-are-named
landed: 2026-11-15
---

# The 0.x keys are named

The 0.x configuration keys are refused by name, pointing at MIGRATION.md, as the 1.x `cmd` and the 2.x `on-<cycle>-running` are: the root table `presets` naming `accounts`, an account's `preset` naming the presets of config.sample.toml, and its `timer-precision` naming `precision`. Each is a field of `Config` or `AccountConfig` (src/cli/config.rs) typed by an uninhabited enum refusing it as the configuration loads, after `CmdRenamedConfig`; they were refused before as any unknown key. MIGRATION.md gains the "From v0.1 to v1" section they point at.

Tests: tests/config.rs `the_keys_of_v0_are_refused_by_name`, through the binary, failed before the change.

Capabilities moved: config ("Unknown keys are refused at every level" modified). CHANGELOG (**Breaking:**) and MIGRATION.md follow.

Left: the 1.0 CHANGELOG line "Prefixed preset configs with `presets.`" misstates what 1.0 did, renaming `presets` to `accounts`; the released entry is untouched. serde lists `presets`, `preset` and `timer-precision` among the expected keys when refusing another unknown one, as it lists `cmd`.
