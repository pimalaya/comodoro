# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
