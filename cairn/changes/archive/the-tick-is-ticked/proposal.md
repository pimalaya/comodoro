---
cairn: change
id: the-tick-is-ticked
status: landed
created: 2026-11-14
---

# The tick is ticked

## Why

Every JSON-RPC notification Comodoro pushes is named after the past tense of what happened (cairn/spec/protocol.md, "Notifications are named after the past tense of what happened"), except the per-second one, `timer.running`. The hook it fires was already renamed `on-<cycle>-tick` in this cycle. v3 already breaks the wire (MIGRATION.md, "Upgrading a running timer": a v2 and a v3 peer cannot read each other), so renaming it now costs users no new break. Owner decision, 2026-11-14.

## What

- src/protocol.rs: the notification method is `timer.ticked`, the constant `TimerEvent::TICKED`.
- src/timer.rs: the event is `TimerEvent::Ticked`, so its `event` discriminator inside a result reads `ticked`; the server never puts it in a result today.
- src/cli/hook.rs, src/lib.rs, tests: follow the rename. A protocol test checks the notification a tick emits is `timer.ticked`.
- cairn/spec/protocol.md, CHANGELOG (`[Unreleased]`, **Breaking:**), MIGRATION.md ("Upgrading a running timer").

## Scope / non-goals

- The timer state value `running` (`state: running|paused|stopped`) is unchanged.
- The hook stays `on-<cycle>-tick`; no hook variable or other user-facing name carries `running` for the tick.
- No version bump; no release.
