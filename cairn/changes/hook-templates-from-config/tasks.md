---
cairn: tasks
change: hook-templates-from-config
---

# Tasks

- [ ] Owner answer on `running` versus `tick`
- [ ] Tests, each seen failing first: a cycle variable refused on `on-timer-start`; `${cycleName:none}` refused and `${cycleName:-none}` accepted; a command reading `$cycleName` from its environment (a `printf` through pimalaya-config's `command::shell`); a notification and a command on one hook both run; an unknown hook name refused, listing the names; a hook with neither refused
- [ ] src/cli/hook.rs: `TimerHook` struct, the variables per event, `execute` rendering and passing them
- [ ] src/cli/config.rs: validation when the configuration is read; the `notify` feature check
- [ ] tests/hook.rs adapted
- [ ] config.sample.toml: hook lines using `$cycleName`
- [ ] CHANGELOG: Added (variables, both reactions on one hook), Changed (unknown hook name refused, BREAKING)
- [ ] `cargo fmt --check`, clippy (default, all features, `cli` without `notify`), `cargo test --all-features`
- [ ] Fold the delta into a new spec/hooks.md, log, land, archive
