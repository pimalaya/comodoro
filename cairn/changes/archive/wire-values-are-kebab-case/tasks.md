---
cairn: tasks
change: wire-values-are-kebab-case
---

# Tasks

- [x] Unit tests in src/timer.rs: `TimerState` serializes `running`, `paused`, `stopped`; `TimerLoop` serializes `null` and `4` and reads them back; seen failing first
- [x] tests/timer.rs: a fake server answering `timer.get` in the 2.x shape gets the version error; seen failing first
- [x] src/timer.rs: `TimerState` kebab-case; `TimerLoop` through `Option<usize>`, schemars attribute matching
- [x] src/client/std.rs: the version error in place of "Deserialize timer response error"
- [x] examples/ and the rest of tests/timer.rs unchanged and passing
- [x] CHANGELOG: Changed entry, BREAKING, for the wire and `--json` alike
- [x] `cargo fmt --check`, clippy (default, all features, `--no-default-features`), `cargo test --all-features`; `json-schema -d` diff showing only the two value spellings (checked through the registry test and printed schemas, no `-d` diff run)
- [x] Fold the delta into spec/protocol.md, log, land, archive
