---
cairn: tasks
change: the-wizard-leaves
---

# Tasks

- [x] Delete src/cli/configure.rs, the `Configure` variant and its `wizard` alias
- [x] src/cli.rs: drop `offer_configuration` and the offer inside `take_account`; bare `comodoro` prints the help; a missing configuration fails naming the path and config.sample.toml; drop the long_about pitch
- [x] src/cli/config.rs: drop `AccountConfig::render` and the `Serialize` derives of `Config`, `AccountConfig`, `SocketConfig` and `TcpConfig`; src/cli/hook.rs: drop `Serialize` on `TimerHook`
- [x] src/cli/json_schema.rs: drop `GeneratedConfig` and `comodoro-configure`
- [x] Cargo.toml: drop the pimalaya-cli `prompt` feature, after checking nothing else imports `pimalaya_cli::prompt`
- [x] src/main.rs header: no longer describes the wizard
- [x] README, config.sample.toml header
- [x] CHANGELOG: one Removed entry, BREAKING
- [x] `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` (default and all features), `cargo test --all-features`; `json-schema -d` diff showing only `comodoro-configure` gone (checked through the registry test and printed schemas, no `-d` diff run)
- [x] Fold the delta (spec/config.md created, spec/json-output.md), log, land, archive
