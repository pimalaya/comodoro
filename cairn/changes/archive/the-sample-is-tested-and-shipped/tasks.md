---
cairn: tasks
change: the-sample-is-tested-and-shipped
---

# Tasks

- [x] Test `the_sample_configuration_parses_with_each_variant`, seen failing on the current sample
- [x] config.sample.toml rewritten in Ortie's shape, the three preset sections at the end
- [x] Cargo.toml: pimalaya-config `sample` in the dev-dependencies
- [x] package.nix: share/doc/comodoro/config.sample.toml; install.sh copies it
- [x] `nix build .#default`: result/share/doc/comodoro/config.sample.toml present; the welcome under `script` copies it instead of running `curl`
- [x] README Configuration points at the sections
- [x] CHANGELOG: Added (the sample shipped), Changed (the sample's shape)
- [x] `cargo fmt --check`, clippy (default, all features), `cargo test --all-features`
- [x] Fold the delta into spec/config.md and a new spec/packaging.md, log, land, archive
