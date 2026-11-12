---
cairn: tasks
change: the-sample-is-tested-and-shipped
---

# Tasks

- [ ] Test `the_sample_configuration_parses_with_each_variant`, seen failing on the current sample
- [ ] config.sample.toml rewritten in Ortie's shape, the three preset sections at the end
- [ ] Cargo.toml: pimalaya-config `sample` in the dev-dependencies
- [ ] package.nix: share/doc/comodoro/config.sample.toml; install.sh copies it
- [ ] `nix build .#default`: result/share/doc/comodoro/config.sample.toml present; the welcome under `script` copies it instead of running `curl`
- [ ] README Configuration points at the sections
- [ ] CHANGELOG: Added (the sample shipped), Changed (the sample's shape)
- [ ] `cargo fmt --check`, clippy (default, all features), `cargo test --all-features`
- [ ] Fold the delta into spec/config.md and a new spec/packaging.md, log, land, archive
