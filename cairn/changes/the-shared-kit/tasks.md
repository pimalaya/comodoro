---
cairn: tasks
change: the-shared-kit
---

# Tasks

- [ ] Tests in src/cli.rs, each seen failing first where the old code allows: `a_missing_configuration_is_typed_for_the_welcome`, `an_unknown_account_lists_the_held_ones`, `several_default_accounts_are_refused_naming_them`, `a_delimited_config_value_stays_one_path`
- [ ] tests/first_run.rs: bare with a configuration, with a broken one and with none; `--json` without one printing the failure object
- [ ] Cargo.toml: pimalaya-cli and pimalaya-config `0.4.0` (normal and build), `[patch.crates-io]` gone, clap without `env` and `wrap_help`
- [ ] Imports from `clap::commands::{completion, manual, json_schema}`; build.rs on the 0.4 build helpers
- [ ] src/cli.rs: `ConfigFlags`, `meet_bare_invocation`, `take_account` through `Config::load`, `Command::execute` split into resolution and `run`; `srv` hidden
- [ ] src/cli/config.rs: the 0.4 `TomlConfig`; `take_default_account` and `CONFIG_SAMPLE_URL` gone
- [ ] src/main.rs: the welcome on `ConfigMissingError`
- [ ] src/cli/json_schema.rs: `new` and `insert`; test `the_registry_matches_the_command_tree`
- [ ] src/cli/client/timer.rs: the cycle name through `sanitize::text`
- [ ] tests/config.rs on the new resolution
- [ ] README and config.sample.toml header: `-c` repeated, layered defaults, no `COMODORO_CONFIG`
- [ ] CHANGELOG: Changed (BREAKING `-c`), Added (welcome), Fixed (several defaults refused)
- [ ] `cargo fmt --check`, clippy (default, all features), `cargo test --all-features`; `json-schema -d` structurally identical but for `comodoro-error`
- [ ] Manual run under `script` with an empty `HOME`: bare `comodoro` and `comodoro get` print the welcome and exit 1
- [ ] Fold the delta into spec/config.md and spec/output.md, log, land, archive
