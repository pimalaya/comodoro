---
cairn: change
id: default-is-a-name
status: landed
created: 2026-11-15
---

# `default` is a name like any other

## Why

Owner ruling of 2026-11-15 (family item 24): no account name is special. pimalaya-config master d5a33cb makes `TomlConfig::take_account` look `default` up as written, an absent or empty name taking the account marked default, and adds "omit `-a` to take the account marked default" to `AccountError::Missing` for that name. Comodoro read `-a default` as the account marked default since v2 (v2.0.0 src/cli.rs:295 filtered it out, the 0.4.0 `take_account` after it), so an account named `default` could not be picked while another was marked default.

The account failures also carried no code under `--json` (`code: null`), where a script needs to tell a typo in `-a` from a configuration marking no default or several. Owner ruling of the same day: code the three `AccountError` variants in each app's own vocabulary.

## What

- Cargo.toml: pimalaya-config pinned in `[patch.crates-io]` at d5a33cba3fe9826b82f15a479043a7f79ca742ce; Cargo.lock moves its source line only.
- src/cli/error.rs: `ErrorCode` gains `AccountMissing`, `AccountDefaultMissing` and `AccountDefaultSeveral` (`account-missing`, `account-default-missing`, `account-default-several`); `code` reads them off an `AccountError` as it reads the server codes off a `TimerClientError`.
- src/cli.rs: `take_account` codes its `AccountError` through `pimalaya_cli::error::with_code`, so the one-shot commands, `repl` and `mcp` all carry the code.
- Tests: tests/output.rs, through the binary under `--json`, each code (`-a default` with only `a` marked default fails `account-missing` with the hint); the `comodoro-error` schema test names the three codes.
- cairn/spec/config.md and output.md, CHANGELOG (**Breaking:**), MIGRATION.md.

## Scope / non-goals

- Comodoro's docs, sample and tests held no `-a default`; nothing else to update.
- The missing configuration keeps its welcome and no code, as before.
