# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

Comodoro joins the standard the Pimalaya CLIs share. MIGRATION.md lists what to act on when upgrading from 2.x.

### Added

- Added the first-run welcome: run in a terminal with no configuration, `comodoro` and every command needing an account print the commands creating one from config.sample.toml, copied from share/doc/comodoro/ beside the binary when it ships there and downloaded otherwise, then exit 1. Scripts and `--json` callers keep getting the error.
- Added config.sample.toml to the Nix package and the release archives as share/doc/comodoro/config.sample.toml, which install.sh copies too.
- Added `comodoro repl`, a session running the CLI's commands over one account, one per stdin line, on the REPL Himalaya and Ortie share; `watch` and `server start` are refused in it, since they run until interrupted, and a `repl` or `mcp` line, by name, alias or prefix, is refused coded `usage-invalid` by pimalaya-cli. A `-V` or `--version` line prints the version the command line prints.
- Added `comodoro mcp`, a Model Context Protocol server over stdio exposing `get` to an agent, and with `--allow-write` the timer controls, each answering the timer as `structuredContent`. A call missing an argument or giving a value the command refuses names the argument as the tool's input does (``Invalid value `-1` for `seconds`: invalid digit found in string``), coded `usage-invalid`, rather than clap's usage text.
- Added hook variables, after GUIDELINES cli-008: a cycle hook fills `cycleName` and `cycleDuration` (the remaining seconds), expanded in its notification (`$cycleName`, `${cycleName}`, `${cycleName:-default}`) and handed to its command as environment variables of the same names. A hook can now run a command and show a notification both, the command first.
- Added a stable `code` to the `--json` failure object of the client commands: `server-unreachable`, `server-closed`, `server-error` and `server-mismatch`, and of an account `-a` cannot take: `account-missing`, `account-default-missing` and `account-default-several`, described by `comodoro-error`.
- Added the plural aliases `completions`, `manuals` and `json-schemas`, hidden from `--help`.
- Added `cmd` as the alias of a hook's `command`, as wherever the family configures a command: a 1.x hook spelling `cmd` loads again, where 2.x refused it as an unknown key.

### Changed

- Changed the pimalaya-cli dependency to 0.7.1.
- **Breaking:** changed the timer's values to kebab-case, both in the `--json` output and in the JSON-RPC protocol: `state` reads `running`, `paused` or `stopped` instead of `Running`, `Paused` or `Stopped`, and the schedule's `loops` is the number of loops or `null` when unbounded instead of `"Infinite"` or `{"Fixed": n}`. A client meeting a server of the previous version fails saying it may run another Comodoro version: restart the server, and any `watch` still running, after upgrading.
- **Breaking:** renamed the timer's `started_at` key to `startedAt`, both in the `--json` output and in the JSON-RPC protocol (the `timer.get` result), so every JSON key Comodoro emits is camelCase.
- **Breaking:** changed `startedAt` under `--json` to an RFC 3339 instant in UTC (`2026-11-12T09:00:00Z`) instead of epoch seconds. The JSON-RPC protocol keeps epoch seconds.
- **Breaking:** changed `start`, `pause`, `resume`, `stop` and `set` to print the timer after their change, as `get` does, instead of a confirmation sentence: the one-line status as text, and the timer object under `--json`, each described in `json-schema` (`comodoro-start` and the others).
- **Breaking:** changed `-c` to take one path per flag, repeated to merge several files, and removed the `COMODORO_CONFIG` environment variable and the `:` delimiter, since `:` belongs to paths (`C:\` on Windows): use `-c "$VAR"`. Without `-c`, every existing default path now loads, each merged on top of the ones before, `$HOME/.comodororc` last.
- **Breaking:** renamed the per-second cycle hook `on-<cycle>-running` to `on-<cycle>-tick`, the event being a bare verb as GUIDELINES cli-008 asks.
- **Breaking:** renamed the per-second JSON-RPC notification `timer.running` to `timer.ticked`, past tense as every other notification is and in step with the `on-<cycle>-tick` hook; the timer state keeps its `running` value. In the library, `TimerEvent::Running` becomes `TimerEvent::Ticked` and `TimerEvent::RUNNING` becomes `TimerEvent::TICKED`.
- **Breaking:** changed account resolution to refuse a hook name no event of the account fires (such as a mistyped `on-wrok-end`, which used to load and stay silent), a hook with neither `command` nor `notify`, a notification naming a variable its hook does not fill, and an account with no cycle, which used to load and fail once the server started.
- **Breaking:** changed a cycle to refuse a key other than `name` and `duration`, which it ignored, as every other configuration table already refused a key it does not read; the removed hook name `on-<cycle>-running` is refused naming `on-<cycle>-tick` and MIGRATION.md.
- **Breaking:** changed the 0.x configuration keys to be refused by name pointing at MIGRATION.md, where they were refused as any unknown key: the root table `presets` names `accounts`, an account's `preset` the presets of config.sample.toml, and its `timer-precision` names `precision`. In the library, `Config` gains `presets` and `AccountConfig` gains `preset` and `timer_precision`, typed so they never hold a value.
- **Breaking:** changed `AccountConfig::cycles` in the library to a `Vec<CycleConfig>`, the configuration's own cycle refusing unknown keys, where it was a `Vec<TimerCycle>`, the wire type; `TimerCycle::from` turns a `CycleConfig` into one.
- **Breaking:** changed the JSON-RPC protocol to refuse a key an object does not read in the timer, its schedule, a cycle and an event inside a result, as the parameters and results around them already did: a client meeting a result with an unknown key fails saying the server may run another Comodoro version, and ignores a notification whose cycle carries one. The schedule and the cycle say `additionalProperties: false` in the `json-schema` schemas and the MCP `outputSchema`.
- **Breaking:** changed `watch` to stream the timer as events: each change is a `timer-changed` event, the timer's keys beside `"event": "timer-changed"` under `--json`, where it printed a bare timer object per change; a script selects `.event == "timer-changed"` lines. A termination signal (SIGINT, SIGTERM, SIGHUP, Ctrl-C) stops it cleanly with one result, `{"stoppedBy":"signal"}` (`Watch stopped` as text), and exit 0, where the signal killed it; the server closing the connection fails coded `server-closed`, exit 1, where it exited 0. Its events are described apart as `comodoro-watch.events` by `json-schema`, its result by `comodoro-watch`.
- **Breaking:** changed `-a default` to name the account called `default`, as any other name, where it took the account marked `default = true`: omit `-a` to take that one. With no account called `default`, it fails saying so, coded `account-missing`.
- **Breaking:** changed a command line the grammar refuses under `--json` to print the failure object coded `usage-invalid` on stdout, where clap printed its text on stderr; it still exits 2, the family keeping 2 for a refused line and 3 for a partial outcome. A refused REPL line and an MCP tool call whose arguments the grammar refuses carry `usage-invalid` too.
- **Breaking:** changed `cli::Command::execute` in the library to take a `cli::Session`, holding the configuration paths, the account name and the account once taken, where it took the paths and the name; a REPL line and an MCP call run through it too.
- Changed config.sample.toml to one `[accounts.example]` table of dotted keys, each line ending in a tab-aligned comment, and the cycle presets to sections to uncomment, Pomodoro, 52/17 and 112/26: a copied sample holds no active `cycles` until one is uncommented. A test parses it with every section and option uncommented.
- Changed piped `--json` output to print one value per line, flushed, so `watch --json` streams newline-delimited JSON instead of values glued together and held in the buffer; each schema of `json-schema` now carries an `$id` naming the contract version, the major version of Comodoro.
- Changed the `srv` alias of `server` to a hidden one.
- Moved to pimalaya-cli 0.5.2 (with its `signal` feature) and pimalaya-config 0.5.
- **Breaking:** changed the remaining duration of the timer's text line to jiff's friendly form, as the other Pimalaya CLIs print a duration: `[Work] 25m` instead of `[Work] 25min`, `1h 30m 5s` instead of `1h 30min 5s`, zero units left out (`1h` instead of `1h 0min`), and a day and over in days (`1d 1h` instead of `25h 0min`). The precision truncates as before. The `--json` output and the `$cycleDuration` hook variable keep seconds.
- Changed the humantime dependency to jiff 1, humantime being unmaintained (RUSTSEC-2026-0542); `startedAt` under `--json` is written as before.

### Removed

- **Breaking:** removed the setup wizard, the `configure` command and its `wizard` alias, with the `comodoro-configure` schema. A bare `comodoro` prints the help, and a person without a configuration meets the first-run welcome instead of the wizard: write the configuration from the sample.

### Fixed

- Fixed, through pimalaya-cli 0.6.1 and pimalaya-config 0.6.0, a value filled into a hook notification being read as markup by the notification daemon: a cycle name holding `<b>` or `&` is shown as written, its line breaks and tabs as spaces, its other control and bidi override characters dropped. Fixed the text error report writing the control characters of its causes raw to the terminal, `--log-file` creating the file under the umask and following a symbolic link (it is created at 0600 and a link is refused), the manual pages printing `vv<version>`, a TOML parse error of the configuration printing its source line, and a refused secret echoing its value; a secret command that fails names the command and says to run it yourself, never printing its output.
- Fixed install.sh saving an HTTP error page as the archive and letting `tar` fail on it: it downloads with `curl -fsSLo` and stops on the error.
- Fixed several accounts marked `default = true` resolving to whichever came first: they are refused, naming them.
- Fixed `completion` writing files to the working directory instead of printing the script to stdout.

  It broke every packaging helper capturing stdout. `manual` and `json-schema` now share its shape: a positional list selecting what to generate, printed to stdout, and an optional `--dir` deciding where it lands instead of the directory they used to take as a positional argument.

  `comodoro manual ./man` becomes `comodoro manual --dir ./man`, and the three accept command names (`comodoro`, `comodoro-get`) to generate a single item.

## [2.0.0] - 2026-08-14

### Added

- Adopted [JSON-RPC 2.0](https://www.jsonrpc.org/specification) as the wire protocol, framed as NDJSON over a local socket or over TCP.

  The specification defines the payload and leaves the transport alone, so a client is implementable in any language. Failures come back as standard error codes.

- Added timer notifications, the capability the protocol change was for.

  A connection calling `timer.subscribe` receives `timer.started`, `timer.began`, `timer.running`, `timer.durationSet`, `timer.paused`, `timer.resumed`, `timer.ended` and `timer.stopped`.

- Added the `watch` command, which subscribes and prints the timer on every change until interrupted, so a status bar no longer has to poll.
- Added the `set` command, exposing the `timer.set` method that was previously reachable on the wire but not from the CLI.
- Added the `configure` command, which generates an account from one of the documented cycle presets.

  It asks for the preset, names the account after it, then appends it to the configuration file, or prints it on stdout when declined, redirected or in JSON mode.

  The append is plain text, so comments and formatting survive. A taken name is suffixed, and `default` is claimed only when no other account holds it.

  A bare `comodoro` and any command needing an account open with a welcome naming the missing file, then offer the wizard. Declining is a hook, not a gate: the command carries on.

  The generated account holds its cycles alone, since every other field has a default. Custom cycles, addresses, precision and hooks stay hand-written.

- Added the `json-schema <DIR>` command, writing one JSON Schema file per command emitting structured output, so a script consuming `--json` can validate what it reads.

  Three land today, `comodoro-get`, `comodoro-watch` and `comodoro-configure`. The commands driving the timer report a confirmation rather than data, so they carry none.

- Turned Comodoro into a library as well as a binary, by merging the io-time crate into this repository.

  Three layers: the timer and protocol types (the only no_std one, always compiled), a blocking client and server (`client` and `server` features), and the CLI (`cli`, default).

### Changed

- **BREAKING** Renamed the `unix-socket` configuration table to `socket`, and gave every transport field a default, so a table adjusts an address rather than enabling a transport.

  `socket.path` defaults to comodoro.sock inside `$XDG_RUNTIME_DIR` or the temporary directory, `tcp.host` to 127.0.0.1 and `tcp.port` to 9999.

  An account therefore requires nothing but its `cycles`, and `unix-socket` stays an accepted alias so a 1.x file still loads.

  Which transport runs is no longer configured: `server start` binds what it is given, the default one when given none. A client still follows `socket.default` and `tcp.default`.

- **BREAKING** Renamed the `[PROTOCOL]` argument to `[TRANSPORT]`, since a protocol is now what the two peers speak rather than what carries it.

  It takes `socket`, aliased `unix-socket`, or `tcp`. `server start` takes a list, so serving both at once is `comodoro server start socket tcp`.

  Only `set` reorders its arguments, as `comodoro set <SECONDS> [TRANSPORT]`, since an optional argument cannot stand before a required one.

- **BREAKING** Replaced the `--debug` and `--trace` flags with `--log-level <LEVEL>` and `--log-file <PATH>`, which the pimalaya-cli toolkit now provides.

  `--log-level` accepts `off`, `error`, `warn`, `info`, `debug` and `trace`, and overrides `RUST_LOG` when given.

- **BREAKING** Reworked the cargo features, dropping `std`, `timer` and `command`.

  `std` pulled no crate of its own, every layer above the timer required it, and shell hooks pull no extra crate. `vendored` now forwards to notify-rust.

  A build without `notify` refuses an account carrying notify hooks as it loads, naming the missing feature rather than failing the build.

- **BREAKING** Rewrote the library API, so nothing published before this release still resolves.

  The I/O-free coroutine layer is gone, the remaining items are domain-prefixed per the Pimalaya naming guidelines, and everything the CLI needs sits under one `cli` subtree.

  What this release keeps stable is the TOML and the wire shapes. The Rust API is documented on docs.rs and counts as new.

- Re-licensed the project from AGPL-3.0-or-later to dual MIT OR Apache-2.0.
- Moved to edition 2024 with a 1.89 minimum supported Rust version.

### Fixed

- Fixed the configuration paths ignoring what the documentation describes.

  `-c` now reads the `COMODORO_CONFIG` environment variable when given no path, and splits `:`-delimited paths into the base and the ones deep-merged on top.

- Fixed the errors raised when no account can be resolved saying nothing actionable.

  A missing configuration names the path it looked for and offers the wizard, or points at `comodoro configure` when no one is there to answer.

  A missing named account lists the ones the configuration does hold, and a missing default account names the two ways to pick one.

- Fixed a client connection blocking every other one.

  The server ran each connection to completion before accepting the next, so a long-lived client held it to itself. Each connection now gets its own thread.

- Fixed a panic in one request killing the daemon.

  A panic while holding the timer poisoned its lock, so every later request exited. The lock is now recovered, since a panic can leave the timer stale but never torn.

- Fixed shell hooks not going through a shell.

  A command given as a string was split on whitespace and executed directly, so a hook redirecting its output passed `>>` to echo as an argument.

  String commands now run through `/bin/sh -c` (`cmd /C` on Windows), and an array still executes directly.

- Fixed a duration set through `timer.set` lasting less than a second.

  It was written onto the current cycle, which the tick recomputes from the elapsed time, so the write was discarded within a second while the call reported success.

  A set now moves the timeline to the point leaving the requested duration remaining. Longer than the cycle is clamped to it, and setting a stopped timer does nothing.

- Fixed a paused timer being impossible to stop.

  `timer.stop` acted only on a running timer, so it reported success on a paused one and left it paused.

- Fixed a configuration of zero-length cycles panicking the server, by way of a modulo by zero.

  Such a timer now ticks and sets without effect, since no elapsed time can name a cycle in it.

- Fixed a timer completing its last loop in silence.

  `timer.ended`, `timer.stopped` and the `on-timer-stop` hook never fired on the one moment a `cycles-count` timer exists for, and the timer reported itself stopped mid-cycle.

  The completing tick now emits both events and resets, exactly as `timer.stop` does.

- Fixed a cycle boundary going unannounced between two cycles sharing a name.

  Boundaries were detected by comparing names, so a single cycle looping forever never announced a round. A boundary is now also recognised by the remaining duration going back up.

- Fixed the per-second tick reporting the previous second.

  Every `timer.running` notification lagged one tick, and a duration written by `timer.set` was announced again by the next one, twice within the same wall-clock second.

  A tick now reports what it just computed: `timer.running` inside a cycle, `timer.ended` then `timer.began` when crossing into another, nothing when nothing changed.

## [1.0.0] - 2026-02-11

### Changed

- Bumped major dependencies.
- Renamed cargo feature `hook-command` into `command`.
- Renamed cargo feature `hook-notify` into `notify`.
- Prefixed preset configs with `presets.`, see config.sample.toml.
- Used `dbus` instead of `zbus` for `notify-rust`.

### Fixed

- Fixed Windows build with `dbus`.

### Removed

- Removed `hooks` and `tcp` cargo features.

## [0.1.2] - 2024-02-03

### Fixed

- Prevented commands `manual` and `completion` to return an error when configuration file was not found.

## [0.1.1] - 2024-02-03

### Fixed

- Fixed unix release builds.

## [0.1.0] - 2024-02-03

### Added

- Added hook support for system notifications. A hook can now either execute a shell command, send a system notification or both.
- Added cargo feature `hook-command` to enable hook based on shell commands (enabled by default).
- Added cargo feature `hook-notify` to enable hook based on system notifications (enabled by default).
- Added config.sample.toml at <https://github.com/pimalaya/comodoro/blob/master/config.sample.toml>.

### Changed

- Moved top-level commands related to client to the `timer` subcommand.
- Improved configuration API:

  | Before                  | After                              |
  |-------------------------|------------------------------------|
  | `[example]`             | `[presets.example]`                |
  | `tcp-host`              | `tcp.host`                         |
  | `tcp-port`              | `tcp.port`                         |
  | `on-time-begin = "cmd"` | `hooks.on-timer-begin.cmd = "cmd"` |

  The main purpose is to improve error diagnostic line numbers, see <https://github.com/toml-rs/toml/issues/589>.

## [0.0.10] - 2023-10-09

### Changed

- Upgraded nixpkgs channel from `22.11` to `23.05`.
- Upgraded cargo dependencies.
- Improved documentations.

## [0.0.9] - 2023-06-24

### Added

- Added preset option `preset` to get preconfigured timer. Available options: `pomodoro`, `52/17`.
- Added preset option `cycles-count` to control how the timer loops. `0` means infinite, whereas any integer makes the timer stop automatically after n loops.
- Added preset option `timer-precision` to customize the timer format. Available options: `second`, `minute` (default), `hour`.

## [0.0.8] - 2023-05-18

### Changed

- Changed the aim of the project. The timer is not Pomodoro-specific anymore, it became generic (which allows you to turn it into a Pomodoro timer, or whatever).
- Changed hooks name from `timer-started-hook` to `on-timer-start` and so on.

## [0.0.7] - 2023-04-24

### Added

- Add `zip` archive to releases.

## [0.0.6] - 2023-04-21

### Changed

- Improved cross compilation.

## [0.0.5] - 2023-04-20

### Changed

- Replaced `pimalaya` by `pimalaya-pomodoro`.

### Removed

- Removed durations and hooks from `TcpConfig`, since they conflicted with the ones from the main config.

## [0.0.4] - 2023-04-14

### Fixed

- Fixed hooks not triggered properly.

## [0.0.3] - 2023-04-14

### Added

- Added hooks support (check https://docs.rs/comodoro/0.0.3/comodoro/config/struct.HooksConfig.html for the list of available hooks).

### Changed

- Improve the way the timer is displayed via the `get` command.

## [0.0.2] - 2023-04-10

### Changed

- Rewrote the project in Rust, using the [Pimalaya](https://git.sr.ht/~soywod/pimalaya) library.

## [0.0.1] - 2020-12-15

### Added

- Added installation script.
- Added `.mli` files [#2].

### Changed

- Improved README [#8].
- Made timers customizable [#4].

### Fixed

- Fixed OS specific `tmp` and `xdg` dirs [#3].
- Fixed missing CI artifacts from releases [#6].

[#2]: https://github.com/pimalaya/comodoro/issues/2
[#3]: https://github.com/pimalaya/comodoro/issues/3
[#4]: https://github.com/pimalaya/comodoro/issues/4
[#6]: https://github.com/pimalaya/comodoro/issues/6
[#8]: https://github.com/pimalaya/comodoro/issues/8

[2.0.0]: https://github.com/pimalaya/comodoro/compare/v1.0.0...v2.0.0
[1.0.0]: https://github.com/pimalaya/comodoro/compare/v0.1.2...v1.0.0
[0.1.2]: https://github.com/pimalaya/comodoro/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/pimalaya/comodoro/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/pimalaya/comodoro/compare/v0.0.10...v0.1.0
[0.0.10]: https://github.com/pimalaya/comodoro/compare/v0.0.9...v0.0.10
[0.0.9]: https://github.com/pimalaya/comodoro/compare/v0.0.8...v0.0.9
[0.0.8]: https://github.com/pimalaya/comodoro/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/pimalaya/comodoro/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/pimalaya/comodoro/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/pimalaya/comodoro/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/pimalaya/comodoro/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/pimalaya/comodoro/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/pimalaya/comodoro/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/pimalaya/comodoro/releases/tag/v0.0.1
