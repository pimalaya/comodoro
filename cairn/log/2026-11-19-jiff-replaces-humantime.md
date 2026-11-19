---
cairn: log
change: jiff-replaces-humantime
landed: 2026-11-19
---

# jiff replaces humantime

Owner ruling of 2026-11-19, after RUSTSEC-2026-0542 (humantime unmaintained) failed `cargo deny check`. Cargo.toml depends on jiff 1 (`std` only) in the `cli` feature instead of humantime 2, which leaves the lock; jiff 1.0.1 brings jiff-core, jiff-static, portable-atomic and portable-atomic-util.

- src/cli/client/timer.rs `serialize_rfc3339` writes `startedAt` under `--json` from jiff's civil date-time at the Unix epoch plus the seconds, `Z` appended, clamped to 9999-12-31T23:59:59Z, as ortie/src/token/inspect.rs does: the text is the one humantime wrote. tests/output.rs checks it unchanged.
- The running timer's text line prints the remaining duration in jiff's friendly form through `friendly`, ortie's src/token/issued.rs function: `truncate` keeps the precision rules (`second`, `minute`, `hour`, down to the unit under it when the precision's own reads zero), then a `Span` balanced up to days of 24 hours prints. `[Work] 25min` reads `[Work] 25m`, `1h 30min 5s` `1h 30m 5s`, `1h 0min` `1h`, `25h 0min` `1d 1h`. Breaking for a status bar parsing the text; CHANGELOG and MIGRATION.md say so.

Nothing comodoro parses moved: cycle durations and `set` take integer seconds, `precision` unit names; humantime only wrote the instant. JSON output, the wire and `$cycleDuration` keep seconds.

Evidence: unit tests `a_remaining_duration_prints_at_its_precision` (twelve rows over the three precisions, `0s` to `2d 4h 1m 1s`) and `a_duration_past_a_span_prints_in_hours` (`u64::MAX`).

Capabilities moved: output (the requirement "A duration prints in jiff's friendly form", the scenario "A status bar reads the timer").

`cargo fmt`, `cargo test` (default and `--all-features`), `cargo clippy --locked --all-targets -- -D warnings` over the seven CI feature sets pass; `cargo deny check`: advisories, bans, licenses and sources ok, the two warnings left there before (the syn 2 and 3 duplicate, an allowed GitHub organization no source uses).
