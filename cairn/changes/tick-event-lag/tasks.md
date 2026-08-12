---
cairn: tasks
change: tick-event-lag
---

# Tasks

- [x] Emit `Running` with the newly computed cycle instead of the one being replaced
- [x] Drop `Running` from a tick that crosses into another cycle
- [x] Emit nothing from a tick that changes nothing
- [x] Update the `Running` variant and `update` documentation
- [x] Update the unit tests asserting the event stream
- [x] Verify against a running server that a set duration is announced once
- [x] Update CHANGELOG.md
- [x] Fold the delta into cairn/spec/protocol.md and write the log entry
