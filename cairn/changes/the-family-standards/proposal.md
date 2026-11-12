---
cairn: change
id: the-family-standards
status: active
created: 2026-11-12
---

# comodoro takes the family standards

## Why

Ortie and Himalaya moved between 2026-11-08 and 2026-11-12 onto a standard the whole family follows (ortie and himalaya cairn/log from 2026-11-08, pimalaya-cli and pimalaya-config 0.4.0, released 2026-11-11): no wizard; the pimalaya-cli and pimalaya-config 0.4 kit (the first-run welcome, `-c` one path per flag, typed account resolution, the shared REPL and MCP server); a named `*Output` type for every printed value, writes answering a receipt, values in kebab-case and instants in RFC 3339; a stable `code` in the failure object; hook templates with variables (GUIDELINES cli-008); a tested sample shipped in share/doc; dependencies naming their features with `cargo deny` clean; every feature set clippy-clean; crates.io dependencies. No app releases alone: comodoro releases as 3.0.0 with the family, so it has to reach the same level first. calendula and cardamum take the same path under the same id.

comodoro 2.0.0 (master 6e20250) stopped at the 2026-10-27 state (camelCase keys, a versioned `$id`, NDJSON when piped):

- pimalaya-cli 0.2.6 behind a git pin (`55b6693`) and pimalaya-config 0.1.4;
- the wizard (`configure`, alias `wizard`), offered from a missing configuration, with `AccountConfig::render` and the `Serialize` derives it needed;
- `-c` reading `COMODORO_CONFIG` and splitting on `:`;
- two `default = true` accounts resolving at random (`HashMap::find_map`, src/cli/config.rs:59);
- the timer controls printing the removed `printer::Message`, and `get`/`watch` printing PascalCase values (`"Running"`, `"Infinite"`, `{"Fixed": n}`) and an epoch `startedAt`;
- the failure object's `code` always `null`;
- hooks with no variables, and a mistyped hook name silently dead;
- a prose config.sample.toml, neither tested nor shipped;
- dependencies on their defaults, no `[graph] targets` in deny.toml, no clippy matrix;
- no REPL and no MCP server.

## What

Eleven changes, landed in this order, each with its own proposal, tasks and delta, each its own commit:

1. the-wizard-leaves: the wizard, `configure` and its alias, the offer, `AccountConfig::render`, the config `Serialize` derives, `comodoro-configure`; bare `comodoro` prints the help. On the 0.2 kit, so it compiles alone.
2. wire-values-are-kebab-case: the JSON-RPC wire takes the family spellings in the same break, `running`, `paused`, `stopped`, `loops` a number or `null`; a peer speaking 2.x is named rather than failing as an opaque parse error. The wire keeps epoch seconds.
3. the-outputs-become-data: `TimerOutput` replaces `DisplayTimer`, `startedAt` an RFC 3339 instant; the timer controls answer the timer after the change; `watch` stays the one command printing a result per change.
4. the-shared-kit: pimalaya-cli and pimalaya-config 0.4.0 from crates.io in one step, the `[patch.crates-io]` pin gone; `ConfigFlags`, `TomlConfig` and `take_account`, the welcome, the registry built by `new` and checked against the command tree. Carries A1 and F6 of cli-alignment-leftovers.
5. json-error-code: a kebab-case `ErrorCode` (`server-unreachable`, `server-closed`, `server-error`, `server-mismatch`) in the failure object.
6. the-sample-is-tested-and-shipped: config.sample.toml in Ortie's shape, a section per cycle preset exactly as Ortie has one per provider, tested through `pimalaya_config::sample::variants`, installed as share/doc/comodoro/config.sample.toml.
7. hook-templates-from-config: a hook takes a command, a notification or both, with `cycleName` and `cycleDuration` as template and environment variables; a hook name no event can fire is refused.
8. dependencies-trimmed: every dependency `default-features = false` naming its features, deny.toml targets and licences, `cargo deny check` clean.
9. every-feature-set-builds-clean: clippy `-D warnings` over the feature matrix (the no_std core, `client`, `server`, `cli` without `notify`), checked in CI.
10. the-shared-repl-and-mcp: `comodoro repl` and `comodoro mcp` on the shared core, the timer controls behind `--allow-write`, `stop` destructive, `openWorldHint` false on every tool.
11. release-polish (Ortie's id, calendula and cardamum using it too): README, MIGRATION.md 2.x to 3.0, SECURITY.md, CONTRIBUTING, the src/main.rs and src/lib.rs headers, the CHANGELOG netted.

Owner decisions, 2026-11-12:

- `watch` keeps printing one result per change, so a status bar piping its text keeps working; the spec names it the one streaming command.
- The wire takes the kebab-case values in the same 3.0 break as the CLI output.
- `--allow-write` gates the timer controls in the MCP server, as in Himalaya.
- The cycle presets become sample sections exactly like Ortie's provider sections, no `cycles` active at the top.
- `openWorldHint` is true only when an effect reaches people beyond the user, the family rule (Himalaya's meaning generalized; Ortie's "reaches a remote server" is the outlier to bring in line).

Rules agreed with the calendula and cardamum sessions on 2026-11-12:

- The REPL runs the full `Command` tree, as Himalaya's does, refusing `repl` and `mcp`, and by name with the reason any command that cannot finish inside a session: pimalaya-cli installs no Ctrl-C handler, so comodoro's `watch` and `server start` would take the session down with them.
- The MCP server copies himalaya/src/mcp.rs (`Grammar` and `mcp::serve`), not Ortie's hand-written tool table.
- A sample section sets no key the top keeps active, since `variants` uncomments it over the active lines and a duplicate key fails the variant.

Family items comodoro does not need, stated so the matrix with calendula and cardamum is complete: published-dependencies (the one git pin leaves with the-shared-kit), live-provider-tests (no provider), capability-shared-api (one backend), `idna_adapter` (no url crate in Cargo.lock) and `SimdCpu::install_detected` (no codec).

## Out

The version bump to 3.0.0 is the owner's call, released with the family. GUIDELINES cli-003 to cli-007 still describe the wizard and wait for the family guidelines refactor.
