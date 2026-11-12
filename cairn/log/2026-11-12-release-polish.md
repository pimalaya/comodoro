---
cairn: log
change: release-polish
landed: 2026-11-12
---

# Release polish

README gained the session and MCP features, a `watch --json` recipe and the REPL and MCP sections Ortie's has; its Configuration section covers the welcome, the shipped sample and its preset sections, `-c` repeated and the layered defaults. MIGRATION.md covers 2.x to 3.0: the server restart after upgrading, the configuration and first run, the hooks, the output, with jq recipes each run against a scratch server, and the new commands. SECURITY.md supports 3.x.x. CONTRIBUTING names the two shared crates from crates.io and the clippy feature sets CI runs. The src/lib.rs header names the welcome, the error codes, the REPL and the MCP server, and the user-owned configuration. The CHANGELOG `[Unreleased]` is netted, the breaking changes first, with no pin left in it.

GUIDELINES conformance: cli-001 holds (no product prefix; the `Timer` domain prefix stays where `start` and `server start` would collide), cli-002 through pimalaya-config's resolution, cli-008 with `on-<cycle>-tick` and the hook variables, cli-009 with `ConfigFlags`. cli-003 to cli-007 still describe the wizard and wait for the family guidelines refactor. Cairn conformance (CAIRN.md §8): every file typed, every change with a proposal and tasks, every landed change logged, the delta headings literal.

The final pass moved tests/common.rs to tests/common/mod.rs: cargo discovered it as a test target, which failed to build without the `server` feature, and spec/repl.md quotes the `set` usage line with its `[TRANSPORT]`. Then: `cargo fmt --check`; clippy `-D warnings` over the seven feature sets; `cargo test --all-features`, 105 passing; `cargo deny check`, the syn duplicate its one named warning; `json-schema -d` writing eleven schemas, valid under `check-jsonschema --check-metaschema`; `nix build .#default` carrying share/doc/comodoro/config.sample.toml and share/schemas.

The version stays 2.0.0: the bump to 3.0.0 and the release are the owner's, with the family.

Capabilities: none moved.
