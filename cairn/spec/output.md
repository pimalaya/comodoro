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

## Requirement: `watch` streams events

`watch` SHALL stream the timer as events through `printer.event`, on pimalaya-cli's streaming printer, so its text lines reach a pipe as they come: one `timer-changed` event on start and one per visible change of the timer, several events of one tick printing one, each the `TimerOutput` keys beside `"event": "timer-changed"` and the timer's one line as text. A termination signal (SIGINT, SIGTERM, SIGHUP, Ctrl-C) SHALL stop it through pimalaya-cli's `signal::Shutdown`, printing one result, `{"stoppedBy": "signal"}` (`Watch stopped` as text), and exiting 0; a second signal ends it at once with 130. The server closing the connection SHALL fail, coded `server-closed`, after the events, and exit 1.

### Scenario: A status bar stops its watch

`comodoro watch --json` piped on a stopped timer prints `{"event":"timer-changed",...,"state":"stopped",...}`, then a `timer-changed` line per change; on SIGTERM it prints `{"stoppedBy":"signal"}` and exits 0.

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
| `usage-invalid` | the command line, a REPL line or an MCP tool call's arguments do not match the grammar, raised by pimalaya-cli; nothing ran |
| `account-missing` | no account has the name `-a` gave |
| `account-default-missing` | no account is marked `default = true` and `-a` named none |
| `account-default-several` | several accounts are marked `default = true` and `-a` named none |
| `server-unreachable` | nothing listens at the transport's address; `comodoro server start` starts a server |
| `server-closed` | the server closed the connection before answering |
| `server-error` | the server answered a JSON-RPC error |
| `server-mismatch` | the server's answer cannot be read, as from a server of another Comodoro version; restarting it with this version fixes it |

### Scenario: No server is running

With an account whose socket nobody listens on, `comodoro get --json` prints the failure object with `"code": "server-unreachable"` and exits 1.

## Requirement: A refused command line exits 2

The command line SHALL be parsed through pimalaya-cli's `clap::parse`: help and version print as clap prints them, and a command line the grammar refuses (an unknown flag or command, a missing argument, a bad value) SHALL exit 2 before any configuration is read. With a `--json` word before any `--`, it SHALL print on stdout the failure object coded `usage-invalid`, its `error` clap's message without its `error: ` prefix nor styles, `sources` empty and `backtrace` null, and nothing on stderr; otherwise clap's rendering on stderr and nothing on stdout. Exit 1 stays the failure object; Comodoro has no partial outcome, which the family numbers from 3.

### Scenario: An unknown flag under JSON

`comodoro --json get --nope` prints one failure object coded `usage-invalid`, its `error` naming `--nope`, on stdout and exits 2.

## Requirement: The JSON contract is described and versioned

`json-schema` SHALL describe the payload of every command printing data, and the failure object every command prints instead under the key `comodoro-error`, checked by a test walking the command tree: only a bare `comodoro` (the help), `server start` (printing nothing), `repl` and `mcp` are left out. Every generated schema SHALL carry an `$id` naming the contract version, the major version of Comodoro, reading `https://pimalaya.org/schemas/comodoro/v<major>/<key>.json`. Every registered payload SHALL be a JSON object, checked by a test. The keys are `comodoro-get`, `comodoro-watch`, `comodoro-start`, `comodoro-pause`, `comodoro-resume`, `comodoro-stop`, `comodoro-set` and `comodoro-error`, and `watch`'s events are described apart as `comodoro-watch.events`, written to `comodoro-watch.events.json`, every alternative carrying `event`, checked by the same test.

### Scenario: `--json` output is piped

Each value is printed compact on its own line and flushed, so `watch --json` is newline-delimited JSON a reader receives as each change happens. On a terminal a result is pretty-printed instead, an event staying one line.
