---
cairn: delta
change: the-sample-is-tested-and-shipped
---

# Delta

## ADDED Requirements

### Requirement: One sample configuration, with a section per preset
(config) The repository SHALL ship config.sample.toml holding one `[accounts.example]` table with dotted keys and no nested table. It SHALL list the fields every account shares and each of their values, the default active and the others commented as `#key = value`, every line ending in one tab-aligned sentence, and every command a failing `exit 127` placeholder. It SHALL end with one section per cycle preset (Pomodoro, 52/17, 112/26), each a paragraph of commented code only, headed by its name, holding the preset's `cycles`; no `cycles` SHALL be active outside them. The file SHALL hold a working account once one section is uncommented. The test suite SHALL parse config.sample.toml through the loader `-c` uses with each section uncommented whole, then on top of it each commented element inside it and each line of commented code outside the sections, uncommented in place of the active lines of its field, so a configuration change that breaks one fails the build.

#### Scenario: A field is renamed
- GIVEN config.sample.toml, whose commented code sets `cycles-count`
- WHEN the field is renamed in the configuration types and the sample is not updated
- THEN the test suite fails naming the sample

### Requirement: The sample ships beside the binary
(packaging) The Nix package SHALL install config.sample.toml as share/doc/comodoro/config.sample.toml, the release archives SHALL carry it with the share/ of the build they come from, and install.sh SHALL copy it to the same path under its prefix when the downloaded archive holds it, so the first-run welcome copies it rather than downloading it.

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
