---
cairn: tasks
change: wire-values-are-kebab-case
---

# Tasks

- [ ] Unit tests in src/timer.rs: `TimerState` serializes `running`, `paused`, `stopped`; `TimerLoop` serializes `null` and `4` and reads them back; seen failing first
- [ ] tests/timer.rs: a fake server answering `timer.get` in the 2.x shape gets the version error; seen failing first
- [ ] src/timer.rs: `TimerState` kebab-case; `TimerLoop` through `Option<usize>`, schemars attribute matching
- [ ] src/client/std.rs: the version error in place of "Deserialize timer response error"
- [ ] examples/ and the rest of tests/timer.rs unchanged and passing
- [ ] CHANGELOG: Changed entry, BREAKING, for the wire and `--json` alike
- [ ] `cargo fmt --check`, clippy (default, all features, `--no-default-features`), `cargo test --all-features`; `json-schema -d` diff showing only the two value spellings
- [ ] Fold the delta into spec/protocol.md, log, land, archive
