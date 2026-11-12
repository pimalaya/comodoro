---
cairn: change
id: every-feature-set-builds-clean
status: landed
created: 2026-11-12
---

# Every feature set builds clean

## Why

comodoro's CI runs the shared pimalaya/nix tests workflow, which runs `cargo test --all-features` and no clippy, so a build of the no_std core, of the client or server alone, or of the CLI without notifications can break or warn unseen. Himalaya runs clippy `-D warnings` over its feature matrix in a CI `features` job (himalaya cairn/spec/backends.md "Every backend builds alone", .github/workflows/tests.yml), and calendula and cardamum adopt it under this id.

## What

- `cargo clippy --all-targets -- -D warnings` passes with the default features; `--all-features`; `--no-default-features` (the no_std core); `--no-default-features --features client`; `server`; `client,server`; and `cli` without `notify`.
- Code only some feature sets use is gated on the item with exactly those features, never allowed crate-wide.
- .github/workflows/tests.yml gains Himalaya's `features` job with comodoro's sets.

## Scope / non-goals

No new feature. The tests keep running with `--all-features`.
