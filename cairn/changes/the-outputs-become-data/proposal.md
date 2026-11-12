---
cairn: change
id: the-outputs-become-data
status: active
created: 2026-11-12
---

# The outputs become data

## Why

The five timer controls print `printer::Message::new("Timer successfully started")` and the like (src/cli/client/start.rs and its siblings). pimalaya-cli removed `Message` in 0.3.0 (2026-11-03), so they hold the-shared-kit back, and the family's output contract wants every printed value a named `*Output` type and a write answering a receipt (ortie cairn/spec/output.md, "A data command prints a named output type"; himalaya and ortie the-outputs-become-data). A script driving the timer today learns nothing from a control under `--json`, and has to call `get` again to know where the timer stands.

`get` and `watch` print `DisplayTimer` (src/cli/client/timer.rs), which serializes the wire's `Timer` as is: `startedAt` in epoch seconds, where the family prints an RFC 3339 instant in UTC and no output type prints an epoch number (ortie output spec, "Shared shapes and spellings").

## What

- `TimerOutput`, a CLI-owned type in src/cli/client/timer.rs, replaces `DisplayTimer`. It derives `Serialize` and `JsonSchema` in camelCase with `state`, `cycle`, `startedAt`, `elapsed` and `schedule`, the wire's fields, `startedAt` an RFC 3339 instant in UTC or `null` through a `serialize_rfc3339` helper as ortie/src/token/inspect.rs:62 has (humantime, `default-features = false`). The account precision is `#[serde(skip)]` and drives `Display` only.
- Its `Display` stays the one-line status (`[Work] 25min`, `[Work] paused`, `OFF`), which status bars read, ending with a newline.
- `start`, `pause`, `resume`, `stop` and `set` send their request, then `timer.get` on the same `TimerClient` connection, and print the `TimerOutput` of the timer after the change: the receipt, read on the connection that made the change. `Message` leaves every file under src/cli/client/.
- `watch` keeps printing one `TimerOutput` per visible change, until interrupted (owner, 2026-11-12): a status bar piping its text keeps working, and `watch --json` stays newline-delimited JSON. It is the one command printing more than one result, and the spec says so.
- The registry gains `comodoro-start`, `comodoro-pause`, `comodoro-resume`, `comodoro-stop` and `comodoro-set`, all `TimerOutput`, beside `comodoro-get` and `comodoro-watch`.
- spec/json-output.md becomes spec/output.md, the capability's name in Ortie.

## Scope / non-goals

On pimalaya-cli 0.2, so it lands before the-shared-kit and the kit bump has no `Message` left to replace. The cycle name goes through `sanitize::text` once the-shared-kit brings it (0.2 keeps it behind the `table` feature comodoro does not build): a TCP server may be remote, and its cycle name lands on a terminal. The events a control causes stay on the wire; the CLI prints the timer they lead to.
