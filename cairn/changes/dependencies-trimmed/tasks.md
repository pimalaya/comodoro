---
cairn: tasks
change: dependencies-trimmed
---

# Tasks

- [ ] `cargo tree -e features` before, kept in the scratchpad to diff
- [ ] Cargo.toml: `default-features = false` and named features on every dependency; unused ones gone
- [ ] deny.toml: `[graph] targets` as ortie's, licences trimmed
- [ ] `cargo deny check` clean, or each warning named in the log
- [ ] `cargo fmt --check`, clippy (default, all features, `--no-default-features`), `cargo test --all-features`; `nix build .#default`
- [ ] CHANGELOG: Changed entry
- [ ] Fold the delta into spec/packaging.md, log, land, archive
