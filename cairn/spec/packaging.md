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

The library SHALL build without the standard library with no feature, the timer and the protocol being free of I/O. `client` SHALL add the blocking client, `server` the server, `cli` the binary over both, and `notify`, on by default, the desktop notifications of the hooks.

## Requirement: Every feature set builds clean

`cargo clippy --all-targets -- -D warnings` SHALL pass with the default features; with `--all-features`; with `--no-default-features`; with `--no-default-features` and `client`, `server`, or both; and with `--no-default-features --features cli`, notifications left out. Code only some feature sets use SHALL be gated on the item with exactly those features, never allowed crate-wide. CI SHALL run every set, and the tests of a build without notifications.

### Scenario: A client-only build

`cargo clippy --all-targets --no-default-features --features client -- -D warnings` passes.
