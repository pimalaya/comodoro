---
cairn: tasks
change: cli-alignment-leftovers
---

# Tasks

- [ ] A1: move pimalaya-config to 0.2 (Cargo.toml:75), building secret commands as `CommandConfig`. Moved to the-shared-kit, which takes 0.4.0.
- [ ] B4: skip `default` when false (src/cli/config.rs:73-74, src/cli/configure.rs:96). Lapsed: the-wizard-leaves removes the serializer.
- [ ] B9: rename `GeneratedConfig` to a `*Output` type (src/cli/json_schema.rs:23, src/cli/configure.rs). Lapsed: the-wizard-leaves removes the type.
- [ ] F6: meet the bare invocation in `main`, as himalaya does (src/cli.rs:130-155). Moved to the-shared-kit (`meet_bare_invocation`).
- [ ] H1, H2: once pimalaya-cli ships them, drop src/cli/configure.rs:198, :331, :360 and src/cli.rs:267. Lapsed: the wizard leaves, and shared-account-resolution was abandoned for pimalaya-config's `take_account`.
