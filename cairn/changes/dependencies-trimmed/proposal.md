---
cairn: change
id: dependencies-trimmed
status: active
created: 2026-11-12
---

# Dependencies trimmed

## Why

Ortie and Himalaya declare every dependency with `default-features = false` and the features they use, so a default turned on upstream never reaches the binary, and pass `cargo deny check` clean over the targets they build (ortie dependencies-trimmed and deny-warnings-cleared, ortie cairn/spec/packaging.md). comodoro takes most dependencies on their defaults, and its deny.toml lists no `[graph] targets`, so the check walks platforms comodoro never builds and its licence list carries what the tree no longer holds.

## What

- Every dependency, build and dev ones included, takes `default-features = false` and names the features it uses, each checked against the item it gates.
- What nothing uses leaves (each candidate checked with a build, `convert_case` staying for the hook names).
- deny.toml takes ortie's `[graph] targets` (the release targets and the ones people build from source) and the licences the tree carries; `cargo deny check` passes, a duplicate the tree cannot avoid staying a named warning.
- A new spec/packaging.md also records what comodoro already follows: released dependencies only (no git pin, no comment inside a dependency or patch section, the reason for a pin living in cairn, owner rule 2026-11-09), and the feature layers (the no_std core, `client`, `server`, `cli`, `notify`).

## Scope / non-goals

No `idna_adapter` pin (no url crate in Cargo.lock) and no `SimdCpu::install_detected` (no codec). No dependency swapped for another.
