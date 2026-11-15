---
cairn: log
change: default-is-a-name
landed: 2026-11-15
---

# `default` is a name like any other

pimalaya-config is pinned at d5a33cb in `[patch.crates-io]`, Cargo.lock moving its source line only. `-a default` now names the account called `default`; without `-a` the account marked default is taken, and `-a default` with no such account fails listing the accounts and saying to omit `-a`. Comodoro read it as the account marked default since v2.

The three `AccountError` variants carry codes under `--json`: `account-missing`, `account-default-missing` and `account-default-several` (src/cli/error.rs), read off the chain by `code` beside the server codes. `take_account` (src/cli.rs) codes its failure through `pimalaya_cli::error::with_code`, so the one-shot commands, `repl` and `mcp` carry them alike; the missing configuration keeps its welcome and no code.

Tests: tests/output.rs `a_missing_account_is_coded_account_missing` (`-a default` with only `a` marked default), `no_default_account_is_coded_account_default_missing` and `several_default_accounts_are_coded_account_default_several`, through the binary, each failing on the old code (no code, and `-a default` reaching the server); the `comodoro-error` schema test names the three codes; the several-defaults unit test of src/cli.rs reads the code, the typed `AccountError` no longer reaching the caller.

Capabilities moved: config ("Account resolution failures name what is missing" modified), output ("A JSON error carries a stable code", three codes). CHANGELOG (**Breaking:**) and MIGRATION.md follow. No doc, sample or test of Comodoro said `-a default`.
