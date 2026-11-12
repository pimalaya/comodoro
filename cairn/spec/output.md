---
cairn: spec
capability: output
---

# Output

What the CLI prints, as text for a person or a status bar and under `--json` for a script or an MCP client. The JSON-RPC wire a client and a server speak is cairn/spec/protocol.md.

## Requirement: A data command prints a named output type

Each command printing a value SHALL print one type named after what it prints and suffixed `Output`, deriving `Display`, `Serialize` and `JsonSchema`, so the human rendering, the machine rendering and the schema are three faces of one declaration. A sentence SHALL be the `Display` of such a type, never a value of its own.

## Requirement: The timer prints as one line

`TimerOutput` SHALL carry the timer's `schedule`, `state`, `cycle`, `startedAt` and `elapsed`, as the wire's timer does but for `startedAt`. Its text SHALL be one line: `OFF` when stopped, `[<cycle>] paused` when paused, and `[<cycle>]` followed by the remaining duration at the account precision when running, ending with a newline.

### Scenario: A status bar reads the timer

A timer running its `Work` cycle with 25 minutes left, on an account whose precision is `minute`, makes `comodoro get` print `[Work] 25min` and a newline.

## Requirement: A timer control answers the timer

`start`, `pause`, `resume`, `stop` and `set` SHALL print the `TimerOutput` of the timer after the change, read on the connection that made it, and SHALL print it as well when the timer was already in the state asked for.

### Scenario: A script starts the timer

On a stopped timer, `comodoro start --json` prints one object whose `state` is `running` and whose `startedAt` is the instant it started.

## Requirement: `watch` prints a result per change

`watch` SHALL print one `TimerOutput` on start and one per visible change of the timer, until interrupted, several events of one tick printing one result. It is the one command printing more than one result.

## Requirement: Shared shapes and spellings

`startedAt` SHALL print as an RFC 3339 instant in UTC with `Z`, `null` unless the timer runs, and no output type SHALL print an epoch number. Values SHALL be kebab-case (cairn/spec/protocol.md, "Values are kebab-case").

## Requirement: Server text is sanitized

A cycle name a server sends SHALL reach the terminal through pimalaya-cli's `sanitize::text`, control characters and terminal escapes replaced, since a TCP server may be remote. `--json` SHALL print it verbatim.

## Requirement: Text output ends with a newline

Every command's text rendering SHALL end with a newline, so the shell prompt starts on a line of its own and a line-buffered pipe receives each `watch` line as it happens.

## Requirement: A JSON error carries a stable code

A failure a caller is expected to act on SHALL carry a stable code, printed under `--json` as a `code` string beside `error`, `sources` and `backtrace`. The code SHALL be found anywhere in the error chain, so added context does not hide it. A failure with no code SHALL print `code: null`. The plain output SHALL stay unchanged. Codes are kebab-case and never renamed; `error` stays free wording.

| Code | Raised when |
|---|---|
| `server-unreachable` | nothing listens at the transport's address; `comodoro server start` starts a server |
| `server-closed` | the server closed the connection before answering |
| `server-error` | the server answered a JSON-RPC error |
| `server-mismatch` | the server's answer cannot be read, as from a server of another Comodoro version; restarting it with this version fixes it |

### Scenario: No server is running

With an account whose socket nobody listens on, `comodoro get --json` prints the failure object with `"code": "server-unreachable"` and exits 1.

## Requirement: The JSON contract is described and versioned

`json-schema` SHALL describe the payload of every command printing data, and the failure object every command prints instead under the key `comodoro-error`, checked by a test walking the command tree: only a bare `comodoro` (the help), `server start` (printing nothing), `repl` and `mcp` are left out. Every generated schema SHALL carry an `$id` naming the contract version, the major version of Comodoro, reading `https://pimalaya.org/schemas/comodoro/v<major>/<key>.json`. Every registered payload SHALL be a JSON object, checked by a test. The keys are `comodoro-get`, `comodoro-watch`, `comodoro-start`, `comodoro-pause`, `comodoro-resume`, `comodoro-stop`, `comodoro-set` and `comodoro-error`.

### Scenario: `--json` output is piped

Each value is printed compact on its own line and flushed, so `watch --json` is newline-delimited JSON a reader receives as each change happens. On a terminal the value is pretty-printed instead.
