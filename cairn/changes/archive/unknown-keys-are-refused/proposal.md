---
cairn: change
id: unknown-keys-are-refused
status: landed
created: 2026-11-14
---

# Unknown keys are refused

## Why

Owner decision, 2026-11-14, family-wide: every configuration struct carries `#[serde(deny_unknown_fields)]`, and a key a CLI used to read and removed is refused by name with a message pointing at MIGRATION.md rather than serde's generic error, as Himalaya refuses its removed `backend` and `m2dir` tables and Calendula its `table`, `account`, `todo` and `journal` ones.

Comodoro already denied unknown keys on `Config`, `AccountConfig`, `SocketConfig`, `TcpConfig` and `TimerHook`, and pimalaya-config's notification table denies its own (src/cli/config.rs, src/cli/hook.rs; no configuration type uses `#[serde(flatten)]`). One table slipped: a cycle is read as `TimerCycle`, the wire type, which accepts any key, so `{ name = "Work", duraton = 1500 }` fails only on the missing `duration` and `{ name = "Work", duration = 1500, color = "red" }` loads.

The keys Comodoro read and removed, from the CHANGELOG, MIGRATION.md and the v1.0.0 and v2.0.0 sources: the 2.x per-second hook name `on-<cycle>-running` (renamed `on-<cycle>-tick` in this cycle), refused today as a name no event fires; and the 1.x hook key `cmd`, an alias of `command` that 2.0 dropped without a word, refused as an unknown key. The 2.x wizard wrote only keys v3 still reads. `unix-socket` stays an accepted alias.

## What

- src/cli/config.rs: a cycle is read as `CycleConfig` (`name`, `duration`, kebab-case, `deny_unknown_fields`), turned into a `TimerCycle` when the account resolves (src/cli/account.rs). The wire's `TimerCycle` keeps its leniency: refusing a key there is a protocol decision, not a configuration one.
- src/cli/hook.rs: `TimerHook::validate` refuses `on-<cycle>-running`, `<cycle>` one of the account's, naming `on-<cycle>-tick` and MIGRATION.md; a `cmd` field typed by an uninhabited `CmdRenamedConfig` refuses the key as it loads, naming `command` and MIGRATION.md (Calendula's pattern).
- tests/config.rs, through the binary: an unknown key at the root, in an account, in `tcp`, `socket`, a hook, a cycle and (with `notify`) a notification, each refused naming it; `on-work-running` and `cmd` refused naming their replacement and MIGRATION.md.
- cairn/spec/config.md and hooks.md, CHANGELOG (**Breaking:**), MIGRATION.md.

## Scope / non-goals

- The pre-1.0 keys (the root `presets` table, `preset`, `timer-precision`) stay refused as unknown keys: 1.0 did not name them either.
- No change to the wire's leniency; no version bump; no release.
