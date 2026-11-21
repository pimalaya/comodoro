---
cairn: spec
capability: packaging
---

# Packaging

How Comodoro is built and shipped.

## Requirement: The sample ships beside the binary

The Nix package SHALL install config.sample.toml as share/doc/comodoro/config.sample.toml, the release archives SHALL carry it with the share/ of the build they come from, and install.sh SHALL copy it to the same path under its prefix when the downloaded archive holds it, so the first-run welcome copies it rather than downloading it.

## Requirement: Dependencies name their features

Every dependency SHALL be declared with `default-features = false` and the features Comodoro uses, so a default turned on upstream never reaches the binary. clap SHALL be built without `color` and `wrap_help`: help and usage carry no colour, nothing asks the terminal for its size, and help prints each paragraph on one line.

## Requirement: Dependency checks pass clean

`cargo deny check` SHALL pass over the targets deny.toml lists, which are the release targets and the ones people build from source. A duplicate the tree cannot avoid stays a visible warning rather than a skip: today syn 2, which the windows crates under notify-rust still build with on Windows.

## Requirement: Released dependencies only

Every dependency SHALL come from its registry at a released version. A git or path patch SHALL live in `[patch.crates-io]` only while the release it waits for is unpublished, and no dependency or patch section SHALL carry a comment: the reason for a pin lives in cairn.

## Requirement: Feature layers

Comodoro SHALL ship a binary and no library. `notify`, on by default, SHALL add the desktop notifications of the hooks, and `vendored` SHALL forward to the vendored builds of its dependencies; no other feature SHALL exist.

## Requirement: Every feature set builds clean

`cargo clippy --all-targets -- -D warnings` SHALL pass with the default features, with `--all-features` and with `--no-default-features`, notifications left out. CI SHALL run every set, and the tests of a build without notifications.

### Scenario: A build without notifications

`cargo clippy --all-targets --no-default-features -- -D warnings` passes.
