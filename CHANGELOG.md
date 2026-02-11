# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Bumped major dependencies.
- Renamed cargo feature `hook-command` into `command`.
- Renamed cargo feature `hook-notify` into `notify`.
- Prefixed preset configs with `presets.`, see `./config.sample.toml`.

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
- Added `config.sample.toml` at <https://github.com/pimalaya/comodoro/blob/master/config.sample.toml>.

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

[Unreleased]: https://github.com/pimalaya/comodoro/compare/v0.1.2...master
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
