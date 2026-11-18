---
cairn: spec
capability: hooks
---

# Hooks

The reactions a server runs when its timer emits an event, bound to the event by name in the account's `hooks` table (GUIDELINES cli-008).

## Requirement: A hook is named after the event that fires it

An account's `hooks` table SHALL key each hook by the event firing it: `on-timer-start` and `on-timer-stop` for the timer, and `on-<cycle>-<event>` for a cycle, `<cycle>` the cycle name in kebab-case and `<event>` one of `begin`, `tick`, `set`, `pause`, `resume` and `end`, `tick` firing every second the cycle runs. A name no event of the account's cycles can fire SHALL be refused when the account is resolved, naming the cycles and the events.

### Scenario: A mistyped hook name

An account whose cycles are `Work` and `Rest` and whose hooks hold `on-wrok-end` fails any command naming `on-wrok-end`, the cycles `work, rest`, the six events and the two timer hooks.

## Requirement: A hook runs a command, shows a notification, or both

A hook SHALL hold a `command`, read through pimalaya-config's `command` module and also read under `cmd`, its alias wherever the family configures a command (pimalaya-config cairn/spec/command.md), a `notify` with a `summary` and an optional `body`, or both, and SHALL be refused holding neither. With both, the command SHALL run first, then the notification show. A hook failing SHALL be logged and SHALL NOT stop the timer.

### Scenario: A 1.x hook command

An account whose hooks hold `on-work-end.cmd = "true"` loads, and runs `true` when the Work cycle ends, as `on-work-end.command = "true"` does.

## Requirement: A hook templates against what its event carries

A cycle hook SHALL fill `cycleName` and `cycleDuration`, the cycle's name and its remaining seconds as the event carries them, and the timer hooks none. A notification SHALL be templated in the shell's syntax (`$name`, `${name}`, `${name:-default}`, `$$`) through pimalaya-config, and one naming a variable its hook does not fill, or using any other `${...}` form, SHALL be refused when the account is resolved, naming the hook and the variables it fills. A value filled into a notification SHALL be rendered by pimalaya-config's `NotificationTemplate::render`: where the notification daemon reads markup (XDG platforms) its `&`, `<`, `>`, `"` and `'` are escaped, a line break or a tab becomes a space, and any other control or bidirectional override character is dropped, the template's own text kept as written, so a cycle name a server sends can neither style nor reshape the notification. A command SHALL receive the same variables as environment variables of the same names, and SHALL NOT be validated.

### Scenario: One notification for every cycle

An account whose `on-work-end` and `on-rest-end` hooks both notify `$cycleName ended` shows "Work ended" when the Work cycle ends.

### Scenario: A cycle variable on a timer hook

An account whose `on-timer-start` notification reads `$cycleName` is refused, naming `on-timer-start` and saying it fills no variable.

## Requirement: Notifications are shown when the build can show them

A `notify` cargo feature, on by default, SHALL decide whether hooks can show notifications. A build without it SHALL refuse a configuration carrying a `notify` table, naming the feature.

## Requirement: An account runs at least one cycle

An account whose `cycles` is empty SHALL be refused when it is resolved, rather than failing once a server builds its timer.

## Requirement: A removed hook spelling is refused by name

A hook name `on-<cycle>-running`, `<cycle>` one of the account's, SHALL be refused when the account is resolved, naming `on-<cycle>-tick` and MIGRATION.md.

### Scenario: A v2 tick hook

An account whose cycle is `Work` and whose hooks hold `on-work-running` fails any command with "Hook `on-work-running` is now `on-work-tick`, see MIGRATION.md".
