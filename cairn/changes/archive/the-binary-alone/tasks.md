---
cairn: tasks
change: the-binary-alone
---

# Tasks

- [x] Remove src/lib.rs, move its header into src/main.rs, declare the modules there
- [x] Flatten src/client/std.rs and src/server/std.rs
- [x] std imports in place of core and alloc
- [x] Cargo.toml: features, manifest fields, test and example blocks; build.rs ungated
- [x] Move tests/timer.rs, tests/config.rs, tests/hook.rs and the library parts of tests/output.rs and tests/common into the crate or onto the binary
- [x] Remove examples/ and the dead code left behind
- [x] CI feature sets, CONTRIBUTING.md (package.nix needs nothing: it builds the default or given features)
- [x] CHANGELOG, MIGRATION.md
- [x] Fold into spec/packaging.md, log
