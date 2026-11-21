---
cairn: delta
change: the-binary-alone
---

## MODIFIED Requirements

### Requirement: Feature layers

Comodoro SHALL ship a binary and no library. `notify`, on by default, SHALL add the desktop notifications of the hooks, and `vendored` SHALL forward to the vendored builds of its dependencies; no other feature SHALL exist.

### Requirement: Every feature set builds clean

`cargo clippy --all-targets -- -D warnings` SHALL pass with the default features, with `--all-features` and with `--no-default-features`, notifications left out. CI SHALL run every set, and the tests of a build without notifications.

#### Scenario: A build without notifications

`cargo clippy --all-targets --no-default-features -- -D warnings` passes.
