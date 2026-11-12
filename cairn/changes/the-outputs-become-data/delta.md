---
cairn: delta
change: the-outputs-become-data
---

# Delta

## ADDED Requirements

### Requirement: A data command prints a named output type
(output) Each command printing a value SHALL print one type named after what it prints and suffixed `Output`, deriving `Display`, `Serialize` and `JsonSchema`, so the human rendering, the machine rendering and the schema are three faces of one declaration. A sentence SHALL be the `Display` of such a type, never a value of its own.

### Requirement: The timer prints as one line
(output) `TimerOutput` SHALL carry the timer's `state`, `cycle`, `startedAt`, `elapsed` and `schedule`, as the wire's timer does but for `startedAt`. Its text SHALL be one line: `OFF` when stopped, `[<cycle>] paused` when paused, and `[<cycle>]` followed by the remaining duration at the account precision when running, ending with a newline.

#### Scenario: A status bar reads the timer
- GIVEN a timer running its `Work` cycle with 25 minutes left and the account precision `minute`
- WHEN `comodoro get` runs
- THEN it prints `[Work] 25min` and a newline

### Requirement: A timer control answers the timer
(output) `start`, `pause`, `resume`, `stop` and `set` SHALL print the `TimerOutput` of the timer after the change, read on the connection that made it, and SHALL print it as well when the timer was already in the state asked for.

#### Scenario: A script starts the timer
- GIVEN a stopped timer
- WHEN `comodoro start --json` runs
- THEN it prints one object whose `state` is `running` and whose `startedAt` is the instant it started

### Requirement: `watch` prints a result per change
(output) `watch` SHALL print one `TimerOutput` on start and one per visible change of the timer, until interrupted, several events of one tick printing one result. It is the one command printing more than one result.

### Requirement: Shared shapes and spellings
(output) `startedAt` SHALL print as an RFC 3339 instant in UTC with `Z`, `null` unless the timer runs, and no output type SHALL print an epoch number. Values SHALL be kebab-case (cairn/spec/protocol.md, "Values are kebab-case").

### Requirement: Text output ends with a newline
(output) Every command's text rendering SHALL end with a newline, so the shell prompt starts on a line of its own and a line-buffered pipe receives each `watch` line as it happens.

### Requirement: The JSON contract is described and versioned
(output) `json-schema` SHALL describe the payload of every command printing data, and the failure object every command prints instead under the key `comodoro-error`. Every generated schema SHALL carry an `$id` naming the contract version, the major version of Comodoro, reading `https://pimalaya.org/schemas/comodoro/v<major>/<key>.json`. Every registered payload SHALL be a JSON object, checked by a test. The keys are `comodoro-get`, `comodoro-watch`, `comodoro-start`, `comodoro-pause`, `comodoro-resume`, `comodoro-stop`, `comodoro-set` and `comodoro-error`.

#### Scenario: `--json` output is piped
Each value is printed compact on its own line and flushed, so `watch --json` is newline-delimited JSON a reader receives as each change happens. On a terminal the value is pretty-printed instead.

## MODIFIED Requirements

None.

## REMOVED Requirements

### Requirement: The JSON output is described and versioned
(json-output) The capability is renamed output, as Ortie's is, and this requirement moves there, the timer controls now carrying a schema. spec/json-output.md is deleted.
