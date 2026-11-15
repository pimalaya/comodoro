---
cairn: change
id: cli-alignment-2026-11-15
status: landed
created: 2026-11-15
---

# The CLI alignment round

## Why

The family's CLI alignment round of 2026-11-15 (pimalaya/.github cairn/changes/cli-alignment-2026-11-15, ledger line M1) holds one line for Comodoro, from the owner's ruling D6: "accept it everywhere instead, cmd is a cmon alias, no point to refuse it". Comodoro refused a hook's 1.x `cmd` by name (`CmdRenamedConfig`, src/cli/hook.rs), where pimalaya-config 0.5.1 states `cmd` as the alias of `command` wherever a command is configured, and carillon reads it again in its hooks.

pimalaya-cli 0.5.1 and 0.5.2 refuse a nested `repl` or `mcp` line in the kit, coded `usage-invalid`, answer a `-V` or `--version` line once the binary's command enum carries the version, and name a refused MCP argument as the tool's input does (`Grammar::usage_error`). Comodoro was on 0.5.0, with its own uncoded refusal of a nested session in `Command::run` (src/cli.rs), and clap's wording in a refused MCP call.

## What

- src/cli/hook.rs: `#[serde(alias = "cmd")]` on `TimerHook::command`; `CmdRenamedConfig` and the `cmd` field go.
- Cargo.toml and Cargo.lock: pimalaya-cli `0.5.2`, the lock moving that crate alone.
- src/cli.rs: `#[command(version, long_version = long_version!())]` on `Command`. One `Command::execute` over a `Session` (the configuration paths, the account name, the account once taken), in place of `execute` taking the paths and `run` taking the account: `run` matched every command, so its nested-session arm could not go while it existed; a REPL line and an MCP call now run through `execute`, as in Ortie, Himalaya and Cardamum, where the `repl` and `mcp` arms are the real ones.
- src/cli/repl.rs, src/cli/mcp.rs, src/main.rs: run over the `Session`; MCP answers a refused call through `tool.grammar.usage_error(&err)`.
- Tests: tests/config.rs and tests/hook.rs read `cmd` as the command; tests/repl.rs codes the nested refusal and prints the version; tests/mcp.rs reads the family wording.
- cairn/spec hooks.md, config.md, repl.md, mcp.md; CHANGELOG; MIGRATION.md.

## Scope / non-goals

- pimalaya-config stays at 0.5.0: `#[serde(alias = "cmd")]` needs nothing of 0.5.1.
- No version bump, no release.
