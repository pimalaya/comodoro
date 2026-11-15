---
cairn: spec
capability: mcp
---

# MCP

`comodoro mcp` serves the timer commands of one account to an agent as Model Context Protocol tools over stdio, on pimalaya-cli's shared MCP core (pimalaya-cli cairn/spec/mcp.md).

## Requirement: An MCP server over stdio

`comodoro mcp` SHALL run an MCP server (revision 2025-06-18) over stdio for one account on pimalaya-cli's shared MCP core, answering `initialize`, `ping`, `tools/list` and `tools/call`, one JSON-RPC 2.0 message per line, with stdout carrying protocol messages only. A line that is not UTF-8 or not JSON SHALL answer -32700 and one that is not a JSON object, a batch included, -32600, both with a null `id`; the server SHALL read on after each. Each call SHALL connect to the timer server anew. A failure of the server itself, resolving its account included, SHALL print as text on stderr and exit 1, a person missing a configuration meeting the welcome instead (cairn/spec/config.md).

## Requirement: The server tells the agent how to use it

`initialize` SHALL carry `instructions` saying that the timer belongs to a `comodoro server start` process, that `get` reads it, that each control answers the timer after the change, that `--allow-write` gates the controls, and that a failed call answers `isError` with the `comodoro-error` object, `server-unreachable` meaning no timer server runs. Every tool SHALL be named in it, checked by a test.

## Requirement: A tool's input is the command's grammar

A tool's `inputSchema` SHALL be generated from its clap command by pimalaya-cli's `mcp::grammar::Grammar`, each property named as the CLI spells the argument (`transport`, and `seconds` for `set`), with `additionalProperties: false`, and a call SHALL be parsed back by that same command from the argv the grammar writes. An argument the `inputSchema` does not declare, a required argument missing, and a value the command's grammar refuses, SHALL be refused before the command runs as a tool error whose failure object is coded `usage-invalid`: the first naming the arguments the tool takes, the others naming the argument as the tool's input does, never clap's usage text, through `Grammar::usage_error` (``Missing argument `<name>` ``, ``Invalid value `<value>` for `<name>`: <reason>``).

### Scenario: A negative duration

A `set` call with `seconds: -1` answers `isError` with the failure object coded `usage-invalid` reading ``Invalid value `-1` for `seconds`: invalid digit found in string``, and the server reads on.

## Requirement: A tool's output is the command's payload

A successful call SHALL return the command's JSON value, the `TimerOutput`, as `structuredContent` and as text, described as `outputSchema` by its registered schema. A refused or failed call SHALL return `isError: true` with the `comodoro-error` object and no `structuredContent`; an unknown tool SHALL be a JSON-RPC error.

### Scenario: No timer server runs

A `get` call on an account whose socket nobody listens on answers `isError` with the failure object coded `server-unreachable`, and the server reads on.

## Requirement: Writing is opt-in

Without flags the server SHALL list `get` only. `--allow-write` SHALL add `start`, `pause`, `resume`, `set` and `stop`. `watch`, `server start` and the meta commands SHALL NOT be tools.

## Requirement: Tool annotations

Each tool SHALL carry `readOnlyHint`, `destructiveHint` and `openWorldHint`. `get` alone SHALL be read-only and `stop` alone destructive, since it drops the elapsed time. `openWorldHint` SHALL be false on every tool: it is true only for an effect reaching people beyond the user, which no timer control has.
