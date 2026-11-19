---
cairn: log
change: the-duration-from-the-kit
landed: 2026-11-19
---

# The duration from the kit

Comodoro moved onto pimalaya-cli 0.7.1, released on 2026-11-19, with its new `duration` feature. src/cli/client/timer.rs `friendly` and its test `a_duration_past_a_span_prints_in_hours` are deleted: the timer text calls `pimalaya_cli::duration::friendly`, the same function moved as is, whose tests cover the span limit. `truncate` stays, resting on the `TimerPrecision` setting, and `a_remaining_duration_prints_at_its_precision` now calls the kit through it. jiff stays a dependency of the `cli` feature for `serialize_rfc3339`.

## What reaches users

Nothing: the text output is byte-identical.

## Capabilities moved

None: output keeps its requirement *A duration prints in jiff's friendly form*.

## Verified

`cargo fmt`, `cargo test` (default and `--all-features`), `cargo clippy --locked --all-targets -- -D warnings` over the CI feature sets (default, `--all-features`, `--no-default-features` alone and with `client`, `server`, `client,server` and `cli`) and `cargo deny check` pass.
