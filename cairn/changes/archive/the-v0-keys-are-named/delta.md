---
cairn: delta
change: the-v0-keys-are-named
---

# Delta

Every requirement belongs to cairn/spec/config.md. In the spec file a requirement is a `##` heading and a scenario a `###` one.

## ADDED Requirements

None.

## MODIFIED Requirements

### Requirement: Unknown keys are refused at every level
Every configuration table, the root, an account, `socket`, `tcp`, a cycle, a hook and its `notify`, SHALL refuse a key it does not read, naming it, rather than ignore it. No configuration type SHALL flatten another, which would defeat the refusal. A key Comodoro used to read and removed SHALL be refused naming what replaced it and MIGRATION.md: the 0.x root table `presets` naming `accounts`, an account's 0.x `preset` naming the presets of config.sample.toml, its 0.x `timer-precision` naming `precision`, and the removed hook spellings (cairn/spec/hooks.md, "A removed hook spelling is refused by name").

#### Scenario: A 0.x configuration
- WHEN the configuration holds `[presets.example]`
- THEN every command fails with "The table `presets` is now `accounts`, see MIGRATION.md"

The other scenarios of this requirement are unchanged.

## REMOVED Requirements

None.
