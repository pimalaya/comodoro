---
cairn: tasks
change: default-is-a-name
---

# Tasks

- [x] Pin pimalaya-config d5a33cb under `[patch.crates-io]`; `cargo fetch`; Cargo.lock moves the pimalaya-config source line only.
- [x] `ErrorCode` gains the three account codes; `code` maps `AccountError`; `take_account` codes through `with_code`.
- [x] tests/output.rs: `account-missing` (with `-a default`), `account-default-missing`, `account-default-several` through the binary, each failing before the change; the schema test names the three; the several-defaults unit test reads the code.
- [x] Search the docs, sample and tests for `-a default`: none.
- [x] CHANGELOG `[Unreleased]` **Breaking:** line; MIGRATION.md v2 to v3 lines.
- [x] Delta folded into cairn/spec/config.md and output.md; log written; status landed; archived.
