---
cairn: log
change: the-tick-is-ticked
landed: 2026-11-14
---

# The tick is ticked

The per-second JSON-RPC notification is `timer.ticked`, past tense as every other notification and in step with the `on-<cycle>-tick` hook; it was `timer.running`. In the library `TimerEvent::Running` is `TimerEvent::Ticked` and `TimerEvent::RUNNING` is `TimerEvent::TICKED`, so the event's `event` discriminator would read `ticked` inside a result, where the server never puts it. The timer state keeps its `running` value. v3 already breaks the wire, so this adds no break for users.

Tests: src/protocol.rs `the_tick_notification_is_ticked` (a started timer's first tick wraps into `timer.ticked`) failed on `timer.running` before the rename; the round-trip, hook-name and timer tests follow the variant.

Capabilities moved: protocol ("Notifications are named after the past tense of what happened", "A tick reports the state it just computed"). CHANGELOG (**Breaking:**) and MIGRATION.md ("Upgrading a running timer") follow.

Left: nothing user-facing carries `running` for the tick; the hook was already `on-<cycle>-tick` and its variables are `cycleName` and `cycleDuration`.
