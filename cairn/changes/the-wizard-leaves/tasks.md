---
cairn: tasks
change: the-wizard-leaves
---

# Tasks

- [ ] Delete src/cli/configure.rs, the `Configure` variant and its `wizard` alias
- [ ] src/cli.rs: drop `offer_configuration` and the offer inside `take_account`; bare `comodoro` prints the help; a missing configuration fails naming the path and config.sample.toml; drop the long_about pitch
- [ ] src/cli/config.rs: drop `AccountConfig::render` and the `Serialize` derives of `Config`, `AccountConfig`, `SocketConfig` and `TcpConfig`; src/cli/hook.rs: drop `Serialize` on `TimerHook`
- [ ] src/cli/json_schema.rs: drop `GeneratedConfig` and `comodoro-configure`
- [ ] Cargo.toml: drop the pimalaya-cli `prompt` feature, after checking nothing else imports `pimalaya_cli::prompt`
- [ ] src/main.rs header: no longer describes the wizard
- [ ] README, config.sample.toml header
- [ ] CHANGELOG: one Removed entry, BREAKING
- [ ] `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` (default and all features), `cargo test --all-features`; `json-schema -d` diff showing only `comodoro-configure` gone
- [ ] Fold the delta (spec/config.md created, spec/json-output.md), log, land, archive
