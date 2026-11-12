---
cairn: log
change: dependencies-trimmed
landed: 2026-11-12
---

# Dependencies trimmed

Every dependency in Cargo.toml was already declared with `default-features = false` and named features once the-shared-kit and the-outputs-become-data landed (clap without `env` and `wrap_help`, humantime and tempfile bare), and each is used, so Cargo.toml did not move.

deny.toml took Ortie's `[graph] targets` and lost `allow-org.github`, the pimalaya-cli git pin being gone. `cargo deny check` passes: advisories, bans, licences and sources ok, the licence list (Apache-2.0, MIT, MPL-2.0, Unicode-3.0) all in use. One warning stays, named in the spec: syn 2 beside syn 3, pulled on Windows by the windows crates under notify-rust.

No `idna_adapter` pin (no url crate in the tree) and no `SimdCpu::install_detected` (no codec).

Capabilities: packaging (Dependencies name their features, Dependency checks pass clean, Released dependencies only, Feature layers ADDED).
