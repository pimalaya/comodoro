---
cairn: delta
change: the-shared-repl-and-mcp
---

# Delta

## ADDED Requirements

### Requirement: A session runs the CLI over one account
(repl) `comodoro repl` SHALL start a session bound to one account and its configuration, and run pimalaya-cli's shared REPL loop over it: read one command per stdin line, split as a shell splits it, a blank line or a line whose first word starts a `#` comment skipped, parse it with the one-shot grammar plus `--json`, subcommand names inferred from a unique prefix, and run it, until `quit`, `exit` or EOF. `repl` and `mcp` SHALL be refused inside it with `Cannot start a session inside a session`, and `watch` and `server start` with `` `<command>` runs until interrupted, run it outside a session ``. A line whose quote is not closed, a line that is not UTF-8, a parse error or a command error SHALL be printed, on stdout as the failure object when piped, and the loop SHALL continue.

#### Scenario: A status bar command in a session
- GIVEN a piped session
- WHEN the lines `watch` and `get` are read
- THEN the first answers that `watch` runs until interrupted and the second prints the timer

### Requirement: Each line connects anew
(repl) Each line SHALL connect to the server on its own, so a server stopped or restarted between two lines is met by the next as a one-shot run meets it.

### Requirement: A piped session answers once per line
(repl) When stdin is not a terminal, every line SHALL produce one flushed answer on stdout: the result, or the failure object (`comodoro-error` under `--json`). Nothing SHALL be added after a result, every text output ending with its own newline. When stdin is a terminal, the prompt `comodoro> ` SHALL render on stderr.

### Requirement: A line switches itself to JSON
(repl) A line carrying `--json` SHALL print its answer as JSON, the lines after it keeping the session's format. `--json` given to `repl` SHALL make every line JSON.

### Requirement: Session flags are refused by name
(repl) A line setting a flag the session fixes when it starts, `-a`, `--account`, `-c`, `--config`, `--log-level`, `--log` or `--log-file`, SHALL fail with ``Cannot set `<flag>` inside a session, start a new session with it instead``, the flag recognised alone, as `--<long>=<value>` or as a short flag with its value attached, before any `--`.

### Requirement: Help keeps JSON lines whole
(repl) A line asking for help SHALL print clap's text on stdout in a text session, and on stderr when the session's `--json` or a `--json` word of the line is set, so stdout keeps one JSON value per line. Its usage line SHALL name the binary before the command, as `Usage: comodoro get [OPTIONS]`.

### Requirement: An MCP server over stdio
(mcp) `comodoro mcp` SHALL run an MCP server (revision 2025-06-18) over stdio for one account on pimalaya-cli's shared MCP core, answering `initialize`, `ping`, `tools/list` and `tools/call`, one JSON-RPC 2.0 message per line, with stdout carrying protocol messages only. A line that is not UTF-8 or not JSON SHALL answer -32700 and one that is not a JSON object, a batch included, -32600, both with a null `id`; the server SHALL read on after each. A failure of the server itself, resolving its account included, SHALL print as text on stderr and exit 1, a person missing a configuration meeting the welcome instead (cairn/spec/config.md).

### Requirement: The server tells the agent how to use it
(mcp) `initialize` SHALL carry `instructions` saying that the timer belongs to a `comodoro server start` process, that `get` reads it, that each control answers the timer after the change, that `--allow-write` gates the controls, and that a failed call answers `isError` with the `comodoro-error` object, `server-unreachable` meaning no server runs. Every tool name it holds SHALL be one of the tool table.

### Requirement: A tool's input is the command's grammar
(mcp) A tool's `inputSchema` SHALL be generated from its clap command by pimalaya-cli's `mcp::grammar::Grammar`, each property named as the CLI spells the argument, with `additionalProperties: false`, and a call SHALL be parsed back by that same command from the argv the grammar writes. An argument the `inputSchema` does not declare SHALL be refused before the command runs as a tool error naming the arguments the tool takes.

### Requirement: A tool's output is the command's payload
(mcp) A successful call SHALL return the command's JSON value as `structuredContent` and as text, described as `outputSchema` by its registered schema. A refused or failed call SHALL return `isError: true` with the `comodoro-error` object and no `structuredContent`; an unknown tool SHALL be a JSON-RPC error.

### Requirement: Writing is opt-in
(mcp) Without flags the server SHALL list `get` only. `--allow-write` SHALL add `start`, `pause`, `resume`, `set` and `stop`. `watch`, `server start` and the meta commands SHALL NOT be tools.

### Requirement: Tool annotations
(mcp) Each tool SHALL carry `readOnlyHint`, `destructiveHint` and `openWorldHint`. `get` alone SHALL be read-only and `stop` alone destructive, since it drops the elapsed time. `openWorldHint` SHALL be false on every tool: it is true only for an effect reaching people beyond the user, which no timer control has.

## MODIFIED Requirements

### Requirement: The JSON contract is described and versioned
(output) `json-schema` SHALL describe the payload of every command printing data, and the failure object every command prints instead under the key `comodoro-error`, checked by a test walking the command tree: only a bare `comodoro` (the help), `server start` (printing nothing), `repl` and `mcp` are left out. Every generated schema SHALL carry an `$id` naming the contract version, the major version of Comodoro, reading `https://pimalaya.org/schemas/comodoro/v<major>/<key>.json`. Every registered payload SHALL be a JSON object, checked by a test. The keys are `comodoro-get`, `comodoro-watch`, `comodoro-start`, `comodoro-pause`, `comodoro-resume`, `comodoro-stop`, `comodoro-set` and `comodoro-error`.

#### Scenario: `--json` output is piped
Each value is printed compact on its own line and flushed, so `watch --json` is newline-delimited JSON a reader receives as each change happens. On a terminal the value is pretty-printed instead.

## REMOVED Requirements

None.
