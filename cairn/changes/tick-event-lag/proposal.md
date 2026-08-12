---
cairn: change
id: tick-event-lag
status: landed
created: 2026-08-12
---

# Report the tick that just happened, not the one before it

## Why

`Timer::update` publishes the cycle it is about to replace. It pushes `Running(self.cycle)` and only then writes the value it just computed, so every tick announces the previous second's state. Two consequences, both visible in a server trace.

Values arrive one second late. A timer at 1500 seconds emits `Began(1500)` and then `Running(1500)`, and the tick that already knows the answer is 1499 still says 1500.

A duration written from outside gets published twice. `timer.set` writes the cycle, the next tick republishes it as its stale value, and if that tick lands in the same wall-clock second it also recomputes the same number and stores it again, so a third one follows. Setting 32 seconds produced `Set(32)`, `Running(32)`, `Running(32)`.

The tick also emits `Running` on the second a cycle ends, right before `Ended` and `Began`, so a cycle transition announces the old cycle, its end, and the new cycle in that order, and the following tick repeats the new cycle's duration that `Began` already carried.

Nothing above the timer noticed, because `comodoro watch` compares consecutive timers and drops repeats. The notification stream a third party consumes carries all of it.

## What

`update` reports the state it just computed, and reports it once.

A tick that stays inside its cycle emits `Running` carrying the newly computed remaining duration.

A tick that crosses into another cycle emits `Ended` then `Began`, and no `Running`. `Began` already carries the new cycle, so a `Running` next to it is the same fact twice, and it used to arrive before the `Ended` it follows.

A tick that changes nothing emits nothing. That happens when the clock has not advanced a whole second since the last state write, which is exactly the case a `set` creates. The rule this gives is worth stating on its own: two consecutive `Running` notifications never carry the same duration.

The wire contract does not move. `timer.running` keeps its name, its shape and its meaning. What changes is which value it carries and how often it fires, so it belongs in the spec next to the notification requirement.
