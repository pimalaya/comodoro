---
cairn: log
change: hook-templates-from-config
landed: 2026-11-12
---

# Hook templates from the configuration

`TimerHook` (src/cli/hook.rs) is a struct of an optional `CommandConfig` and an optional `NotificationTemplate`, `deny_unknown_fields`; `execute` runs the command with the event's variables as environment variables, then renders and shows the notification. `TimerEvent::hook_variables` fills `cycleName` and `cycleDuration` for the cycle events and nothing for the two timer events. src/cli/server/start.rs passes them.

`Account` is built with `TryFrom<AccountConfig>`, which refuses an account with no cycle and runs `TimerHook::validate` on every hook: the name must be one an event of the account's cycles fires, the hook must hold a command or a notification, and the notification may name only the variables its event fills. The checks run as the account is resolved, before any command, rather than when the whole file is read: an account no command uses is not resolved.

The per-second hook was renamed `on-<cycle>-tick` (BREAKING): cli-008 names events with a bare verb, and `running` was not one. The owner had not answered the question the proposal raised by the time this landed, so this follows the proposal's recommendation; the protocol's `timer.running` notification keeps its name.

An account with `cycles = []` used to load and panic in `Timer::new` when the server started; the-sample-is-tested-and-shipped surfaced it, and it is refused here.

Tests: tests/config.rs drives the binary on a mistyped hook name, an empty `cycles` and `$cycleName` on `on-timer-start`, all three failing on the old code first (they reached the transport and failed `server-unreachable`); tests/hook.rs covers the variables reaching a command's environment, a command and a notification on one hook, a hook doing nothing, `${cycleName:-none}` accepted and `${cycleName:none}` refused, and the name refusal listing the cycles. The sample's hook line reads `$cycleName ended`.

Capabilities: hooks created (A hook is named after the event that fires it, A hook runs a command, shows a notification, or both, A hook templates against what its event carries, Notifications are shown when the build can show them, An account runs at least one cycle).
