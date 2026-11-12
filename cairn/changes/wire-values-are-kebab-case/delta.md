---
cairn: delta
change: wire-values-are-kebab-case
---

# Delta

## ADDED Requirements

### Requirement: Values are kebab-case
(protocol) Every value Comodoro emits spelled from a variant name, on the wire and under `--json` alike, SHALL be kebab-case, and a count that may be unbounded SHALL be a number or `null`, never a tagged object.

### Requirement: A peer speaking another version is named
(protocol) A client that cannot read a server's result SHALL fail with "Cannot read the timer server's answer, it may run another Comodoro version: restart it with this one", rather than with the parse error alone.

#### Scenario: A 2.x server still running after an upgrade
- GIVEN a server answering `timer.get` with `"state": "Running"`
- WHEN `comodoro get` runs
- THEN it fails naming another Comodoro version and the restart

## MODIFIED Requirements

### Requirement: The timer is one object, and it says what it runs
(protocol) A timer carries its `schedule`, its `state`, its current `cycle`, the `startedAt` it was last started or resumed at, and the `elapsed` seconds accumulated before the last pause or stop.

```json
{
  "schedule": {
    "cycles": [{ "name": "Work", "duration": 1500 }, { "name": "Rest", "duration": 300 }],
    "loops": null
  },
  "state": "running",
  "cycle": { "name": "Work", "duration": 1493 },
  "startedAt": 1786711298,
  "elapsed": 0
}
```

The schedule is what the timer was given and never changes: its `cycles` in the order they run, each with its configured duration, and its `loops`, the number of full loops through those cycles, `null` when unbounded. The `cycle` is where the timer is now, and its `duration` is the time remaining rather than the configured one, so it is the field a status bar reads. `state` is `running`, `paused` or `stopped`, and `startedAt` is null unless the state is `running`. `startedAt` is in seconds since the Unix epoch, the value a peer computes the remaining time from.

The remaining duration is derived from `startedAt` and `elapsed` rather than stored, which is why nothing in the object counts down loops or cycles: an elapsed time names both.

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

## REMOVED Requirements

None.
