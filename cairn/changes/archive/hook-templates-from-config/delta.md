---
cairn: delta
change: hook-templates-from-config
---

# Delta

## ADDED Requirements

### Requirement: A hook is named after the event that fires it
(hooks) An account's `hooks` table SHALL key each hook by the event firing it: `on-timer-start` and `on-timer-stop` for the timer, and `on-<cycle>-<event>` for a cycle, `<cycle>` the cycle name in kebab-case and `<event>` one of `begin`, `running`, `set`, `pause`, `resume` and `end`. A name no event of the account's cycles can fire SHALL be refused when the configuration is read, listing the names they can fire.

#### Scenario: A mistyped hook name
- GIVEN an account whose cycles are `Work` and `Rest` and whose hooks hold `on-wrok-end`
- WHEN any command reads the configuration
- THEN it fails naming `on-wrok-end` and listing `on-timer-start`, `on-timer-stop`, `on-work-begin` and the other names the two cycles fire

### Requirement: A hook runs a command, shows a notification, or both
(hooks) A hook SHALL hold a `command`, read through pimalaya-config's `command` module, a `notify` with a `summary` and a `body`, or both, and SHALL be refused holding neither. With both, the command SHALL run first, then the notification show. A hook failing SHALL be logged and SHALL NOT stop the timer.

### Requirement: A hook templates against what its event carries
(hooks) A cycle hook SHALL fill `cycleName` and `cycleDuration`, the cycle's name and its remaining seconds as the event carries them, and the timer hooks none. A notification SHALL be templated in the shell's syntax (`$name`, `${name}`, `${name:-default}`, `$$`) through pimalaya-config, and one naming a variable its hook does not fill, or using any other `${...}` form, SHALL be refused when the configuration is read, naming the hook and the variables it may use. A command SHALL receive the same variables as environment variables of the same names, and SHALL NOT be validated.

#### Scenario: One notification for every cycle
- GIVEN an account whose `on-work-end` and `on-rest-end` hooks both notify `$cycleName ended`
- WHEN the Work cycle ends
- THEN the notification reads "Work ended"

#### Scenario: A cycle variable on a timer hook
- GIVEN an account whose `on-timer-start` notification reads `$cycleName`
- WHEN the configuration is read
- THEN it is refused, naming `on-timer-start` and saying it fills no variable

### Requirement: Notifications are shown when the build can show them
(hooks) A `notify` cargo feature, on by default, SHALL decide whether hooks can show notifications. A build without it SHALL refuse a configuration carrying a `notify` table, naming the feature.

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
