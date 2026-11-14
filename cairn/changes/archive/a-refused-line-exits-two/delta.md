---
cairn: delta
change: a-refused-line-exits-two
---

# Delta

Each requirement names its capability file in parentheses. In the spec files a requirement is a `##` heading and a scenario a `###` one.

## ADDED Requirements

### Requirement: A refused command line exits 2
(output) The command line SHALL be parsed through pimalaya-cli's `clap::parse`: help and version print as clap prints them, and a command line the grammar refuses (an unknown flag or command, a missing argument, a bad value) SHALL exit 2 before any configuration is read. With a `--json` word before any `--`, it SHALL print on stdout the failure object coded `usage-invalid`, its `error` clap's message without its `error: ` prefix nor styles, `sources` empty and `backtrace` null, and nothing on stderr; otherwise clap's rendering on stderr and nothing on stdout. Exit 1 stays the failure object; Comodoro has no partial outcome, which the family numbers from 3.

#### Scenario: An unknown flag under JSON
- WHEN `comodoro --json get --nope` runs
- THEN it prints one failure object coded `usage-invalid`, its `error` naming `--nope`, on stdout and exits 2

## MODIFIED Requirements

### Requirement: A JSON error carries a stable code
(output) A failure a caller is expected to act on SHALL carry a stable code, printed under `--json` as a `code` string beside `error`, `sources` and `backtrace`. The code SHALL be found anywhere in the error chain, so added context does not hide it. A failure with no code SHALL print `code: null`. The plain output SHALL stay unchanged. Codes are kebab-case and never renamed; `error` stays free wording.

| Code | Raised when |
|---|---|
| `usage-invalid` | the command line, a REPL line or an MCP tool call's arguments do not match the grammar, raised by pimalaya-cli; nothing ran |
| `server-unreachable` | nothing listens at the transport's address; `comodoro server start` starts a server |
| `server-closed` | the server closed the connection before answering |
| `server-error` | the server answered a JSON-RPC error |
| `server-mismatch` | the server's answer cannot be read, as from a server of another Comodoro version; restarting it with this version fixes it |

#### Scenario: No server is running
With an account whose socket nobody listens on, `comodoro get --json` prints the failure object with `"code": "server-unreachable"` and exits 1.

### Requirement: A piped session answers once per line
(repl) When stdin is not a terminal, every line SHALL produce one flushed answer on stdout: the result, or the failure object (`comodoro-error` under `--json`), a line the grammar refuses coded `usage-invalid` and printed without clap's `error: ` prefix. Nothing SHALL be added after a result, every text output ending with its own newline. When stdin is a terminal, the prompt `comodoro> ` SHALL render on stderr.

### Requirement: A tool's input is the command's grammar
(mcp) A tool's `inputSchema` SHALL be generated from its clap command by pimalaya-cli's `mcp::grammar::Grammar`, each property named as the CLI spells the argument (`transport`, and `seconds` for `set`), with `additionalProperties: false`, and a call SHALL be parsed back by that same command from the argv the grammar writes. An argument the `inputSchema` does not declare, and a value the command's grammar refuses, SHALL be refused before the command runs as a tool error whose failure object is coded `usage-invalid`, the first naming the arguments the tool takes.

## REMOVED Requirements

None.
