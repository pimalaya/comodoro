---
cairn: delta
change: unknown-keys-are-refused
---

# Delta

Each requirement names its capability file in parentheses. In the spec files a requirement is a `##` heading and a scenario a `###` one.

## ADDED Requirements

### Requirement: Unknown keys are refused at every level
(config) Every configuration table, the root, an account, `socket`, `tcp`, a cycle, a hook and its `notify`, SHALL refuse a key it does not read, naming it, rather than ignore it. No configuration type SHALL flatten another, which would defeat the refusal. A key Comodoro used to read and removed SHALL be refused naming what replaced it and MIGRATION.md (cairn/spec/hooks.md, "A removed hook spelling is refused by name").

#### Scenario: A misspelled cycle key
- WHEN an account's cycles read `{ name = "Work", duraton = 1500 }`
- THEN every command fails naming `duraton` and the keys a cycle takes, `name` and `duration`

### Requirement: A removed hook spelling is refused by name
(hooks) A hook name `on-<cycle>-running`, `<cycle>` one of the account's, SHALL be refused when the account is resolved, naming `on-<cycle>-tick` and MIGRATION.md, and a hook's `cmd` key, the 1.x spelling of `command`, SHALL be refused as the configuration loads, naming `command` and MIGRATION.md.

#### Scenario: A v2 tick hook
- WHEN an account whose cycle is `Work` holds the hook `on-work-running`
- THEN any command fails with "Hook `on-work-running` is now `on-work-tick`, see MIGRATION.md"

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
