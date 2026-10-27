---
cairn: log
change: json-keys-are-camel-case
landed: 2026-10-27
---

# JSON keys became camelCase

`Timer` in src/timer.rs and `GeneratedConfig` in src/cli/configure.rs (the proposal's `ConfigureOutput`) take `#[serde(rename_all = "camelCase")]`. The one key that moves is the timer's `started_at`, now `startedAt`, on the `--json` output of `get` and `watch` and on the JSON-RPC wire alike: the `timer.get` result carries the same `Timer`. src/cli/config.rs stays kebab-case and `TimerPrecision` is untouched.

The regenerated schemas hold no key with an underscore. The rename landed under [Unreleased], marked breaking, with no version bump: the CHANGELOG has no 3.0 section yet, and the bump is the release's call.

Spec updated: protocol (ADDED: JSON keys are camelCase; MODIFIED: The timer is one object, and it says what it runs, `started_at` spelled `startedAt` in its prose and example).
