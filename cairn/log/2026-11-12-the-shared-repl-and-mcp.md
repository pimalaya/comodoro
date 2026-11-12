---
cairn: log
change: the-shared-repl-and-mcp
landed: 2026-11-12
---

# The shared REPL and MCP server

pimalaya-cli takes its `repl` and `mcp` features. src/cli/repl.rs runs `Repl::new("comodoro", json).run::<Command, ErrorCode>` over the full command tree, refusing `watch` and `server start` by name (they run until interrupted, and pimalaya-cli installs no Ctrl-C handler) and handing every other line to `Command::run` against the account resolved once; `Command::run` refuses `repl` and `mcp` with "Cannot start a session inside a session". src/cli/mcp.rs follows himalaya/src/mcp.rs on `mcp::serve` and `Grammar`, with a `TOOLS` table of `get` (read-only) and, under `--allow-write`, `start`, `pause`, `resume`, `set` and `stop` (destructive), `openWorldHint` false on all; each call parses the argv its grammar writes through `McpLine`, runs into a `CapturePrinter` and answers `ToolResult::printed::<ErrorCode>`. The `mcp` arm of src/cli.rs fails through `exit_on_failure::<ErrorCode, _>`, a person missing a configuration meeting the welcome; the registry check holds `comodoro-repl` and `comodoro-mcp` silent.

The modules live under src/cli/, where comodoro keeps everything the `cli` feature gates, rather than at src/repl.rs and src/mcp.rs as the proposal said.

A smoke run against a scratch server drove a piped session (`get`, `start --json`, `set 60`, `watch` refused, `repl` refused, `-a` refused, `sto` inferred as `stop`) and an MCP session (initialize, tools/list with and without `--allow-write`, `get`, `set`). The `set` tool's argument is `seconds`, the name the grammar takes from the CLI's value name.

Tests: tests/repl.rs (six) and tests/mcp.rs (seven) against an in-process timer server, on a shared tests/common.rs; tests/first_run.rs checks a `mcp` missing its configuration leaves stdout empty; `the_instructions_name_every_tool` replaces the proposal's `the_instructions_name_existing_tools`, comodoro's tool names carrying no `_` to scan for.

Capabilities: repl created; mcp created; output (The JSON contract is described and versioned MODIFIED: `repl` and `mcp` left out).
