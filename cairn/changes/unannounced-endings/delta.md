---
cairn: delta
change: unannounced-endings
---

## MODIFIED Requirements

### Requirement: A tick reports the state it just computed

The per-second tick recomputes the current cycle from the elapsed time, then reports that value. It never reports the value it just replaced.

#### Scenario: The tick stays inside its cycle

It sends `timer.running` carrying the remaining duration as of that tick.

#### Scenario: The tick crosses into another cycle

It sends `timer.ended` for the cycle that finished, then `timer.began` for the one starting, and no `timer.running`. The started cycle's remaining duration travels with `timer.began`, so a `timer.running` beside it would repeat it.

A boundary is recognised by the remaining duration going back up as well as by the cycle name changing, since remaining time only ever decreases inside a cycle. A configuration looping a single cycle, or repeating a name twice in a row, therefore announces its boundaries like any other.

#### Scenario: The tick changes nothing

It sends nothing. A tick lands less than a second after the last state change when a client has just called `timer.set`, and the recomputed duration is then the one already reported.

Two consecutive `timer.running` notifications therefore never carry the same duration.

#### Scenario: The tick completes the last configured loop

A `cycles-count` bounds how many full loops the timer runs. The tick reaching that bound sends `timer.ended` for the cycle that ran out, then `timer.stopped`, and leaves the timer in the state a fresh one is in, exactly as `timer.stop` does.
