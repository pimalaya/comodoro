---
cairn: log
change: unannounced-endings
landed: 2026-08-13
---

# Announced the endings the timer kept to itself

## Why

Both were found while assessing whether 2.0 was fit to tag, and both were old enough to predate the rewrite.

A timer with a `cycles-count` finished its last loop by flipping its state to stopped and returning nothing. No `timer.ended`, no `timer.stopped`, no `on-timer-stop` hook. The single moment a Pomodoro user waits for was the one moment the timer said nothing. It also stopped halfway, since only the state changed: the cycle, the elapsed time and the start instant kept the values they had, so a completed timer reported itself stopped in the middle of a cycle.

Cycle boundaries were detected by comparing names, so a boundary between two cycles sharing a name went unseen. The exotic-sounding case turned out to be the simplest configuration there is: a single cycle looping forever never announced a round, because the cycle coming back carries the name of the one that just ended.

## What landed

The tick that completes the last loop emits `Ended` for the cycle that ran out, then `Stopped`, and resets. `Timer::stop` did that already, so the reset moved into a method both call and the two paths can no longer drift.

A boundary is now recognised by the remaining duration going back up, as well as by the name changing. Remaining time only ever decreases inside a cycle, so an increase is a boundary whatever the cycles are called. The name comparison stays: a long cycle followed by a short one can cross a boundary while the remaining time still drops, which a suspended machine can produce.

Three unit tests cover it: completion resetting and announcing itself, a single looping cycle announcing every round, and two cycles sharing a name staying two cycles.

## What else landed with it

The hooks had no test at all, which was the other gap the release assessment turned up. tests/hook.rs now covers what the documentation promises: a string command goes through a shell, proven by a redirection landing in a file; an array command does not, proven by a shell metacharacter surviving as part of a file name; a non-zero exit is not an error, since a hook is a reaction rather than a step of the timer; a missing program is; an empty command is refused at deserialization; and every event names the hook it fires. Sending a desktop notification is left alone deliberately, since it would make the suite need a notification daemon.

## Capabilities moved

- protocol: the tick requirement gained the completion scenario, and its boundary scenario now says how a boundary is recognised.
