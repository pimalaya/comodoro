---
cairn: log
change: first-run-wizard
landed: 2026-08-14
---

# Met the newcomer with a wizard rather than with an error

## Why

A first run ended at `Config file not found`, a message naming no path and pointing nowhere, and the bare binary printed a help screen listing commands none of which could run. The 1.x README had promised a wizard "coming soon"; Comodoro has nothing to discover, so keeping the promise turned out to be two questions and a file.

## What landed

`comodoro configure` asks for one of the three documented cycle presets and for the endpoints to serve the timer over, then saves, appends or prints the account. The account name is derived from the preset rather than prompted, suffixed `-2`, `-3` when the configuration already holds it.

A bare `comodoro` and any command needing an account raise the same offer when they find no configuration, behind a welcome naming the file that is missing. The offer is a hook rather than a gate: the command runs afterwards either way. A bare invocation, having nothing to run, falls back to the help.

The three resolution failures now name the path they looked at, the accounts the configuration does hold, and the two ways to pick a default.

## What it cost

The wizard first rendered through `pimalaya_config::toml::to_string`, which puts a whole array on one line: a six-cycle pomodoro came out as a two-hundred-column line with the keys alphabetized, in a file whose whole point is to be edited afterwards. Rendering moved onto `AccountConfig`, hand-written, one cycle per line.

Appending is a literal text append rather than a parse and re-serialize, which is the only write that provably leaves a hand-written document alone. It buys two invariants the account map otherwise breaks silently: a duplicate `[accounts.<name>]` makes the whole file unparseable, and a second `default = true` makes the account every command picks depend on iteration order.

Two documented features turned out not to exist: `COMODORO_CONFIG` was ignored and `-c a.toml:b.toml` was read as one filename. Both were two attributes and a clap feature away.

## What is still open

Unticking the local socket cannot actually stop the server binding it, since an account always resolves one, so an account asking for TCP alone gets `tcp.default` instead. The wizard covers the presets and nothing else: custom cycles, a socket path, another TCP endpoint, the precision and the hooks stay hand-written against config.sample.toml.
