---
cairn: change
id: the-shared-kit
status: active
created: 2026-11-12
---

# The shared kit

## Why

comodoro pins pimalaya-cli 0.2.6 at git `55b6693` and stays on pimalaya-config 0.1.4, the last of the family on either. Ortie and Himalaya run on both crates' 0.4.0, released on crates.io on 2026-11-11 (ortie cairn/log/2026-11-11-the-shared-kit.md, himalaya the-first-run-welcome). Staying behind keeps comodoro off the shared behaviour the family now documents:

- `-c` reads `COMODORO_CONFIG` and splits on `:` (src/cli.rs:226-235), so a Windows path `C:\...` is read as two, against GUIDELINES cli-009.
- Two accounts marked `default = true` resolve to whichever `HashMap::find_map` meets first (src/cli/config.rs:59), the very trap the 2026-08-14 log names.
- A missing configuration ends at a plain error (the-wizard-leaves), where the family prints the welcome copying the shipped sample.
- The registry is a struct literal (src/cli/json_schema.rs) nothing checks against the command tree.

pimalaya-cli 0.4 no longer carries the 0.2 API (`printer::Message`, the command re-exports, the registry literal) and pimalaya-config 0.4 changes the `TomlConfig` trait, so the two move in one step.

## What

- Cargo.toml: pimalaya-cli and pimalaya-config `0.4.0` from crates.io, normal and build dependencies, `[patch.crates-io]` gone. pimalaya-cli takes `terminal` (`repl` and `mcp` come with the-shared-repl-and-mcp); pimalaya-config keeps `toml` and `secret` and forwards `notify`. clap loses `env` and `wrap_help`.
- Imports from the command modules (`clap::commands::{completion, manual, json_schema}`), the 0.2 re-exports being gone.
- `-c` is pimalaya-cli's `ConfigFlags`: one path per flag, repeatable, never split, no environment variable. BREAKING: `COMODORO_CONFIG` and the `:` delimiter are removed, `-c "$VAR"` replacing them.
- src/cli/config.rs implements pimalaya-config 0.4's `TomlConfig` (`project_name`, `sample_url`, `accounts`, `take_named_account`), as ortie/src/config.rs:37-56. `take_default_account` and `CONFIG_SAMPLE_URL` go. The default paths load as layers, the rc file on top.
- src/cli.rs resolves the account as `Config::load(paths)?.take_account(name)?` (ortie/src/cli.rs:133-139): the shared `AccountError` refuses several defaults naming them, and lists the accounts held for an unknown name. `Command::execute` splits into the resolution and a per-command `run(printer, &Account)`, which the REPL and the MCP server reuse.
- A bare invocation is met before any command runs (`meet_bare_invocation`, as ortie/src/cli.rs:118-129), carrying F6 of cli-alignment-leftovers: the help when a configuration exists, a broken one included, `ConfigMissingError` otherwise.
- src/main.rs downcasts `ConfigMissingError` and, when `welcome::is_wanted`, prints pimalaya-cli's `Welcome` on stderr and exits 1 (ortie/src/main.rs:105-136): intro "Welcome to Comodoro, the CLI to manage timers.", next "Edit it and uncomment the options you need, then start the timer server:", command `server start`, the raw config.sample.toml URL, and `welcome::shipped_sample("comodoro")`.
- src/cli/json_schema.rs builds the registry with `JsonSchemaRegistry::new("comodoro", version)` and `insert::<T>`, checked against the command tree by `registry().check(&Cli::command(), SILENT)`, `SILENT` holding `comodoro` and `comodoro-server-start`.
- The cycle name a server sends goes through `sanitize::text` before reaching a terminal, a TCP server possibly being remote.
- The `srv` alias of `server` becomes hidden, as family aliases are.
- tests/config.rs follows the new `take_account`; a new tests/first_run.rs as Himalaya's.

## Scope / non-goals

The error codes are json-error-code's, the sample's shape and shipping the-sample-is-tested-and-shipped's, the REPL and MCP the-shared-repl-and-mcp's. A1 and F6 of cli-alignment-leftovers land here.
