---
cairn: delta
change: jiff-replaces-humantime
---

# Delta

## ADDED Requirements

### Requirement: A duration prints in jiff's friendly form
(output) The remaining duration SHALL be truncated to the account precision (`second`, `minute` or `hour`), down to the unit under it when the precision's own would read zero (`59s` at `minute` or `hour`, `59m` at `hour`), then printed as a jiff `Span` balanced up to days of 24 hours through the default friendly printer: `0s`, `1m 30s`, `25m`, `1h 30m`, `2d 4h`, zero units left out. A duration too long for a span SHALL print in hours. The JSON output and hook variables SHALL keep seconds.

## MODIFIED Requirements

### Requirement: The timer prints as one line
(output) Scenario "A status bar reads the timer": `comodoro get` prints `[Work] 25m` and a newline, where it printed `[Work] 25min`.
