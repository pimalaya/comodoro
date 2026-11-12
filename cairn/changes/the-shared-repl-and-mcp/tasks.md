---
cairn: tasks
change: the-shared-repl-and-mcp
---

# Tasks

- [ ] tests/repl.rs, against an in-process server on a scratch socket: a piped JSON session answers each line and goes on after a failure; a line's `--json` switches that line alone; a session flag is refused by name; a comment line is skipped and a unique prefix inferred; help goes to stderr in a JSON session; `repl`, `mcp`, `watch` and `server start` refused by name
- [ ] tests/mcp.rs, against the same: only `get` listed without flags, read-only, `additionalProperties: false`; `--allow-write` lists the controls, `stop` destructive, `openWorldHint` false everywhere; `initialize` names the tools and the flag; a malformed message refused and the server reads on; `get` and `start` answer `structuredContent` matching their `outputSchema`; an unknown argument is a tool error; no server running answers `isError` with `server-unreachable` and the server reads on
- [ ] Unit test `the_instructions_name_existing_tools`
- [ ] Cargo.toml: pimalaya-cli `repl` and `mcp`
- [ ] src/repl.rs, src/mcp.rs; src/cli.rs `Repl` and `Mcp`; `SILENT`
- [ ] Manual run in the scratchpad: a piped session; an MCP session over stdio (initialize, tools/list with and without `--allow-write`, tools/call `get` and `start`)
- [ ] CHANGELOG: Added entries
- [ ] `cargo fmt --check`, the clippy matrix, `cargo test --all-features`
- [ ] Fold the delta into new spec/repl.md and spec/mcp.md, log, land, archive
