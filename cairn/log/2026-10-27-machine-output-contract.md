---
cairn: log
change: machine-output-contract
landed: 2026-10-27
---

# Adopted the shared machine output contract

pimalaya-cli is pinned at `55b6693` in `[patch.crates-io]` until its next release, moving Cargo.lock from the 0.2.4 release to 0.2.6 from git. src/cli/json_schema.rs returns a `JsonSchemaRegistry` (bin `comodoro`, the crate version) instead of a bare map, and registers `comodoro-error`, the uncoded `ErrorReport`. Comodoro keeps `ErrorReport::eval`: it has no failure codes, so the error object prints exactly as before.

Each schema now carries `$id` = `https://pimalaya.org/schemas/comodoro/v2/<key>.json`. A test asserts every payload root is an object; all already were, so no output moved. Piped `--json` is now one value per line and flushed, from the pimalaya-cli printer, which is what makes `watch --json` usable in a pipe. `watch` had no JSON newline workaround to remove: the newline `DisplayTimer` writes only reaches the plain text output.

Verified: 64 tests pass with all features, clippy clean on all features and targets, `--no-default-features` checks.

Spec updated: json-output (new capability; ADDED: The JSON output is described and versioned).
