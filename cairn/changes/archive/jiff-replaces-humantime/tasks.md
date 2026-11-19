---
cairn: tasks
change: jiff-replaces-humantime
---

# Tasks

- [x] Cargo.toml and Cargo.lock: jiff 1.0.1 in, humantime out
- [x] src/cli/client/timer.rs: `serialize_rfc3339` on jiff; `truncate` and `friendly` drive the text line
- [x] Unit tests: a print table over the three precisions, and a duration past a jiff span; tests/output.rs `startedAt` RFC 3339 checks unchanged and passing
- [x] CHANGELOG Changed (BREAKING), MIGRATION.md Scripts line and example, spec/output.md
- [x] `cargo fmt`, `cargo test` (default, `--all-features`), clippy `-D warnings` over the CI feature sets, `cargo deny check`
- [x] Fold the delta into spec/output.md, log, land, archive
