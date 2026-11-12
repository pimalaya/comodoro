---
cairn: tasks
change: the-outputs-become-data
---

# Tasks

- [x] Test: `start --json` against an in-process server on a scratch socket prints a `TimerOutput` with `"state": "running"` and an RFC 3339 `startedAt`; seen failing first
- [x] Test: `get --json` prints `startedAt` as an RFC 3339 instant, `null` once stopped; seen failing first
- [x] src/cli/client/timer.rs: `TimerOutput` with `serialize_rfc3339`, `Display` as today's line
- [x] src/cli/client/{start,pause,resume,stop,set}.rs: request, then `get` on the same client, print `TimerOutput`; no `Message` left
- [x] src/cli/client/{get,watch}.rs on `TimerOutput`
- [x] src/cli/json_schema.rs: the five control keys; the module doc no longer says the controls carry no schema
- [x] Cargo.toml: humantime with `default-features = false`, in the `cli` feature
- [x] CHANGELOG: Changed entries, BREAKING (`startedAt` an RFC 3339 instant under `--json`; the controls print the timer)
- [x] `cargo fmt --check`, clippy (default, all features), `cargo test --all-features`; `json-schema -d` diff showing the five new keys and `startedAt` a string (checked through the registry test and printed schemas, no `-d` diff run)
- [x] Fold the delta (spec/json-output.md renamed spec/output.md), log, land, archive
