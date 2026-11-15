---
cairn: delta
change: wire-unknown-keys-are-refused
---

# Delta

Every requirement belongs to cairn/spec/protocol.md. In the spec file a requirement is a `##` heading and a scenario a `###` one.

## ADDED Requirements

### Requirement: Unknown keys are refused on the wire
Every object of the method surface, the parameters, the results, the timer, its schedule, a cycle and an event inside a result, SHALL refuse a key it does not read rather than ignore it: a server and its clients upgrade together, so an unknown key is another version, never something to skip. The JSON-RPC envelope around them follows the specification instead.

#### Scenario: A result carries a key the client does not read
A `timer.get` result whose cycle carries `"color": "red"` makes `comodoro get` fail naming another Comodoro version and the restart, coded `server-mismatch` under `--json`.

#### Scenario: A notification carries a key the client does not read
It is ignored, as a notification the client does not know is.

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
