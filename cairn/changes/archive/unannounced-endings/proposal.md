---
cairn: change
id: unannounced-endings
status: landed
created: 2026-08-13
---

# Announce the endings the timer kept to itself

## Why

Two cases end something without telling anyone.

A timer with a `cycles-count` that finishes its last loop flips its state to stopped and returns no events at all. No `timer.ended`, no `timer.stopped`, and therefore no `on-timer-stop` hook. The one moment a Pomodoro user waits for is the one moment the timer says nothing. It also stops halfway: the state changes while the cycle, the elapsed time and the start instant keep the values they had, so a completed timer reports itself stopped mid-cycle.

A cycle boundary is detected by comparing names, so a boundary between two cycles carrying the same name is missed. That sounds exotic until you write the simplest configuration there is: a single cycle looping forever never announces a round, because the cycle coming back carries the name of the one that just ended. A configuration repeating a name twice in a row has the same hole.

## What

The tick completing the last loop emits `timer.ended` for the cycle that ran out, then `timer.stopped`, and resets the timer to the state a fresh one is in. That is what `timer.stop` already does, so the two paths share one reset.

A boundary is recognised by the remaining time going back up, as well as by the name changing. Remaining time only ever decreases inside a cycle, so an increase is a boundary whatever the cycles are called. The name comparison stays, since a long cycle followed by a short one can cross a boundary while the remaining time drops.
