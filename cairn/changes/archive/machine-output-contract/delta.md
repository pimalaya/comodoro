---
cairn: delta
change: machine-output-contract
---

## ADDED Requirements

### Requirement: The JSON output is described and versioned

`json-schema` SHALL describe the payload of every command printing data, and the failure object every command prints instead under the key `comodoro-error`. Every generated schema SHALL carry an `$id` naming the contract version, the major version of Comodoro. Every registered payload SHALL be a JSON object, checked by a test.

#### Scenario: `--json` output is piped

Each value is printed compact on its own line and flushed, so `watch --json` is newline-delimited JSON a reader receives as each change happens. On a terminal the value is pretty-printed instead.
