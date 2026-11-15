---
cairn: delta
change: the-watch-streams
---

# Delta

Every requirement belongs to cairn/spec/output.md. In the spec file a requirement is a `##` heading and a scenario a `###` one.

## ADDED Requirements

### Requirement: `watch` streams events
`watch` SHALL stream the timer as events through `printer.event`, on pimalaya-cli's streaming printer, so its text lines reach a pipe as they come: one `timer-changed` event on start and one per visible change of the timer, several events of one tick printing one, each the `TimerOutput` keys beside `"event": "timer-changed"` and the timer's one line as text. A termination signal (SIGINT, SIGTERM, SIGHUP, Ctrl-C) SHALL stop it through pimalaya-cli's `signal::Shutdown`, printing one result, `{"stoppedBy": "signal"}` (`Watch stopped` as text), and exiting 0; a second signal ends it at once with 130. The server closing the connection SHALL fail, coded `server-closed`, after the events, and exit 1.

#### Scenario: A status bar stops its watch
- GIVEN a stopped timer
- WHEN `comodoro watch --json` runs piped and receives SIGTERM
- THEN it prints `{"event":"timer-changed",...,"state":"stopped",...}`, a `timer-changed` line per change, then `{"stoppedBy":"signal"}`, and exits 0

## MODIFIED Requirements

### Requirement: The JSON contract is described and versioned
The text is unchanged up to the list of keys, which ends: "and `watch`'s events are described apart as `comodoro-watch.events`, written to `comodoro-watch.events.json`, every alternative carrying `event`, checked by the same test."

#### Scenario: `--json` output is piped
- WHEN `watch --json` is piped
- THEN each value is printed compact on its own line and flushed, newline-delimited JSON a reader receives as each change happens; on a terminal a result is pretty-printed instead, an event staying one line

## REMOVED Requirements

### Requirement: `watch` prints a result per change
Replaced by "`watch` streams events": `watch` printed one `TimerOutput` result on start and per change, the one command printing several results.
