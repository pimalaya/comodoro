---
cairn: change
id: durable-timer-mutations
status: landed
created: 2026-08-12
---

# Make the mutations the timer accepts actually stick

## Why

Two methods report success and change nothing.

`timer.set` writes the remaining duration onto the current cycle, but `Timer::update` recomputes that cycle from the elapsed time on every tick, so the write is gone within a second. Verified against a running server: `set 300` reads back as 300 immediately, and as 1497 three seconds later. The command has always been reachable on the wire, and 2.0 is the release that exposes it as `comodoro set`, so it would ship a command that does nothing.

`timer.stop` acts only on a running timer, so stopping a paused one is a no-op and leaves it stuck in Paused with no way out but restarting the server.

Both survive the test suite because it reads the state back immediately, in the same tick, which is exactly the window where a lost write still looks like a successful one.

The root cause is one design fact: the current cycle is a *derived* value. `update` computes it from `elapsed` against the configured cycles, so anything that writes the cycle directly is writing to a cache, not to the state. The fix is to make a mutation move what the cycle is derived from.

## What

`Timer::set` takes `now` and moves the timeline rather than the displayed value: it finds where the elapsed time currently sits, then rewrites `elapsed` and `started_at` so the cycle it derives has the requested duration remaining. The next tick recomputes the same value it was given.

That makes one limit explicit. A cycle's remaining time cannot exceed the cycle's configured length, since a longer one would place the timeline inside the *previous* cycle and rename the cycle under the user. The requested duration is therefore clamped to the configured length, and the returned event carries the effective value, so a client sees what it got. Setting exactly the configured length restarts the current cycle, and setting zero ends it.

On a stopped timer, `set` becomes a no-op returning no events, joining `pause`, `resume` and the rest. It could never work there anyway: `start` resets the elapsed time, so anything set beforehand was discarded a moment later.

`Timer::stop` acts on any timer that is not already stopped, so a paused timer stops.

The cycle accumulation `update` performs moves into a `TimerConfig` method, since `set` needs the same view. Both then guard against a configuration whose cycles are all zero-length, which currently reaches a modulo by zero and panics the tick thread.
