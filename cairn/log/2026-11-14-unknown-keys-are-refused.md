---
cairn: log
change: unknown-keys-are-refused
landed: 2026-11-14
---

# Unknown keys are refused

Every configuration table refuses a key it does not read, after the family-wide owner decision of 2026-11-14. `Config`, `AccountConfig`, `SocketConfig`, `TcpConfig` and `TimerHook` already carried `deny_unknown_fields`, and pimalaya-config's notification table refuses its own; no configuration type flattens another. The cycle did not: it was read as the wire's `TimerCycle`. It is now read as `CycleConfig` (src/cli/config.rs), refusing any key but `name` and `duration`, and turned into a `TimerCycle` as the account resolves; the wire type keeps its leniency. `AccountConfig::cycles` is a `Vec<CycleConfig>` in the library.

The removed spellings are refused by name, pointing at MIGRATION.md: the 2.x hook name `on-<cycle>-running`, by `TimerHook::validate`, naming `on-<cycle>-tick`; and the 1.x hook key `cmd`, by an uninhabited `CmdRenamedConfig` field as Calendula refuses its removed tables, naming `command`. The 2.x wizard wrote no key v3 drops, and `unix-socket` stays an alias.

Tests: tests/config.rs, through the binary. `an_unknown_key_is_refused_at_every_level` (root, account, `tcp`, `socket`, hook, cycle, and the notification with `notify`) failed on the cycle case before the change, the five cases ahead of it already passing; `the_running_hook_of_v2_is_refused_by_name` and `the_cmd_hook_key_of_v1_is_refused_by_name` failed. tests/hook.rs's no-event test moved from `on-work-running` to `on-work-done`. The shipped sample loads in every variant.

Capabilities moved: config ("Unknown keys are refused at every level" added), hooks ("A removed hook spelling is refused by name" added). CHANGELOG (**Breaking:**) and MIGRATION.md ("Configuration and first run", "Hooks") follow.

Left: serde lists `cmd` among the keys a hook takes when refusing another unknown one, as Himalaya and Calendula list their refused tables. The pre-1.0 keys (`presets`, `preset`, `timer-precision`) stay refused as unknown keys.
