---
cairn: delta
change: cli-account-model
---

## ADDED Requirements

### Requirement: The timer is one object, and it says what it runs

A timer carries its `schedule`, its `state`, its current `cycle`, the `started_at` it was last started or resumed at, and the `elapsed` seconds accumulated before the last pause or stop.

```json
{
  "schedule": {
    "cycles": [{ "name": "Work", "duration": 1500 }, { "name": "Rest", "duration": 300 }],
    "loops": "Infinite"
  },
  "state": "Running",
  "cycle": { "name": "Work", "duration": 1493 },
  "started_at": 1786711298,
  "elapsed": 0
}
```

The schedule is what the timer was given and never changes: its `cycles` in the order they run, each with its configured duration, and its `loops`, either `"Infinite"` or `{"Fixed": n}` full loops through those cycles. The `cycle` is where the timer is now, and its `duration` is the time remaining rather than the configured one, so it is the field a status bar reads. `state` is `Running`, `Paused` or `Stopped`, and `started_at` is null unless the state is `Running`.

The remaining duration is derived from `started_at` and `elapsed` rather than stored, which is why nothing in the object counts down loops or cycles: an elapsed time names both.

## MODIFIED Requirements

### Requirement: A tick reports the state it just computed

#### Scenario: The tick completes the last configured loop

The schedule's `loops` bounds how many full loops the timer runs, and is spelled `cycles-count` in an account file. The tick reaching that bound sends `timer.ended` for the cycle that ran out, then `timer.stopped`, and leaves the timer in the state a fresh one is in, exactly as `timer.stop` does.
