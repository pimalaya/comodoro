---
cairn: spec
capability: json-output
---

# JSON output

What the CLI prints under `--json`, the machine contract a script or an MCP client reads. The JSON-RPC wire a client and a server speak is cairn/spec/protocol.md; the timer object is shared by both.

## Requirement: The JSON output is described and versioned

`json-schema` SHALL describe the payload of every command printing data, and the failure object every command prints instead under the key `comodoro-error`. Every generated schema SHALL carry an `$id` naming the contract version, the major version of Comodoro. Every registered payload SHALL be a JSON object, checked by a test.

The schema keys are `comodoro-get`, `comodoro-watch`, `comodoro-configure` and `comodoro-error`, and an `$id` reads `https://pimalaya.org/schemas/comodoro/v<major>/<key>.json`. The commands driving the timer print a confirmation rather than data, so they carry no schema.

### Scenario: `--json` output is piped

Each value is printed compact on its own line and flushed, so `watch --json` is newline-delimited JSON a reader receives as each change happens. On a terminal the value is pretty-printed instead.
