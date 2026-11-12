---
cairn: change
id: cli-alignment-leftovers
status: abandoned
created: 2026-10-18
---

# What comodoro still owes the family alignment

## Why

The cross-repo alignment audit of 2026-10-18 (CLI_ALIGNMENT_PLAN.md, which kept the item ids of the 2026-08-28/29 plan) found these items still open in comodoro. Each was re-checked against the code on 2026-10-18.

- **A1**: comodoro is the last binary on pimalaya-config 0.1 (Cargo.toml:75, Cargo.lock on 0.1.4). The rest of the family is on 0.2, which carries `SecretResolver` and the comparable `CommandConfig`.
- **B4**: `AccountConfig::default` serializes even when false (src/cli/config.rs:73-74), so the wizard writes a `default = false` line (src/cli/configure.rs:96) the family omits.
- **B9**: the wizard's output is `GeneratedConfig` (src/cli/json_schema.rs:23), where every data command of the family returns a `*Output` type.
- **F6**: the bare invocation is met inside `Cli::execute` (src/cli.rs:130-155); himalaya meets it in `main` (himalaya/src/main.rs:139).

pimalaya-cli will take over two local copies once its changes land: wizard-save-layer replaces `ExistingConfig`, `append_or_print` and `print_saved` (src/cli/configure.rs:198, :331, :360), shared-account-resolution replaces `take_account` (src/cli.rs:267).

## What

Close each item above, one task each. B9 renames a type the `json-schema` registry names; its payload keys do not move, so it stays clear of json-keys-are-camel-case, held for 3.0.

## Abandoned

Abandoned on 2026-11-12, nothing landing under this id: A1 (pimalaya-config past 0.1) and F6 (the bare invocation met before the command runs) moved to the-shared-kit; B4 and B9 lapse with the wizard, which the-wizard-leaves removes; H1 and H2 lapse with it and with pimalaya-cli's abandoned shared-account-resolution. tasks.md says where each went.
