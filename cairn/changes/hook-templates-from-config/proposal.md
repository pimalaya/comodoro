---
cairn: change
id: hook-templates-from-config
status: active
created: 2026-11-12
---

# Hook templates from the configuration

## Why

GUIDELINES cli-008 gives hooks one shape across products: a hook runs a `command` and may show a `notify`, its variables are camelCase, expanded in a notification with the shell's syntax through pimalaya-config and handed to a command as environment variables of the same names, and a notification naming an unknown variable is refused when the configuration is read. comodoro's `TimerHook` is an enum (src/cli/hook.rs:63): a hook is a command or a notification, never both, and neither knows which cycle fired it, so a notification reads "Work ended" only if written once per cycle by hand. A hook name no event can fire, such as `on-wrok-end`, loads and stays silently dead. carillon carries the reference (carillon src/hook.rs and src/config.rs, cairn/spec/daemon.md "A hook templates against what its event carries").

## What

- `TimerHook` becomes a struct with `deny_unknown_fields`: `command: Option<CommandConfig>` and `notify: Option<NotificationTemplate>`. When both are set the command runs, then the notification shows. A hook setting neither is refused. A 2.x hook, `command` or `notify.{summary,body}`, parses unchanged.
- The cycle hooks (`on-<cycle>-begin`, `-running`, `-set`, `-pause`, `-resume`, `-end`) fill `cycleName` and `cycleDuration`, the remaining seconds, as the event's `cycle` carries them; `on-timer-start` and `on-timer-stop` fill none. A notification renders through `NotificationTemplate::render`, a command receives them as environment variables through `CommandConfig::to_command().envs(...)`.
- Checked when the configuration is read, as carillon's `account.validate()` does: a notification naming a variable its hook cannot fill is refused naming the hook and the variables it may use; a notification in a build without the `notify` feature is refused naming the feature; a hook name no event of the account's cycles can fire is refused listing the names they can fire (BREAKING: a mistyped name used to load).
- comodoro had no variables, so no former name has to keep reaching a command.

## Open question for the owner

cli-008 names an event with a bare verb. `running` is not one: `on-<cycle>-running` fires on every tick, once a second. The proposal is to rename it `on-<cycle>-tick` in the 3.0 break, the protocol's `timer.running` notification keeping its name. Keeping `running` leaves a named exception to cli-008.

## Scope / non-goals

Hooks still run in the server, on the account the server was started with. No new hook event.
