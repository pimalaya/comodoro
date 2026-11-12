---
cairn: delta
change: dependencies-trimmed
---

# Delta

## ADDED Requirements

### Requirement: Dependencies name their features
(packaging) Every dependency SHALL be declared with `default-features = false` and the features comodoro uses, so a default turned on upstream never reaches the binary. clap SHALL be built without `color` and `wrap_help`: help and usage carry no colour, nothing asks the terminal for its size, and help prints each paragraph on one line.

### Requirement: Dependency checks pass clean
(packaging) `cargo deny check` SHALL pass over the targets deny.toml lists, which are the release targets and the ones people build from source. A duplicate the tree cannot avoid stays a visible warning rather than a skip.

### Requirement: Released dependencies only
(packaging) Every dependency SHALL come from its registry at a released version. A git or path patch SHALL live in `[patch.crates-io]` only while the release it waits for is unpublished, and no dependency or patch section SHALL carry a comment: the reason for a pin lives in cairn.

### Requirement: Feature layers
(packaging) The library SHALL build without the standard library with no feature, the timer and the protocol being free of I/O. `client` SHALL add the blocking client, `server` the server, `cli` the binary over both, and `notify`, on by default, the desktop notifications of the hooks.

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
