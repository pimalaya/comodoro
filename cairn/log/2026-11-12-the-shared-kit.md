---
cairn: log
change: the-shared-kit
landed: 2026-11-12
---

# The shared kit

pimalaya-cli and pimalaya-config moved to 0.4.0 from crates.io, normal and build dependencies, the `[patch.crates-io]` pin gone; clap lost `env` and `wrap_help` and took `suggestions`.

- src/cli.rs flattens pimalaya-cli's `ConfigFlags` (one path per `-c`, no `COMODORO_CONFIG`), resolves the account through `Config::load(paths)?.take_account(name)?`, and meets a bare invocation in `meet_bare_invocation` before any command runs (F6 of cli-alignment-leftovers). `Command::execute` resolves the account for the account-bound commands and hands it to `Command::run`, which the REPL and the MCP server will reuse. The `srv` alias is hidden.
- src/cli/config.rs implements the 0.4 `TomlConfig` (`sample_url`, `accounts`); `take_default_account` and `CONFIG_SAMPLE_URL` are gone (A1 of cli-alignment-leftovers, past 0.2).
- src/main.rs downcasts `ConfigMissingError` and prints the shared `Welcome` in Comodoro's words when `welcome::is_wanted`, exiting 1. Checked under `script` with an empty `HOME`: bare `comodoro` and `comodoro get` print it, with the `curl` command since no sample ships yet.
- src/cli/json_schema.rs builds the registry with `new` and `insert`; `the_registry_matches_the_command_tree` holds `comodoro` and `comodoro-server-start` silent.
- `TimerOutput` prints the cycle name through `sanitize::text`.

Tests: `several_default_accounts_are_refused_naming_them` and `a_delimited_config_value_stays_one_path` failed on the old code first; `a_missing_configuration_is_typed_for_the_welcome` could not compile against 0.1, which has no `ConfigMissingError`. tests/first_run.rs is ported from Himalaya.

An editing slip during this change prepended a copy of two trait methods to src/cli/config.rs (a perl substitution whose `\|` became an alternation matching the empty string); it was removed by hand before the build, and the module doc line left over 80 columns by the-wizard-leaves rewrapped.

Capabilities: config (A bare invocation shows the help MODIFIED; Account resolution failures name what is missing, The configuration path comes from `-c` only, A missing configuration meets a person with the welcome ADDED); output (The JSON contract is described and versioned MODIFIED; Server text is sanitized ADDED).
