---
cairn: tasks
change: the-outputs-become-data
---

# Tasks

- [ ] Test: `start --json` against an in-process server on a scratch socket prints a `TimerOutput` with `"state": "running"` and an RFC 3339 `startedAt`; seen failing first
- [ ] Test: `get --json` prints `startedAt` as an RFC 3339 instant, `null` once stopped; seen failing first
- [ ] src/cli/client/timer.rs: `TimerOutput` with `serialize_rfc3339`, `Display` as today's line
- [ ] src/cli/client/{start,pause,resume,stop,set}.rs: request, then `get` on the same client, print `TimerOutput`; no `Message` left
- [ ] src/cli/client/{get,watch}.rs on `TimerOutput`
- [ ] src/cli/json_schema.rs: the five control keys; the module doc no longer says the controls carry no schema
- [ ] Cargo.toml: humantime with `default-features = false`, in the `cli` feature
- [ ] CHANGELOG: Changed entries, BREAKING (`startedAt` an RFC 3339 instant under `--json`; the controls print the timer)
- [ ] `cargo fmt --check`, clippy (default, all features), `cargo test --all-features`; `json-schema -d` diff showing the five new keys and `startedAt` a string
- [ ] Fold the delta (spec/json-output.md renamed spec/output.md), log, land, archive
