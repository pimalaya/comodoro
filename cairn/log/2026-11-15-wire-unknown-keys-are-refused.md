---
cairn: log
change: wire-unknown-keys-are-refused
landed: 2026-11-15
---

# Unknown keys are refused on the wire

The owner asked on 2026-11-15 why the wire's `TimerCycle` still accepted unknown keys when the protocol objects around it refused them, and to refuse them unless a valid reason stood. None did: the four timer types came over from io-timer without the refusal, the JSON-RPC move (5ef4469) gave it to every object it created and left them, unknown-keys-are-refused deferred the question, and the spec has a server and its clients upgrade together, tolerating only a notification method the client does not know. `TimerCycle`, `TimerSchedule`, `Timer` and `TimerEvent` (src/timer.rs) now carry `deny_unknown_fields`, so a result with an unknown key fails the client as `server-mismatch`, naming another Comodoro version, and a notification whose cycle carries one is ignored. Their JSON Schemas gain `additionalProperties: false` on the schedule and the cycle.

Tests: src/protocol.rs `a_wire_object_refuses_a_key_it_does_not_read` (an unknown key in the `timer.get` result's cycle, at its root, in a schedule cycle, in the schedule, in an event's cycle and in an event inside a result, and in a notification's cycle) failed before the change, and failed on its own line with the refusal of `TimerEvent`, `TimerSchedule` or `Timer` alone taken back.

Capabilities moved: protocol ("Unknown keys are refused on the wire" added). CHANGELOG (**Breaking:**) and MIGRATION.md ("Upgrading a running timer") follow.

Left: the parameterless methods still ignore any `params` sent with them; refusing those is a server answer, -32602, not decided here. The JSON-RPC envelope is untouched.
