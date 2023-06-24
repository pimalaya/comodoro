# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[Unreleased]: https://github.com/soywod/comodoro/compare/v0.0.8...master
[0.0.8]: https://github.com/soywod/comodoro/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/soywod/comodoro/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/soywod/comodoro/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/soywod/comodoro/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/soywod/comodoro/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/soywod/comodoro/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/soywod/comodoro/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/soywod/comodoro/releases/tag/v0.0.1

[#2]: https://github.com/soywod/comodoro/issues/2
[#3]: https://github.com/soywod/comodoro/issues/3
[#4]: https://github.com/soywod/comodoro/issues/4
[#6]: https://github.com/soywod/comodoro/issues/6
[#8]: https://github.com/soywod/comodoro/issues/8
