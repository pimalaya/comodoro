---
cairn: tasks
change: cli-alignment-2026-11-15
---

# Tasks

- [x] M1 (D6): `cmd` read as the alias of a hook's `command`; `CmdRenamedConfig` removed.
- [x] tests/config.rs `the_cmd_hook_key_reads_as_command`, through the binary, replacing the refusal test, and tests/hook.rs `a_cmd_key_runs_as_the_command`, both failing before the change.
- [x] pimalaya-cli 0.5.2, the Cargo.lock diff limited to that crate, the requirement raised to `0.5.2`.
- [x] The nested-session arm removed, `Command::execute` over a `Session`; the version on `Command`.
- [x] tests/repl.rs: the nested refusal coded `usage-invalid` (by name and prefix), `a_version_line_prints_the_version` failing before the change; tests/mcp.rs on the family wording.
- [x] CHANGELOG `[Unreleased]`, MIGRATION.md.
- [x] Delta folded into cairn/spec hooks.md, config.md, repl.md and mcp.md; log written; status landed; archived.
