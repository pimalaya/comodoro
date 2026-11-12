---
cairn: change
id: the-shared-repl-and-mcp
status: active
created: 2026-11-12
---

# The shared REPL and MCP server

## Why

Every family CLI offers `repl` and `mcp` on pimalaya-cli's shared core, so a person, a script or an agent drives each the same way (himalaya and ortie cairn/spec/repl.md and mcp.md). comodoro offers neither. A comodoro session saves little, each command being one local round trip, but the owner wants the surface consistent everywhere (2026-11-12): an agent tracking work time reads and drives the timer through MCP like any other family tool.

## What

- pimalaya-cli takes its `repl` and `mcp` features.
- src/repl.rs: `comodoro repl` resolves the account once and runs `Repl::new("comodoro", json).run::<Command, ErrorCode>(...)` over the full `Command` tree, as Himalaya's does (rule agreed with the calendula and cardamum sessions, 2026-11-12). `repl` and `mcp` are refused with "Cannot start a session inside a session". `watch` and `server start` are refused with "`<command>` runs until interrupted, run it outside a session", since pimalaya-cli installs no Ctrl-C handler and interrupting them would end the session. Every other line runs the per-command `run(printer, &account)` of the-shared-kit, connecting to the server anew, so a server restarted between two lines is met by the next. The session fixes the shared flags only (`-a`, `-c`, `--log-level`, `--log`, `--log-file`), the transport staying per command.
- src/mcp.rs copies himalaya/src/mcp.rs on `mcp::serve` and `mcp::grammar::Grammar`, with `--allow-write`:
  - `get` is listed always, read-only;
  - `start`, `pause`, `resume`, `set` and `stop` only under `--allow-write`, `stop` destructive since it drops the elapsed time;
  - `openWorldHint` is false on every tool, an effect reaching no one beyond the user (the family rule the owner set on 2026-11-12);
  - `watch`, `server start` and the meta commands are not tools;
  - each tool's input is its command's grammar (the optional transport, `set`'s duration), its output the `TimerOutput` described by `comodoro-<command>`, run into a `CapturePrinter` and answered with `ToolResult::printed::<ErrorCode>`;
  - `initialize` carries one paragraph of instructions: a timer owned by a `comodoro server start` process, `get` for its state, the controls answering the timer after the change, the `--allow-write` gate, and failures as the `comodoro-error` object, `server-unreachable` meaning no server runs.
- src/cli.rs gains `Repl` and `Mcp`; the `mcp` branch fails through `exit_on_failure::<ErrorCode, _>`, a missing configuration meeting the welcome. `SILENT` gains `comodoro-repl` and `comodoro-mcp`.

## Scope / non-goals

No MCP progress notification or streaming tool: the kit has none, which is why `watch` is no tool. No session-held connection.
