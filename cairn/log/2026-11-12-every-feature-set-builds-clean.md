---
cairn: log
change: every-feature-set-builds-clean
landed: 2026-11-12
---

# Every feature set builds clean

The seven sets (default, `--all-features`, `--no-default-features`, `client`, `server`, `client,server` and `cli` without `notify`) already passed `cargo clippy --all-targets -- -D warnings` once the changes before this one landed, so no code moved. .github/workflows/tests.yml gained a `features` job on Himalaya's model running clippy over the seven, then the tests of a `cli` build without notifications, which pass locally (the sample test skips its notification variants there).

Capabilities: packaging (Every feature set builds clean ADDED).
