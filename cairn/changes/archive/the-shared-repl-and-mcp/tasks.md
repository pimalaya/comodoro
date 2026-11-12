---
cairn: tasks
change: the-shared-repl-and-mcp
---

# Tasks

- [x] tests/repl.rs, against an in-process server on a scratch socket: a piped JSON session answers each line and goes on after a failure; a line's `--json` switches that line alone; a session flag is refused by name; a comment line is skipped and a unique prefix inferred; help goes to stderr in a JSON session; `repl`, `mcp`, `watch` and `server start` refused by name
- [x] tests/mcp.rs, against the same: only `get` listed without flags, read-only, `additionalProperties: false`; `--allow-write` lists the controls, `stop` destructive, `openWorldHint` false everywhere; `initialize` names the tools and the flag; a malformed message refused and the server reads on; `get` and `start` answer `structuredContent` matching their `outputSchema`; an unknown argument is a tool error; no server running answers `isError` with `server-unreachable` and the server reads on
- [x] Unit test `the_instructions_name_every_tool` (renamed: no `_` in the tool names)
- [x] Cargo.toml: pimalaya-cli `repl` and `mcp`
- [x] src/cli/repl.rs, src/cli/mcp.rs; src/cli.rs `Repl` and `Mcp`; `SILENT`
- [x] Manual run in the scratchpad: a piped session; an MCP session over stdio (initialize, tools/list with and without `--allow-write`, tools/call `get` and `start`)
- [x] CHANGELOG: Added entries
- [x] `cargo fmt --check`, the clippy matrix, `cargo test --all-features`
- [x] Fold the delta into new spec/repl.md and spec/mcp.md, log, land, archive
