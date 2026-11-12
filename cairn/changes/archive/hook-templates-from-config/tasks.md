---
cairn: tasks
change: hook-templates-from-config
---

# Tasks

- [ ] Owner answer on `running` versus `tick`: unanswered at landing, `tick` taken as the proposal recommends
- [x] Tests, each seen failing first: a cycle variable refused on `on-timer-start`; `${cycleName:none}` refused and `${cycleName:-none}` accepted; a command reading `$cycleName` from its environment (a `printf` through pimalaya-config's `command::shell`); a notification and a command on one hook both run; an unknown hook name refused, listing the names; a hook with neither refused
- [x] src/cli/hook.rs: `TimerHook` struct, the variables per event, `execute` rendering and passing them
- [x] src/cli/config.rs: validation when the configuration is read; the `notify` feature check
- [x] tests/hook.rs adapted
- [x] config.sample.toml: hook lines using `$cycleName`
- [x] CHANGELOG: Added (variables, both reactions on one hook), Changed (unknown hook name refused, BREAKING)
- [x] `cargo fmt --check`, clippy (default, all features, `cli` without `notify`), `cargo test --all-features`
- [x] Fold the delta into a new spec/hooks.md, log, land, archive
