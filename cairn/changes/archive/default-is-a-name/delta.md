---
cairn: delta
change: default-is-a-name
---

# Delta

In the spec files a requirement is a `##` heading and a scenario a `###` one.

## ADDED Requirements

None.

## MODIFIED Requirements

### Requirement: Account resolution failures name what is missing
In cairn/spec/config.md. `-a` SHALL name an account as written, `default` being a name like any other; without `-a`, or with an empty one, the account marked `default = true` SHALL be taken. Each of the four ways account resolution fails SHALL name what is missing or wrong and what to do about it: a missing configuration names the path it looked for and points at config.sample.toml, a missing named account lists the accounts the configuration does hold, sorted, adding for the name `default` to omit `-a` to take the account marked default, a missing default names both ways of picking one, and several accounts marked `default = true`, none named, are refused naming them, sorted, rather than one taken. Under `--json` the last three SHALL be coded `account-missing`, `account-default-missing` and `account-default-several`.

#### Scenario: `-a default` names an account
- WHEN only account `a` exists, marked `default = true`, and `comodoro --json -a default get` runs
- THEN it fails coded `account-missing`, listing `a` and saying to omit `-a` to take the account marked default

The other scenario of this requirement is unchanged.

### Requirement: A JSON error carries a stable code
In cairn/spec/output.md. The requirement text is unchanged; the code table gains three rows, before `server-unreachable`: `account-missing` (no account has the name `-a` gave), `account-default-missing` (no account is marked `default = true` and `-a` named none), `account-default-several` (several accounts are marked `default = true` and `-a` named none).

## REMOVED Requirements

None.
