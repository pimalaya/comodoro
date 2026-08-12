---
cairn: log
change: durable-timer-mutations
landed: 2026-08-12
---

# Made the mutations the timer accepts actually stick

## Why

Two methods reported success and changed nothing.

`timer.set` wrote the remaining duration onto the current cycle, and the tick recomputed that cycle from the elapsed time a second later, discarding it. Found while smoke-testing something else: `set 300` read back as 300 immediately and as 1497 three seconds on. It had been reachable on the wire since 1.x, and 2.0 is the release that exposes it as `comodoro set`, so it would have shipped a command that does nothing.

`timer.stop` acted only on a running timer, so a paused one could not be stopped at all.

Both survived the suite because every assertion read the state back in the same tick, which is exactly the window where a lost write still looks like a successful one. The lesson is cheap to state and easy to forget: a test that never crosses the tick cannot see a value the tick destroys.

Underneath sat one design fact. The current cycle is derived, not stored: `update` computes it from `elapsed` against the configured cycles. Anything writing the cycle directly was writing to a cache.

## What landed

`Timer::set` takes `now` and moves what the cycle is derived from, rewriting `elapsed` and `started_at` so the derived cycle carries the requested remainder. The next tick recomputes the value it was given rather than overwriting it.

That forced a limit into the open. A cycle cannot hold more than its configured length, because a longer remainder places the timeline inside the previous cycle and renames the cycle under the caller. The request is clamped, and the `set` event carries the effective value, so a client that asked for 9999 on a 1500-second cycle is told it got 1500. Setting the configured length restarts the cycle; setting zero ends it.

`set` on a stopped timer became a no-op returning no events, joining the other methods that decline a state they cannot apply to. It never worked there: `start` resets the elapsed time, discarding whatever was set beforehand.

`Timer::stop` now acts on anything not already stopped.

The cycle accumulation moved from inside `update` to a `TimerConfig` method, since `set` reads the same view. Both now bail out when the cycles add up to no time at all, which used to reach a modulo by zero and panic the tick thread on a configuration a user could write.

The wire contract did not move: `timer.set` still takes a `duration` and still results in events. What changed is that the events are now true a second later.

## What it cost

`Timer::set` gained a `now: u64` parameter, which is the shape the rest of the state machine already had, and the server passes its own clock at the call site.

The tests that read back in the same tick were the reason this survived, so the new ones cross a tick on purpose: the unit tests advance the clock explicitly, and the end-to-end test sleeps past a real server tick before asserting.

## Capabilities moved

- protocol: the method requirement, now stating what a mutation does to the timer rather than only what it looks like on the wire.
