---
cairn: delta
change: the-wizard-leaves
---

# Delta

## ADDED Requirements

### Requirement: Config never written
(config) Comodoro SHALL NOT write a configuration file: the user writes it, from config.sample.toml.

### Requirement: A bare invocation shows the help
(config) A bare `comodoro` SHALL print the help. A command needing an account and finding no configuration SHALL fail naming the path it looked at, the one `-c` or `COMODORO_CONFIG` gave or else the default location, and pointing at config.sample.toml. Comodoro SHALL NOT prompt to create a configuration.

#### Scenario: A first run
- GIVEN no configuration at any default location
- WHEN `comodoro get` runs in a terminal
- THEN it fails naming $XDG_CONFIG_HOME/comodoro/config.toml and the config.sample.toml URL, and asks nothing

## MODIFIED Requirements

### Requirement: The JSON output is described and versioned
(json-output) `json-schema` SHALL describe the payload of every command printing data, and the failure object every command prints instead under the key `comodoro-error`. Every generated schema SHALL carry an `$id` naming the contract version, the major version of Comodoro. Every registered payload SHALL be a JSON object, checked by a test.

The schema keys are `comodoro-get`, `comodoro-watch` and `comodoro-error`, and an `$id` reads `https://pimalaya.org/schemas/comodoro/v<major>/<key>.json`. The commands driving the timer print a confirmation rather than data, so they carry no schema.

#### Scenario: `--json` output is piped
Each value is printed compact on its own line and flushed, so `watch --json` is newline-delimited JSON a reader receives as each change happens. On a terminal the value is pretty-printed instead.

## REMOVED Requirements

None: the wizard had no capability of its own, its requirements living in log/2026-08-14-first-run-wizard.md.
