---
cairn: delta
change: durable-timer-mutations
---

## MODIFIED Requirements

### Requirement: Methods are named after the imperative that performs them

`timer.get`, `timer.start`, `timer.pause`, `timer.resume`, `timer.stop`, `timer.set`, `timer.subscribe` and `timer.unsubscribe`.

`timer.set` takes a `duration` in seconds. Every other method takes no parameters. `timer.get` results in the timer, the subscription methods in `{"subscribed": bool}`, and the rest in `{"events": [...]}`.

A method that cannot apply to the current state answers with an empty `events` array rather than with an error. Starting a running timer, pausing a paused one and stopping a stopped one all report nothing happened, since the caller asked for a state the timer is already in.

#### Scenario: A method the server does not know is called

The server answers -32601, naming the method. It does not close the connection.

#### Scenario: `timer.set` is called without a duration

The server answers -32602, with `["duration"]` as the error data.

#### Scenario: A set duration meets the next tick

It survives. The current cycle is derived from the elapsed time rather than stored, so `timer.set` moves the elapsed time to the point that leaves the requested duration remaining. Every later tick recomputes the same value, and a client reading a second later sees what it set.

#### Scenario: `timer.set` asks for more than the cycle holds

The duration is clamped to the cycle's configured length, and the `set` event carries the effective value rather than the requested one. A longer remaining duration is not representable: it would place the timeline inside the previous cycle, renaming the cycle under the caller.

Setting exactly the configured length restarts the current cycle. Setting zero ends it, and the next tick begins the following one.

#### Scenario: `timer.set` is called on a stopped timer

Nothing happens, and no event comes back. `timer.start` resets the elapsed time, so a duration set beforehand could never survive the start that follows it.

#### Scenario: `timer.stop` is called on a paused timer

It stops, emitting `timer.ended` then `timer.stopped` as it does from a running timer. Only an already stopped timer reports nothing.
