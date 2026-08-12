---
cairn: log
change: tick-event-lag
landed: 2026-08-12
---

# Made the tick report itself instead of the tick before it

## Why

Reported from a server trace: setting the timer to 32 seconds logged the number twice.

`Timer::update` pushed `Running(self.cycle)` and only afterwards stored the cycle it had just computed, so every tick announced the previous second. On its own that made values arrive late. Combined with `timer.set`, which writes the cycle directly, it made them arrive twice: the next tick republished the written value as its stale one, and a tick landing in the same wall-clock second recomputed the same number on top, for a third.

The cycle boundary had the same shape. A transition tick emitted `Running` for the cycle it was leaving, then `Ended`, then `Began`, and the tick after that repeated the duration `Began` had already carried.

None of it showed in `comodoro watch`, which compares consecutive timers and drops repeats. It showed in the notification stream, which is the part third parties consume.

## What landed

A tick reports what it computed, once. Staying inside a cycle emits `Running` with the new remaining duration. Crossing into another emits `Ended` then `Began` and no `Running`, since `Began` already carries the value. Changing nothing emits nothing, which is what a tick landing less than a second after a set now does.

That yields an invariant worth naming, and the spec now names it: two consecutive `timer.running` notifications never carry the same duration.

Verified against a running server. `set 32` used to log `Set(32)`, `Running(32)`, `Running(32)`. It now logs `Set(32)`, `Running(31)`, `Running(30)`. Starting a 1500 second cycle used to log `Began(1500)` then `Running(1500)`, and now logs `Began(1500)` then `Running(1499)`.

## What it cost

The wire contract is unchanged: same method names, same shapes, same meaning. What moved is which value `timer.running` carries and how often it fires, so a client rendering the stream sees one fewer redundant frame per cycle and per set, and a `on-{cycle}-running` hook no longer fires on the second a cycle ends. The `begin` hook fires there instead.

Two unit tests asserted the old stream and were rewritten to the new one, which is the whole point of having them.

## Capabilities moved

- protocol: gained the tick requirement, which the notification section never stated.
