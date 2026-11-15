---
cairn: tasks
change: wire-unknown-keys-are-refused
---

# Tasks

- [x] Look for a documented reason in cairn/spec, cairn/log, the archived changes and the history of src/timer.rs and src/protocol.rs: none.
- [x] src/protocol.rs `a_wire_object_refuses_a_key_it_does_not_read`, through `TimerResponse::from_value` and `TimerEvent::from_notification`, failing before the change, and failing on its own line with the refusal of `TimerEvent`, `TimerSchedule` or `Timer` alone taken back.
- [x] `deny_unknown_fields` on `TimerCycle`, `TimerSchedule`, `Timer` and `TimerEvent`.
- [x] CHANGELOG `[Unreleased]` **Breaking:** line; MIGRATION.md bullet.
- [x] Delta folded into cairn/spec/protocol.md; log written; status landed; archived.
