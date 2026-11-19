---
cairn: change
id: jiff-replaces-humantime
status: landed
created: 2026-11-19
---

# jiff replaces humantime

## Why

humantime is flagged unmaintained (RUSTSEC-2026-0542), which fails `cargo deny check` in CI. Owner ruling of 2026-11-19 for the family: jiff replaces it, and a printed duration reads in jiff's friendly compact form (`1h 30m`, `2d 4h`, `0s`), as ortie now prints it.

## What

- Cargo.toml: jiff 1 (`std` only, no time zone database) in the `cli` feature, in place of humantime 2.
- src/cli/client/timer.rs: `startedAt` under `--json` written from jiff's civil date-time at the Unix epoch plus the seconds, `Z` appended, clamped to 9999-12-31T23:59:59Z as ortie does; same text as humantime's `format_rfc3339_seconds`.
- src/cli/client/timer.rs: the running timer's text line truncates the remaining seconds to the account precision as before, then prints them through `friendly`, ortie's src/token/issued.rs function: `25m` where it printed `25min`. Breaking for a status bar parsing the text.

## Scope / non-goals

Comodoro parses no human duration: cycle durations in the configuration and `set`'s argument are integer seconds, and `precision` takes unit names, so no input spelling moves. The JSON output, the JSON-RPC wire and the `$cycleDuration` hook variable keep seconds.
