# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-08-14

### Added

- Turned Comodoro into a library as well as a binary, by merging the io-time crate into this repository.

  The crate now ships three layers: the timer state machine and the protocol types (always compiled, the only no_std layer), a blocking client and server (`client` and `server` features), and the CLI (`cli` feature, default).

- Adopted [JSON-RPC 2.0](https://www.jsonrpc.org/specification) as the wire protocol, framed as NDJSON over a local socket or over TCP.

  The specification defines the payload and leaves the transport alone, so the protocol is now implementable in any language without reading a line of Rust. The method surface lives in the `protocol` module and the envelope in `jsonrpc20`, both no_std. Failures come back as the standard error codes rather than as opaque strings.

- Added timer notifications, the capability the protocol change was for.

  A connection calling `timer.subscribe` receives `timer.started`, `timer.began`, `timer.running`, `timer.durationSet`, `timer.paused`, `timer.resumed`, `timer.ended` and `timer.stopped` as they happen, from any client's actions and from the tick loop.

- Added `TimerClient`, a blocking client over one connection, and `TimerServer`, which owns the timer, answers requests and fans notifications out to subscribers.

  Both live in a module named after the runtime they are written against, `client::std` and `server::std`, so an asynchronous port lands beside them rather than replacing them. A server holds a list of `TimerAddress`, so it serves the same timer over its socket and over TCP at once, and a client connected to either sees what the other does.

- Added the `watch` command, which subscribes and prints the timer on every change until interrupted, so a status bar no longer has to poll.
- Added the `set` command, exposing the `timer.set` method that was previously reachable on the wire but not from the CLI.
- Added the `configure` command, which generates an account from one of the documented cycle presets.

  It asks one question, the cycle preset, then names the account after it and saves it to the configuration file, appends it to the one already there, or prints it on stdout when the offer is declined, when stdout is redirected (`comodoro configure > config.toml`) or in JSON mode. A bare `comodoro`, which is what a newcomer runs first, and any command that needs an account both open with a welcome naming the configuration file they looked for when they find none, then offer the same wizard. The offer is a hook rather than a gate: a command runs afterwards either way, so accepting is what gives it a chance to work and declining leaves it to fail on the configuration it still has not got. A bare `comodoro` has nothing to run, so it shows the help, which is also what it shows when a configuration is already there. Appending is a plain text append, so the comments and the formatting already in the file come out untouched, and a name the configuration already holds is suffixed rather than reused, since two `[accounts.<name>]` tables make the whole document fail to parse. The generated account claims `default` only when no other account does. The account it writes holds its cycles and nothing else, since every other field has a default, so no address of this machine lands in a file that may be copied to another. Anything beyond the presets, meaning custom cycles, a transport address, the display precision and the hooks, is still written by hand against config.sample.toml.

- Added the `transport` module, holding `TimerAddress`, the `TimerStream` connection and the `TimerListener` accepting them, so both transports carry the protocol behind one type.
- Added the repository skeleton the Pimalaya guidelines require: a cairn/ folder with its AGENTS.md activation stanza, SECURITY.md, and the tests and audit CI workflows.

### Changed

- **BREAKING** Renamed `TimerConfig` to `TimerSchedule`, and its `cycles_count` field to `loops`.

  A CLI has a configuration of its own, and this was never it: the struct says what a timer runs, its ordered cycles and how many full loops of them. `loops` also stops the field from reading as a count of cycles, which is what `cycles_count` suggested and never meant. `Timer` carries it as `schedule`, and so does `TimerServer`. The account table keeps its `cycles-count` key, since a 1.x configuration still has to load.

- **BREAKING** Removed `Timer::cycles_count`, which duplicated the schedule's own loop count.

  It was written on creation, start and reset, always from the configured value, read once, and never decremented despite saying so. The completion check reads the schedule directly, so a timer now holds one loop count instead of two, and `timer.get` answers with one instead of the same number under two names.

- **BREAKING** Replaced the `TimerCycles` wrapper with the `Vec<TimerCycle>` it wrapped.

  It serialized transparently and dereferenced to the vector, so it was already a vector everywhere but in the type name, and the `From` impl it added is what `vec!` gives for free. `TimerSchedule { cycles }` now holds the vector itself, and no wire shape changes.

- **BREAKING** Renamed the `unix-socket` configuration table to `socket`, and gave every transport field a default, so a table adjusts an address rather than switching a transport on.

  A 1.x account file still loads, since `unix-socket` remains an accepted alias and both tables keep their fields. What changed is that none of them is required: `socket.path` defaults to comodoro.sock inside `$XDG_RUNTIME_DIR`, or inside the platform temporary directory when that variable is unset, `tcp.host` to 127.0.0.1 and `tcp.port` to 9999. An account therefore requires nothing but its `cycles`, and describes both transports at both default addresses. Windows reaches that same path-addressed socket through uds_windows, as the rest of the Pimalaya stack does.

  Which transport actually runs is no longer a configuration matter at all: `server start` binds what it is given, and the default transport alone when given none, so `comodoro server start` opens no port and `comodoro server start tcp` binds no socket. Which transport a client talks over stays what `socket.default` and `tcp.default` say, the socket winning when neither claims it.

- **BREAKING** Renamed the `[PROTOCOL]` argument to `[TRANSPORT]`, since a protocol is now what the two peers speak rather than what carries it.

  It takes `socket`, aliased `unix-socket`, or `tcp`, and still defaults to the transport the configuration marks as default. `server start` takes the list of transports to bind, and binds the default one when given none, so serving the same timer over both at once is `comodoro server start socket tcp`. Only `set` orders its arguments differently, as `comodoro set <SECONDS> [TRANSPORT]`, because an optional argument cannot stand before a required one.

- **BREAKING** Removed the I/O-free coroutine layer, roughly half the crate.

  The coroutines modelled the socket and the clock as resumable state machines, which is worth doing when several runtimes share the logic. Comodoro has exactly one runtime and will only ever have one, since the server is a blocking daemon and the client a short-lived process. The `coroutines`, `runtimes` and `io` modules are gone, along with `TimeInput`, `TimeOutput`, `TimeNow`, `TimeSleep`, `TimeSleepUntil`, `TimerStreamRead`, `TimerStreamWrite`, `TimerRequestSend` and `TimerRequestHandle`. What was worth keeping I/O-free is untouched: `Timer` still takes `now: u64` and reads no clock.

- **BREAKING** Reshaped `TimerRequest` and `TimerResponse` into the protocol method surface, and moved them from `timer` to `protocol`.

  `TimerRequest::Update` is gone, since the tick belongs to the server rather than to the wire, and `Subscribe` and `Unsubscribe` joined. `TimerEvent` now serializes adjacently, as `{"event": "began", "cycle": {…}}`.

- **BREAKING** Prefixed every library item with its domain, per the Pimalaya naming guidelines.

  `ServerSubcommand` became `TimerServerCommand` and `StartServerCommand` became `TimerServerStartCommand`. The `cli` module is the exception: `Cli`, `Command`, `Config`, `AccountConfig`, `Transport` and their neighbours carry no product prefix, since nothing under it is meant to be consumed as a library.

- **BREAKING** Moved everything the CLI needs under the `cli` module, so its cargo feature gates one subtree rather than a scattering of items.

  `config` became `cli::config`, `hooks` became `cli::hook`, the transport selection moved to `cli::transport`, and each command now has its own module: `cli::client::{get, start, pause, resume, stop, set, watch}` and `cli::server::start`. `cli::config` holds the TOML document alone, and the merged view every command runs against is `cli::account::Account`, resolved from it. What stays outside is what a library consumer can use without clap: `timer`, `protocol`, `jsonrpc20`, `transport`, `client` and `server`.

- **BREAKING** Replaced the io-hook, io-notify and io-process dependencies with an in-crate hook module.

  A hook is now a `TimerHook`, running either a `std::process::Command` deserialized through `pimalaya_config::command`, or a `TimerHookNotification` sent through notify-rust. The TOML shape is unchanged. Comodoro was the only consumer of those three crates, and the rest of the Pimalaya stack had already moved to calling notify-rust and the process API directly.

- **BREAKING** Reworked the cargo features, dropping `std`, `timer` and `command`.

  `std` pulled no crate of its own, and every layer above `timer` required it. Shell hooks no longer pull an extra crate either, so they ship unconditionally. `vendored` now forwards to notify-rust rather than to io-hook and io-notify.

- **BREAKING** Dropped `Clone` from the configuration types, since `std::process::Command` is not clonable.

- **BREAKING** Replaced the `--debug` and `--trace` flags with `--log-level <LEVEL>` and `--log-file <PATH>`, which the pimalaya-cli toolkit now provides.

  `--log-level` accepts `off`, `error`, `warn`, `info`, `debug` and `trace`, and overrides `RUST_LOG` when given.

- Re-licensed the project from AGPL-3.0-or-later to dual MIT OR Apache-2.0.
- Migrated from pimalaya-toolbox to the split pimalaya-cli and pimalaya-config stack, both consumed from crates.io, leaving the `[patch.crates-io]` table empty and the build free of any git dependency.
- Moved to edition 2024 with a 1.89 minimum supported Rust version.
- Made `#![no_std]` unconditional, and replaced the README rustdoc include with a proper architecture header in src/lib.rs.
- Rewrote the README, CONTRIBUTING.md and config.sample.toml against the Pimalaya documentation guidelines.

### Fixed

- Fixed the configuration paths ignoring what the documentation describes.

  `-c` now reads the `COMODORO_CONFIG` environment variable when it is given no path, and splits `:`-delimited paths into the base and the ones deep-merged on top, rather than taking the whole string as a single filename.

- Fixed the errors raised when no account can be resolved saying nothing actionable.

  A missing configuration now names the path it was looked for, which is where `-c` pointed or the default location, and offers the wizard, or points at `comodoro configure` when no one is there to answer, meaning a redirected stdin or JSON output. A missing named account lists the accounts the configuration does hold, and a missing default account names the two ways to pick one.

- Fixed a client connection blocking every other one.

  The accept loop served each connection to completion before accepting the next, so a long-lived client held the server to itself. Each connection now gets its own thread, which is also what makes subscriptions possible.

- Fixed a poisoned lock killing the daemon.

  A panic while holding the timer mutex made every later request call `process::exit(1)`. The guard is now recovered, since the timer is a plain struct that a panic can leave stale but never torn.

- Fixed shell hooks not going through a shell.

  A command given as a string was split on whitespace and executed directly, so the documented `hooks.on-work-begin.command = "echo 'Work started!' >> /tmp/comodoro.log"` appended nothing and passed `>>` to echo as an argument. String commands now run through `/bin/sh -c` (`cmd /C` on Windows), and a command given as an array still executes directly with no shell.

- Fixed a set duration lasting less than a second.

  `timer.set` wrote the remaining duration onto the current cycle, but the tick recomputes that cycle from the elapsed time, so the write was discarded within a second while the call still reported success.

  **BREAKING** `Timer::set` now takes `now: u64` and moves the elapsed time to the point that leaves the requested duration remaining, so every later tick recomputes the value it was given. A duration longer than the cycle's configured length is clamped to it, since a longer one would place the timeline inside the previous cycle, and the returned `set` event carries the effective value. On a stopped timer `set` became a no-op returning no events, as `start` resets the elapsed time anyway. The wire contract is unchanged.

- Fixed a paused timer being impossible to stop.

  `Timer::stop` acted only on a running timer, so `timer.stop` reported success on a paused one and left it paused.

- Fixed a configuration of zero-length cycles panicking the tick thread, by way of a modulo by zero.

  Such a timer now ticks and sets without effect, since no elapsed time can name a cycle in it.

- Fixed a timer completing its last loop in silence.

  `Timer::update` flipped the state to stopped and returned no events, so `timer.ended`, `timer.stopped` and the `on-timer-stop` hook never fired on the one moment a `cycles-count` timer exists for. It also left the cycle, the elapsed time and the start instant untouched, reporting a timer stopped in the middle of a cycle. The completing tick now emits `timer.ended` then `timer.stopped` and resets, exactly as `timer.stop` does.

- Fixed a cycle boundary going unannounced between two cycles sharing a name.

  Boundaries were detected by comparing names, so the simplest configuration there is, a single cycle looping forever, never announced a round. A boundary is now recognised by the remaining duration going back up as well as by the name changing, since remaining time only ever decreases inside a cycle.

- Fixed the per-second tick reporting the previous second.

  `Timer::update` pushed the cycle it was about to replace, so every `timer.running` notification lagged one tick, and a duration written by `timer.set` was announced again by the next tick, twice when that tick landed in the same wall-clock second.

  A tick now reports what it just computed. Staying inside a cycle emits `timer.running` with the current remaining duration, crossing into another emits `timer.ended` then `timer.began` and no `timer.running`, and changing nothing emits nothing. Two consecutive `timer.running` notifications therefore never carry the same duration, and `on-{cycle}-running` hooks no longer fire on the second a cycle ends.

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

[#2]: https://github.com/pimalaya/comodoro/issues/2
[#3]: https://github.com/pimalaya/comodoro/issues/3
[#4]: https://github.com/pimalaya/comodoro/issues/4
[#6]: https://github.com/pimalaya/comodoro/issues/6
[#8]: https://github.com/pimalaya/comodoro/issues/8
