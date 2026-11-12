---
cairn: log
change: the-outputs-become-data
landed: 2026-11-12
---

# The outputs became data

`TimerOutput` (src/cli/client/timer.rs) replaces `DisplayTimer`: the wire's timer fields with `startedAt` serialized as an RFC 3339 instant through humantime's `format_rfc3339_seconds`, the precision skipped and driving the one-line `Display` only. `start`, `pause`, `resume`, `stop` and `set` send their request, then `timer.get` on the same `TimerClient`, and print it; `printer::Message` is gone from every client command. `watch` keeps its loop, one `TimerOutput` per visible change.

The registry describes the five controls with `TimerOutput`, beside `get` and `watch`. tests/output.rs runs the binary against an in-process server on a scratch socket: the three tests failed on the old code first.

The cycle name still reaches the terminal unsanitized: `sanitize::text` comes with the-shared-kit.

Capabilities: output created from json-output (A data command prints a named output type, The timer prints as one line, A timer control answers the timer, `watch` prints a result per change, Shared shapes and spellings, Text output ends with a newline; The JSON contract is described and versioned moved); json-output removed.
