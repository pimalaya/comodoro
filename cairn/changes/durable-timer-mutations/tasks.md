---
cairn: tasks
change: durable-timer-mutations
---

# Tasks

- [x] Extract the cycle accumulation into a `TimerConfig` method, guarded against a zero-length loop
- [x] Rewrite `Timer::set` to take `now` and move `elapsed` and `started_at`
- [x] Clamp the requested duration to the configured cycle length, and return the effective value
- [x] Make `set` a no-op on a stopped timer
- [x] Let `Timer::stop` act on a paused timer
- [x] Pass `now` at the `TimerRequest::Set` call site in the server
- [x] Cover both fixes with tests that cross a tick
- [x] Update CHANGELOG.md
- [x] Fold the delta into cairn/spec/protocol.md and write the log entry
