# ⏱ Comodoro [![Documentation](https://img.shields.io/docsrs/comodoro?style=flat&logo=docs.rs&logoColor=white)](https://docs.rs/comodoro/latest/comodoro) [![Matrix](https://img.shields.io/badge/chat-%23pimalaya-blue?style=flat&logo=matrix&logoColor=white)](https://matrix.to/#/#pimalaya:matrix.org) [![Mastodon](https://img.shields.io/badge/news-%40pimalaya-blue?style=flat&logo=mastodon&logoColor=white)](https://fosstodon.org/@pimalaya)

Library and CLI to manage timers

One server owns a timer, any number of clients drive it and watch it. This project is composed of 3 feature-gated layers:

- Low-level **contract**: the pure timer state machine and the JSON-RPC 2.0 method surface, no_std-compatible and free of any I/O
- Mid-level **client and server**: a blocking client over a local socket, and a server owning the timer behind its listener
- High-level **CLI**: the client and the server behind a flat command grammar, a TOML configuration and per-event hooks

## Table of contents

- [Features](#features)
- [Specification coverage](#specification-coverage)
- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [AI policy](https://github.com/pimalaya/.github/blob/master/AI_POLICY.md)
- [License](#license)
- [Social](#social)
- [Contributing](./CONTRIBUTING.md)
- [Sponsoring](#sponsoring)

## Features

- **Pomodoro-style cycles**: any sequence of named cycles and durations, looping forever or a fixed number of times.
- **Shared timer**: one server owns it, and any number of clients start, pause, resume, stop or query it concurrently.
- **Push notifications**: subscribe once with `comodoro watch` and the server pushes every change, so a status bar never polls.
- **Standard protocol**: plain [JSON-RPC 2.0](https://www.jsonrpc.org/specification) over a local socket or over TCP, so any language can drive the timer.
- **Per-event hooks**: run a shell command or send a desktop notification when a cycle begins, ticks, is set, pauses, resumes or ends.
- **Status-bar friendly output**: the remaining duration renders at second, minute or hour precision, and `--json` emits the raw timer for scripts.

> [!TIP]
> Comodoro is written in [Rust](https://www.rust-lang.org/) and uses [cargo features](https://doc.rust-lang.org/cargo/reference/features.html) to gate its layers. The default feature set is declared in [Cargo.toml](./Cargo.toml).

## Specification coverage

| Specification | What is covered |
|---------------|-----------------|
| [JSON-RPC 2.0] | The whole envelope: requests, notifications, batches, and the standard error codes |

[JSON-RPC 2.0]: https://www.jsonrpc.org/specification

Clients and servers exchange those messages over a Unix domain socket or over TCP, one compact JSON value per line. The specification defines the payload and leaves the transport alone, which is the whole point: anything that can open a connection and write a line can drive the timer, in any language.

The server answers these methods:

| Method | Parameters | Result |
|--------|------------|--------|
| `timer.get` | none | the timer |
| `timer.start` | none | the events it emitted |
| `timer.pause` | none | the events it emitted |
| `timer.resume` | none | the events it emitted |
| `timer.stop` | none | the events it emitted |
| `timer.set` | `duration` in seconds | the events it emitted |
| `timer.subscribe` | none | whether the connection is subscribed |
| `timer.unsubscribe` | none | whether the connection is subscribed |

A subscribed connection additionally receives a notification every time the timer changes: `timer.started`, `timer.began`, `timer.running`, `timer.durationSet`, `timer.paused`, `timer.resumed`, `timer.ended` and `timer.stopped`. Each one carries the cycle it concerns, except the two timer-wide ones which carry nothing. Requests are named after the imperative that performs them and notifications after the past tense of what happened, so the two directions never collide.

Driving the timer by hand takes no client at all:

```sh
echo '{"jsonrpc":"2.0","method":"timer.start","id":1}' | socat - UNIX-CONNECT:$XDG_RUNTIME_DIR/comodoro.sock
```

Failures come back as the standard error codes: -32700 for unparsable JSON, -32600 for a malformed request, -32601 for an unknown method, -32602 for bad parameters and -32603 for an internal failure. The -32000 to -32099 range is reserved for future Comodoro errors and currently unused.

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

If you want a more up-to-date version than the latest release, check out the [releases](https://github.com/pimalaya/comodoro/actions/workflows/releases.yml) GitHub workflow and look for the *Artifacts* section. You will find a pre-built binary matching your OS. These pre-built binaries are built from the master branch.

*Such binaries are built with the default cargo features. If you need more features, please use another installation method.*

### Cargo

Comodoro can be installed with [cargo](https://doc.rust-lang.org/cargo/):

```sh
cargo install comodoro --locked
```

You can also use the git repository for a more up-to-date (but less stable) version:

```sh
cargo install --locked --git https://github.com/pimalaya/comodoro.git
```

### Nix

Comodoro can be installed with [Nix](https://serokell.io/blog/what-is-nix):

```sh
nix-env -i comodoro
```

You can also use the git repository for a more up-to-date (but less stable) version:

```sh
nix-env -if https://github.com/pimalaya/comodoro/archive/master.tar.gz
```

*Or, from within the source tree checkout:*

```sh
nix-env -if .
```

If you have the [Flakes](https://nixos.wiki/wiki/Flakes) feature enabled:

```sh
nix profile install comodoro
```

*Or, from within the source tree checkout:*

```sh
nix profile install
```

*You can also run Comodoro directly without installing it:*

```sh
nix run comodoro
```

### Sources

Comodoro can be installed from sources. First you need to install the Rust development environment (see the [rust installation documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html)):

```sh
curl https://sh.rustup.rs -sSf | sh
```

Then, clone the repository and build:

```sh
git clone https://github.com/pimalaya/comodoro.git
cd comodoro
cargo build --release
```

*Binaries are available under the target/release folder.*

## Configuration

Comodoro ships no wizard: the configuration is written by hand. A configuration is loaded from the first valid path among:

- $XDG_CONFIG_HOME/comodoro/config.toml
- $HOME/.config/comodoro/config.toml
- $HOME/.comodororc

Override the path with -c <PATH> or COMODORO_CONFIG=<PATH>. Multiple paths can be passed at once, separated by :. The first one is the base and the rest are deep-merged on top. The full field reference lives in [config.sample.toml](./config.sample.toml).

An account only needs its `cycles`, the ordered steps the timer runs through. Everything else has a default: `socket.path` falls back to comodoro.sock inside `$XDG_RUNTIME_DIR` (or the platform temporary directory), `cycles-count` leaves the timer looping forever, and `precision` decides how the remaining duration renders. Set `socket.path` per account when running several timers at once.

Two transports carry the same protocol. The local socket is the one every account has, and is also spelled `unix-socket` for accounts written against Comodoro 1.x. TCP is added by a `tcp` table carrying a `port` and an optional `host`, defaulting to 127.0.0.1: an account without that table opens no port. That listener is unauthenticated, so whoever reaches the port drives the timer, which is why it stays on loopback unless you mean otherwise. When both are configured, `socket.default` or `tcp.default` decides which one a command reaches for.

Hooks are bound to events by name, `on-{cycle}-{event}` for a cycle and `on-timer-start` or `on-timer-stop` for the timer itself. A hook runs either a `command`, given as a shell line or as a program followed by its arguments, or a `notify` block carrying a summary and a body. Notifications require the `notify` cargo feature, enabled by default.

## Usage

Start the server, which owns the timer and stays in the foreground:

```sh
comodoro servers start
```

Then drive the timer from anywhere:

```sh
comodoro start
comodoro get
comodoro pause
comodoro resume
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
comodoro servers start --log-level debug 2>/tmp/comodoro.log
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
