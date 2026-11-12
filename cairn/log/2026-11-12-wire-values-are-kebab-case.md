---
cairn: log
change: wire-values-are-kebab-case
landed: 2026-11-12
---

# The wire values became kebab-case

`TimerState` takes `#[serde(rename_all = "kebab-case")]` and `TimerLoop` travels through `Option<usize>` (`#[serde(from, into)]`, `schemars(with)`), so the timer reads `"state": "running"` and `"loops": null` or `4` on the JSON-RPC wire and under `--json` alike. `None` and `Some(0)` both read back as unbounded, as a `cycles-count` of 0 always did. The wire keeps `startedAt` in epoch seconds.

A 3.0 client reading a 2.x answer fails with "Cannot read the timer server's answer, it may run another Comodoro version: restart it with this one" (src/client/std.rs), checked by a fake server in tests/timer.rs answering the 2.x shape. Both regressions failed on the old code first.

Capabilities: protocol (The timer is one object MODIFIED; Values are kebab-case and A peer speaking another version is named ADDED).
