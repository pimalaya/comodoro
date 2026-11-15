---
cairn: log
change: the-watch-streams
landed: 2026-11-15
---

# The watch streams

pimalaya-cli is pinned at 51ce8b1 with its `signal` feature. `watch` (src/cli/client/watch.rs) follows the family's "A long-lived command streams events": each visible change is a `timer-changed` event, the `TimerOutput` keys beside `event`, the timer's line as text, written on a streaming printer (src/main.rs) so a status bar's pipe receives the text too. `Shutdown` is installed before connecting; the client reads on a thread while the command polls the request every 100 ms, then prints `{"stoppedBy":"signal"}` (`Watch stopped`) and exits 0. A closed connection fails with `TimerClientError::Closed`, coded `server-closed`, exit 1, where it exited 0. `json-schema` describes the result as `comodoro-watch` and the events as `comodoro-watch.events`.

`error::coded` is gone for `pimalaya_cli::error::with_code` and Comodoro's `code` function (src/cli/error.rs, src/cli.rs). Behaviour unchanged: `code` downcasts the chain as `coded` did, and `with_code` only adds the skip of a chain already coded, `usage-invalid` or partial, which no client failure carries; the server-code binary tests of tests/output.rs and tests/mcp.rs pass unchanged.

Tests: tests/watch.rs `json_events_stream_then_a_signal_prints_the_result` (SIGINT), `text_events_reach_a_pipe_then_sigterm_stops_the_watch` (SIGTERM, text through a pipe), `a_server_closing_the_connection_fails_coded_server_closed`, each failing on the old watch (results without `event`, killed by the signal with no result, exit 0 on a closed connection); src/cli/json_schema.rs `watch_describes_its_events_apart`.

Deviation: the event is a struct variant flattening the timer, `TimerChanged { timer }`. As a newtype variant schemars writes the `event` property beside a `$ref`, valid JSON Schema 2020-12, but pimalaya-cli's registry check follows the `$ref` alone and reports "event without an event key"; a fix there would let a newtype variant through.

Capabilities moved: output ("`watch` prints a result per change" removed, "`watch` streams events" added, "The JSON contract is described and versioned" modified). README, CHANGELOG (**Breaking:**) and MIGRATION.md follow.
