---
cairn: tasks
change: dependencies-trimmed
---

# Tasks

- [ ] `cargo tree -e features` before, kept in the scratchpad to diff (moot: Cargo.toml did not move)
- [x] Cargo.toml: `default-features = false` and named features on every dependency; unused ones gone
- [x] deny.toml: `[graph] targets` as ortie's, licences trimmed
- [x] `cargo deny check` clean, or each warning named in the log
- [x] `cargo fmt --check`, clippy (default, all features, `--no-default-features`), `cargo test --all-features`; `nix build .#default`
- [ ] CHANGELOG: Changed entry (none: nothing a user sees moved)
- [x] Fold the delta into spec/packaging.md, log, land, archive
