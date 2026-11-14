---
cairn: delta
change: the-tick-is-ticked
---

# Delta

Every requirement belongs to cairn/spec/protocol.md. In the spec file a requirement is a `##` heading and a scenario a `###` one.

## ADDED Requirements

None.

## MODIFIED Requirements

### Requirement: Notifications are named after the past tense of what happened
`timer.started`, `timer.began`, `timer.ticked`, `timer.durationSet`, `timer.paused`, `timer.resumed`, `timer.ended` and `timer.stopped`. Naming the two directions differently is what keeps a request method and a notification method from ever colliding, which is why `TimerEvent::Set` maps to `timer.durationSet` rather than to `timer.set`.

A notification carries `{"cycle": {...}}` when it concerns a cycle, and no parameters otherwise. It identifies its kind by its method name, so repeating the kind in the parameters would be redundant. Events travelling inside a result have no method name to lean on, so those carry their own `event` discriminator.

#### Scenario: A client receives a notification it does not know
It is ignored. A server pushing something newer than the client understands is not an error.

### Requirement: A tick reports the state it just computed
The per-second tick recomputes the current cycle from the elapsed time, then reports that value. It never reports the value it just replaced.

#### Scenario: The tick stays inside its cycle
It sends `timer.ticked` carrying the remaining duration as of that tick.

#### Scenario: The tick crosses into another cycle
It sends `timer.ended` for the cycle that finished, then `timer.began` for the one starting, and no `timer.ticked`. The started cycle's remaining duration travels with `timer.began`, so a `timer.ticked` beside it would repeat it.

#### Scenario: The tick changes nothing
It sends nothing. A tick lands less than a second after the last state change when a client has just called `timer.set`, and the recomputed duration is then the one already reported.

Two consecutive `timer.ticked` notifications therefore never carry the same duration.

The other scenarios of this requirement are unchanged.

## REMOVED Requirements

None.
