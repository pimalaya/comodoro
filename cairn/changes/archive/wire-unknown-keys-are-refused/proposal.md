---
cairn: change
id: wire-unknown-keys-are-refused
status: landed
created: 2026-11-15
---

# Unknown keys are refused on the wire

## Why

Owner question, 2026-11-15: the wire's `TimerCycle` (src/timer.rs) accepts unknown keys while the parameter and result objects around it (`TimerSetParams`, `TimerEventsResult`, `TimerSubscriptionResult`, `TimerCycleParams`, src/protocol.rs) refuse them; keep the leniency only for a documented, valid reason, otherwise refuse, as everything else does.

No reason is documented. `TimerCycle`, `TimerSchedule`, `Timer` and `TimerEvent` came over from io-timer, where `TimerCycle` doubled as the configuration's cycle, without `deny_unknown_fields`; 5ef4469 (2026-08-12, the JSON-RPC move) gave every new protocol object the refusal and left those four as they were, with no word on it in cairn/log/2026-08-11-jsonrpc-protocol.md. unknown-keys-are-refused (2026-11-14) moved the configuration off `TimerCycle` and kept the wire as it was, "a protocol decision, not a configuration one": a deferral, not a reason. Forward compatibility between peers of different versions is not one either: cairn/spec/protocol.md has a server and its clients upgrade together ("A peer speaking another version is named", and wire-values-are-kebab-case: "No compatibility reader for 2.x values"), and the only tolerance it grants is a notification method the client does not know. A newer key beside `cycle` in a notification or beside `events` in a result is refused already, so the leniency of the objects inside them bought nothing coherent.

## What

- src/timer.rs: `TimerCycle`, `TimerSchedule`, `Timer` and `TimerEvent` (inside a result) take `deny_unknown_fields`. A client reading a result with an unknown key fails as `server-mismatch`, naming another Comodoro version; a notification whose cycle carries one is ignored, as an unknown notification is. Their JSON Schemas, in `json-schema` and the MCP `outputSchema`, gain `additionalProperties: false` on the schedule and the cycle.
- src/protocol.rs: the module docs say so; a test reads a result and a notification with an unknown key at each level.
- cairn/spec/protocol.md, CHANGELOG (**Breaking:**), MIGRATION.md.

## Scope / non-goals

- The JSON-RPC envelope (src/jsonrpc20.rs) follows the specification, not Comodoro's surface, and is not touched.
- The parameterless methods still ignore any `params` they are sent: refusing them is a server answer (-32602), left to its own decision.
- No version bump; no release.
