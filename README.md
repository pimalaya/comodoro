# ⏳ Comodoro [![Documentation](https://img.shields.io/docsrs/comodoro?style=flat&logo=docs.rs&logoColor=white)](https://docs.rs/comodoro/latest/comodoro) [![Matrix](https://img.shields.io/badge/chat-%23pimalaya-blue?style=flat&logo=matrix&logoColor=white)](https://matrix.to/#/#pimalaya:matrix.org) [![Mastodon](https://img.shields.io/badge/news-%40pimalaya-blue?style=flat&logo=mastodon&logoColor=white)](https://fosstodon.org/@pimalaya)

CLI to manage timers

One server owns a timer, any number of clients drive it and watch it. This project is composed of 3 feature-gated layers:

- Low-level **contract**: the pure timer state machine and the JSON-RPC 2.0 method surface, no_std-compatible and free of any I/O
- Mid-level **client and server**: a blocking client over one connection, and a server owning the timer behind the listeners it binds
- High-level **CLI**: the client and the server behind a flat command grammar, a TOML configuration and per-event hooks

## Table of contents

- [Features](#features)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [AI policy](https://github.com/pimalaya/.github/blob/master/AI_POLICY.md)
- [License](#license)
- [Social](#social)
- [Contributing](./CONTRIBUTING.md)
- [Sponsoring](#sponsoring)

## Features

- **Shared timer**: one server owns it, and any number of clients start, pause, resume, stop or query it concurrently.
- **Push notifications**: subscribe once with `comodoro watch` and the server pushes every change, so a status bar never polls.
- **Local socket and TCP**: a server binds either transport or both at once, and every command picks the one it talks over.
- **Standard protocol**: plain [JSON-RPC 2.0](https://www.jsonrpc.org/specification), so any language can drive the timer.
- **Pomodoro-style cycles**: any sequence of named cycles and durations, looping forever or a fixed number of times.
- **Per-event hooks**: run a shell command or send a desktop notification when a cycle begins, ticks, is set, pauses, resumes or ends.
- **Status-bar friendly output**: the remaining duration renders at second, minute or hour precision, and `--json` emits the raw timer for scripts.

> [!TIP]
> Comodoro is written in [Rust](https://www.rust-lang.org/) and uses [cargo features](https://doc.rust-lang.org/cargo/reference/features.html) to gate its layers. The default feature set is declared in [Cargo.toml](./Cargo.toml).

## Installation

### Pre-built binary

Comodoro can be installed with the install.sh installer:

*As root:*

```sh
curl -sSL https://raw.githubusercontent.com/pimalaya/comodoro/master/install.sh | sudo sh
```

*As a regular user:*

```sh
curl -sSL https://raw.githubusercontent.com/pimalaya/comodoro/master/install.sh | PREFIX=~/.local sh
```

These commands install the latest binary from the GitHub [releases](https://github.com/pimalaya/comodoro/releases) section.

For a more up-to-date version than the latest release, check out the [releases](https://github.com/pimalaya/comodoro/actions/workflows/releases.yml) GitHub workflow and look for the *Artifacts* section. These pre-built binaries are built from the `master` branch.

> [!NOTE]
> Such binaries are built with the default cargo features. If you need specific features, please use another installation method.

### Cargo

```sh
cargo install comodoro --locked
```

For a more up-to-date version than the latest release:

```sh
cargo install --locked --git https://github.com/pimalaya/comodoro.git
```

Without desktop notifications, which drops the D-Bus system dependency:

```sh
cargo install comodoro --locked \
  --no-default-features \
  --features cli
```

### Nix

If you have the [Flakes](https://nixos.wiki/wiki/Flakes) feature enabled:

```sh
nix profile install github:pimalaya/comodoro
```

Or run without installing:

```sh
nix run github:pimalaya/comodoro
```

### Sources

```sh
git clone https://github.com/pimalaya/comodoro
cd comodoro
nix run
```

## Configuration

Run `comodoro` with no command on a machine that has no configuration: it offers to generate a first account, asking for a cycle preset and for the endpoints to serve the timer over, then either saves it, appends it to the configuration already there, or prints it for you to place. `comodoro configure` runs the same wizard on demand, and any command needing an account offers it too, then carries on. Everything beyond those presets is written by hand. A configuration is loaded from the first valid path among:

- `$XDG_CONFIG_HOME/comodoro/config.toml`
- `$HOME/.config/comodoro/config.toml`
- `$HOME/.comodororc`

Override the path with -c <PATH> or COMODORO_CONFIG=<PATH>. Multiple paths can be passed at once, separated by :. The first one is the base and the rest are deep-merged on top. The full field reference lives in [config.sample.toml](./config.sample.toml).

An account only needs its `cycles`, the ordered steps the timer runs through. Everything else has a default: a local socket under `$XDG_RUNTIME_DIR`, an endless loop, and a display precision of one minute. Give each account its own `socket.path` to run several timers side by side.

Add a `tcp` table to also reach the timer over the network. A server binds every transport its account configures, and a command talks over the one flagged `default`, falling back to the socket. That listener is unauthenticated, so whoever reaches the port drives the timer, and it stays on loopback unless you mean otherwise.

Bind a `command` or a desktop `notify` block to any timer event to run something when a cycle begins, ticks, is set, pauses, resumes or ends, and when the timer itself starts or stops. A hook that fails is logged and never stops the timer.

## Usage

Generate a first account, unless you wrote the configuration by hand already:

```sh
comodoro
```

Start the server, which owns the timer and stays in the foreground:

```sh
comodoro server start
```

Then drive the timer from anywhere:

```sh
comodoro start
comodoro get
comodoro pause
comodoro resume
comodoro set 300
comodoro stop
```

Feed a status bar without polling, which prints the timer once and then on every change until interrupted:

```sh
comodoro watch
```

Every command takes an optional transport, `socket` or `tcp`, and falls back to the one the configuration marks as default. The server takes the list of transports to bind, and binds every configured one when given none:

```sh
comodoro server start socket tcp
comodoro get tcp
```

Every command and every flag is documented behind `--help`. The library API is documented on [docs.rs](https://docs.rs/comodoro/latest/comodoro), and complete runnable programs live in [./examples](./examples).

Logs go to stderr, so they can be redirected to a file while the command output stays on stdout:

```sh
comodoro server start --log-level debug 2>/tmp/comodoro.log
```

Use `--log-file <PATH>` to append them to a file directly. When `--log-level` is omitted the `RUST_LOG` environment variable is consulted, and `RUST_BACKTRACE=1` adds the full error backtrace.

## License

This project is licensed under either of:

- [MIT license](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

## Social

- Chat on [Matrix](https://matrix.to/#/#pimalaya:matrix.org)
- News on [Mastodon](https://fosstodon.org/@pimalaya) or [RSS](https://fosstodon.org/@pimalaya.rss)
- Mail at [pimalaya.org@posteo.net](mailto:pimalaya.org@posteo.net)

## Sponsoring

[![nlnet](https://nlnet.nl/logo/banner-160x60.png)](https://nlnet.nl/)

Special thanks to the [NLnet foundation](https://nlnet.nl/) and the [European Commission](https://www.ngi.eu/) that have been financially supporting the project for years:

- 2022 → 2023: [NGI Assure](https://nlnet.nl/project/Himalaya/)
- 2023 → 2024: [NGI Zero Entrust](https://nlnet.nl/project/Pimalaya/)
- 2024 → 2026: [NGI Zero Core](https://nlnet.nl/project/Pimalaya-PIM/)
- 2026 → 2027: [NGI Zero Commons Fund](https://nlnet.nl/project/Pimalaya-pimdir/)

If you appreciate the project, feel free to donate using one of the following providers:

[![GitHub](https://img.shields.io/badge/-GitHub%20Sponsors-fafbfc?logo=GitHub%20Sponsors)](https://github.com/sponsors/soywod)
[![Ko-fi](https://img.shields.io/badge/-Ko--fi-ff5e5a?logo=Ko-fi&logoColor=ffffff)](https://ko-fi.com/soywod)
[![Buy Me a Coffee](https://img.shields.io/badge/-Buy%20Me%20a%20Coffee-ffdd00?logo=Buy%20Me%20A%20Coffee&logoColor=000000)](https://www.buymeacoffee.com/soywod)
[![Liberapay](https://img.shields.io/badge/-Liberapay-f6c915?logo=Liberapay&logoColor=222222)](https://liberapay.com/soywod)
[![thanks.dev](https://img.shields.io/badge/-thanks.dev-000000?logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQuMDk3IiBoZWlnaHQ9IjE3LjU5NyIgY2xhc3M9InctMzYgbWwtMiBsZzpteC0wIHByaW50Om14LTAgcHJpbnQ6aW52ZXJ0IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxwYXRoIGQ9Ik05Ljc4MyAxNy41OTdINy4zOThjLTEuMTY4IDAtMi4wOTItLjI5Ny0yLjc3My0uODktLjY4LS41OTMtMS4wMi0xLjQ2Mi0xLjAyLTIuNjA2di0xLjM0NmMwLTEuMDE4LS4yMjctMS43NS0uNjc4LTIuMTk1LS40NTItLjQ0Ni0xLjIzMi0uNjY5LTIuMzQtLjY2OUgwVjcuNzA1aC41ODdjMS4xMDggMCAxLjg4OC0uMjIyIDIuMzQtLjY2OC40NTEtLjQ0Ni42NzctMS4xNzcuNjc3LTIuMTk1VjMuNDk2YzAtMS4xNDQuMzQtMi4wMTMgMS4wMjEtMi42MDZDNS4zMDUuMjk3IDYuMjMgMCA3LjM5OCAwaDIuMzg1djEuOTg3aC0uOTg1Yy0uMzYxIDAtLjY4OC4wMjctLjk4LjA4MmExLjcxOSAxLjcxOSAwIDAgMC0uNzM2LjMwN2MtLjIwNS4xNTYtLjM1OC4zODQtLjQ2LjY4Mi0uMTAzLjI5OC0uMTU0LjY4Mi0uMTU0IDEuMTUxVjUuMjNjMCAuODY3LS4yNDkgMS41ODYtLjc0NSAyLjE1NS0uNDk3LjU2OS0xLjE1OCAxLjAwNC0xLjk4MyAxLjMwNXYuMjE3Yy44MjUuMyAxLjQ4Ni43MzYgMS45ODMgMS4zMDUuNDk2LjU3Ljc0NSAxLjI4Ny43NDUgMi4xNTR2MS4wMjFjMCAuNDcuMDUxLjg1NC4xNTMgMS4xNTIuMTAzLjI5OC4yNTYuNTI1LjQ2MS42ODIuMTkzLjE1Ny40MzcuMjYuNzMyLjMxMi4yOTUuMDUuNjIzLjA3Ni45ODQuMDc2aC45ODVabTE0LjMxNC03LjcwNmgtLjU4OGMtMS4xMDggMC0xLjg4OC4yMjMtMi4zNC42NjktLjQ1LjQ0NS0uNjc3IDEuMTc3LS42NzcgMi4xOTVWMTQuMWMwIDEuMTQ0LS4zNCAyLjAxMy0xLjAyIDIuNjA2LS42OC41OTMtMS42MDUuODktMi43NzQuODloLTIuMzg0di0xLjk4OGguOTg0Yy4zNjIgMCAuNjg4LS4wMjcuOTgtLjA4LjI5Mi0uMDU1LjUzOC0uMTU3LjczNy0uMzA4LjIwNC0uMTU3LjM1OC0uMzg0LjQ2LS42ODIuMTAzLS4yOTguMTU0LS42ODIuMTU0LTEuMTUydi0xLjAyYzAtLjg2OC4yNDgtMS41ODYuNzQ1LTIuMTU1LjQ5Ny0uNTcgMS4xNTgtMS4wMDQgMS45ODMtMS4zMDV2LS4yMTdjLS44MjUtLjMwMS0xLjQ4Ni0uNzM2LTEuOTgzLTEuMzA1LS40OTctLjU3LS43NDUtMS4yODgtLjc0NS0yLjE1NXYtMS4wMmMwLS40Ny0uMDUxLS44NTQtLjE1NC0xLjE1Mi0uMTAyLS4yOTgtLjI1Ni0uNTI2LS40Ni0uNjgyYTEuNzE5IDEuNzE5IDAgMCAwLS43MzctLjMwNyA1LjM5NSA1LjM5NSAwIDAgMC0uOTgtLjA4MmgtLjk4NFYwaDIuMzg0YzEuMTY5IDAgMi4wOTMuMjk3IDIuNzc0Ljg5LjY4LjU5MyAxLjAyIDEuNDYyIDEuMDIgMi42MDZ2MS4zNDZjMCAxLjAxOC4yMjYgMS43NS42NzggMi4xOTUuNDUxLjQ0NiAxLjIzMS42NjggMi4zNC42NjhoLjU4N3oiIGZpbGw9IiNmZmYiLz48L3N2Zz4=)](https://thanks.dev/soywod)
[![PayPal](https://img.shields.io/badge/-PayPal-0079c1?logo=PayPal&logoColor=ffffff)](https://www.paypal.com/paypalme/soywod)
