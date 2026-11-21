# :timer_clock: Comodoro [![Matrix](https://img.shields.io/badge/chat-%23pimalaya-blue?style=flat&logo=matrix&logoColor=white)](https://matrix.to/#/#pimalaya:matrix.org) [![Mastodon](https://img.shields.io/badge/news-%40pimalaya-blue?style=flat&logo=mastodon&logoColor=white)](https://fosstodon.org/@pimalaya) [![Sponsor](https://img.shields.io/badge/sponsor-pink?style=flat&logo=github-sponsors&logoColor=white)](https://pimalaya.org/sponsor/)

CLI to manage timers

```sh
$ comodoro start --json
```

```json
{
  "schedule": {
    "cycles": [
      { "name": "Work", "duration": 1500 },
      { "name": "Rest", "duration": 300 }
    ],
    "loops": null
  },
  "state": "running",
  "cycle": { "name": "Work", "duration": 1500 },
  "startedAt": "2026-11-21T09:00:00Z",
  "elapsed": 0
}
```

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

- **Shared timer**: one server owns it, any number of clients drive it and watch it
- **Status bars without polling**: `comodoro watch` prints the timer on every change the server pushes
- **Local socket and TCP**: either transport, or both at once
- **Open protocol**: plain JSON-RPC 2.0, drivable from any language
- **Any cycles**: Pomodoro or any named durations, looping forever or a fixed number of times
- **Hooks**: a command, a desktop notification or both on any timer event, the cycle given as variables
- **REPL** running commands one per line, and **MCP server** serving the timer to an AI agent
- **JSON output** of every command, described by `comodoro json-schema`

## Installation

<details>
  <summary>Debian, Ubuntu</summary>

  Comodoro is not in the official repositories yet, but the signed APT repository of [pkg.pimalaya.org](https://pkg.pimalaya.org) ships the latest release with its man pages and shell completions. Add the repository once:

  ```sh
  sudo install -d -m 0755 /etc/apt/keyrings
  curl -fsSL https://pkg.pimalaya.org/pimalaya.asc | sudo tee /etc/apt/keyrings/pimalaya.asc >/dev/null
  sudo tee /etc/apt/sources.list.d/pimalaya.sources >/dev/null <<EOF
  Types: deb
  URIs: https://pkg.pimalaya.org/apt
  Suites: stable
  Components: main
  Signed-By: /etc/apt/keyrings/pimalaya.asc
  EOF
  sudo apt update
  ```

  Then install Comodoro:

  ```sh
  sudo apt install comodoro
  ```
</details>

<details>
  <summary>Fedora, RHEL, Rocky Linux, AlmaLinux</summary>

  Comodoro is not in the official repositories yet, but the signed DNF repository of [pkg.pimalaya.org](https://pkg.pimalaya.org) ships the latest release with its man pages and shell completions:

  ```sh
  sudo curl -fsSL -o /etc/yum.repos.d/pimalaya.repo https://pkg.pimalaya.org/pimalaya.repo
  sudo dnf install comodoro
  ```
</details>

<details>
  <summary>Homebrew (macOS, Linux)</summary>

  Comodoro is not in homebrew-core yet, but the [pimalaya/tap](https://github.com/pimalaya/homebrew-tap) formula installs the latest release with its man pages and shell completions:

  ```sh
  brew install pimalaya/tap/comodoro
  ```
</details>

<details>
  <summary>Pre-built binary</summary>

  For other systems, Comodoro can be installed with the installer:

  *As root:*

  ```sh
  curl -sSL https://raw.githubusercontent.com/pimalaya/comodoro/master/install.sh | sudo sh
  ```

  *As a regular user:*

  ```sh
  curl -sSL https://raw.githubusercontent.com/pimalaya/comodoro/master/install.sh | PREFIX=~/.local sh
  ```

  These commands install the latest [release](https://github.com/pimalaya/comodoro/releases), built with the default cargo features. For a build of `master`, check out the [releases](https://github.com/pimalaya/comodoro/actions/workflows/releases.yml) GitHub workflow and look for the *Artifacts* section.
</details>

<details>
  <summary>Cargo</summary>

  ```sh
  cargo install --locked comodoro
  ```

  For a build of `master`:

  ```sh
  cargo install --locked --git https://github.com/pimalaya/comodoro.git
  ```
</details>

<details>
  <summary>Nix</summary>

  ```sh
  nix profile install github:pimalaya/comodoro
  ```
</details>

<details>
  <summary>Sources</summary>

  Requires [Rust](https://www.rust-lang.org/tools/install) 1.89 or later:

  ```sh
  git clone https://github.com/pimalaya/comodoro
  cd comodoro
  cargo build --release --locked
  ```

  The binary lands at target/release/comodoro.
</details>

## Configuration

A TOML configuration is loaded from every existing path among the following, each one deep-merged on top of the ones before:

- `$XDG_CONFIG_HOME/comodoro/config.toml`
- `$HOME/.config/comodoro/config.toml`
- `$HOME/.comodororc`

Override the path with `-c <PATH>`, repeated to merge several files.

> [!TIP]
> See [config.sample.toml](./config.sample.toml) for a complete configuration of one example account with commented fields; rename the account and uncomment what you need.

An account only needs its `cycles`: uncomment one of the sample's presets (Pomodoro, 52/17 or 112/26). The TCP listener is unauthenticated, so keep it on loopback.

## Usage

**Step 1**: once your first account is configured, start the server, which owns the timer until a signal stops it:

```sh
comodoro server start
```

**Step 2**: from another terminal, check that the server answers on every transport of the account:

```sh
comodoro account check
```

**Step 3**: start the timer:

```sh
comodoro start
```

**Step 4**: follow it, for a status bar for example:

```sh
comodoro watch
```

Every command runs on the default account unless `-a` names another, and its `--help` details its flags, exit codes and JSON output. More use cases:

```sh
# Pause, resume or stop the timer
comodoro pause
comodoro resume
comodoro stop

# Leave five minutes in the current cycle
comodoro set 300

# Serve both transports, then read the timer over TCP
comodoro server start socket tcp
comodoro get tcp

# Feed a status bar with the cycle name and its remaining seconds
comodoro watch --json | jq -r --unbuffered 'select(.event == "timer-changed") | "\(.cycle.name) \(.cycle.duration)s"'

# Drive the timer from an AI agent
comodoro mcp --allow-write
```

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

This program is part of Pimalaya, free software funded entirely by grants and donations. If you find it useful, consider [sponsoring](https://pimalaya.org/sponsor/) its development:

[![GitHub](https://img.shields.io/badge/-GitHub%20Sponsors-fafbfc?logo=GitHub%20Sponsors)](https://github.com/sponsors/soywod)
[![Ko-fi](https://img.shields.io/badge/-Ko--fi-ff5e5a?logo=Ko-fi&logoColor=ffffff)](https://ko-fi.com/pimalaya)
[![Buy Me a Coffee](https://img.shields.io/badge/-Buy%20Me%20a%20Coffee-ffdd00?logo=Buy%20Me%20A%20Coffee&logoColor=000000)](https://www.buymeacoffee.com/pimalaya)
[![Liberapay](https://img.shields.io/badge/-Liberapay-f6c915?logo=Liberapay&logoColor=222222)](https://liberapay.com/pimalaya)
[![thanks.dev](https://img.shields.io/badge/-thanks.dev-000000?logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQuMDk3IiBoZWlnaHQ9IjE3LjU5NyIgY2xhc3M9InctMzYgbWwtMiBsZzpteC0wIHByaW50Om14LTAgcHJpbnQ6aW52ZXJ0IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxwYXRoIGQ9Ik05Ljc4MyAxNy41OTdINy4zOThjLTEuMTY4IDAtMi4wOTItLjI5Ny0yLjc3My0uODktLjY4LS41OTMtMS4wMi0xLjQ2Mi0xLjAyLTIuNjA2di0xLjM0NmMwLTEuMDE4LS4yMjctMS43NS0uNjc4LTIuMTk1LS40NTItLjQ0Ni0xLjIzMi0uNjY5LTIuMzQtLjY2OUgwVjcuNzA1aC41ODdjMS4xMDggMCAxLjg4OC0uMjIyIDIuMzQtLjY2OC40NTEtLjQ0Ni42NzctMS4xNzcuNjc3LTIuMTk1VjMuNDk2YzAtMS4xNDQuMzQtMi4wMTMgMS4wMjEtMi42MDZDNS4zMDUuMjk3IDYuMjMgMCA3LjM5OCAwaDIuMzg1djEuOTg3aC0uOTg1Yy0uMzYxIDAtLjY4OC4wMjctLjk4LjA4MmExLjcxOSAxLjcxOSAwIDAgMC0uNzM2LjMwN2MtLjIwNS4xNTYtLjM1OC4zODQtLjQ2LjY4Mi0uMTAzLjI5OC0uMTU0LjY4Mi0uMTU0IDEuMTUxVjUuMjNjMCAuODY3LS4yNDkgMS41ODYtLjc0NSAyLjE1NS0uNDk3LjU2OS0xLjE1OCAxLjAwNC0xLjk4MyAxLjMwNXYuMjE3Yy44MjUuMyAxLjQ4Ni43MzYgMS45ODMgMS4zMDUuNDk2LjU3Ljc0NSAxLjI4Ny43NDUgMi4xNTR2MS4wMjFjMCAuNDcuMDUxLjg1NC4xNTMgMS4xNTIuMTAzLjI5OC4yNTYuNTI1LjQ2MS42ODIuMTkzLjE1Ny40MzcuMjYuNzMyLjMxMi4yOTUuMDUuNjIzLjA3Ni45ODQuMDc2aC45ODVabTE0LjMxNC03LjcwNmgtLjU4OGMtMS4xMDggMC0xLjg4OC4yMjMtMi4zNC42NjktLjQ1LjQ0NS0uNjc3IDEuMTc3LS42NzcgMi4xOTVWMTQuMWMwIDEuMTQ0LS4zNCAyLjAxMy0xLjAyIDIuNjA2LS42OC41OTMtMS42MDUuODktMi43NzQuODloLTIuMzg0di0xLjk4OGguOTg0Yy4zNjIgMCAuNjg4LS4wMjcuOTgtLjA4LjI5Mi0uMDU1LjUzOC0uMTU3LjczNy0uMzA4LjIwNC0uMTU3LjM1OC0uMzg0LjQ2LS42ODIuMTAzLS4yOTguMTU0LS42ODIuMTU0LTEuMTUydi0xLjAyYzAtLjg2OC4yNDgtMS41ODYuNzQ1LTIuMTU1LjQ5Ny0uNTcgMS4xNTgtMS4wMDQgMS45ODMtMS4zMDV2LS4yMTdjLS44MjUtLjMwMS0xLjQ4Ni0uNzM2LTEuOTgzLTEuMzA1LS40OTctLjU3LS43NDUtMS4yODgtLjc0NS0yLjE1NXYtMS4wMmMwLS40Ny0uMDUxLS44NTQtLjE1NC0xLjE1Mi0uMTAyLS4yOTgtLjI1Ni0uNTI2LS40Ni0uNjgyYTEuNzE5IDEuNzE5IDAgMCAwLS43MzctLjMwNyA1LjM5NSA1LjM5NSAwIDAgMC0uOTgtLjA4MmgtLjk4NFYwaDIuMzg0YzEuMTY5IDAgMi4wOTMuMjk3IDIuNzc0Ljg5LjY4LjU5MyAxLjAyIDEuNDYyIDEuMDIgMi42MDZ2MS4zNDZjMCAxLjAxOC4yMjYgMS43NS42NzggMi4xOTUuNDUxLjQ0NiAxLjIzMS42NjggMi4zNC42NjhoLjU4N3oiIGZpbGw9IiNmZmYiLz48L3N2Zz4=)](https://thanks.dev/u/gh/soywod)
[![PayPal](https://img.shields.io/badge/-PayPal-0079c1?logo=PayPal&logoColor=ffffff)](https://www.paypal.com/paypalme/soywod)
