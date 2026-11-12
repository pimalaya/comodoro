---
cairn: delta
change: every-feature-set-builds-clean
---

# Delta

## ADDED Requirements

### Requirement: Every feature set builds clean
(packaging) `cargo clippy --all-targets -- -D warnings` SHALL pass with the default features; with `--all-features`; with `--no-default-features`; with `--no-default-features` and `client`, `server`, or both; and with `--no-default-features --features cli`, notifications left out. Code only some feature sets use SHALL be gated on the item with exactly those features, never allowed crate-wide. CI SHALL run every set.

#### Scenario: A client-only build
- GIVEN `--no-default-features --features client`
- WHEN `cargo clippy --all-targets -- -D warnings` runs
- THEN it passes

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
