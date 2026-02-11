# ⏱ Comodoro [![Releases](https://img.shields.io/github/v/release/pimalaya/comodoro?color=success)](https://github.com/pimalaya/comodoro/releases/latest) [![Repology](https://img.shields.io/repology/repositories/comodoro?color=success)]("https://repology.org/project/comodoro/versions) [![Matrix](https://img.shields.io/badge/chat-%23pimalaya-blue?style=flat&logo=matrix&logoColor=white)](https://matrix.to/#/#pimalaya:matrix.org) [![Mastodon](https://img.shields.io/badge/news-%40pimalaya-blue?style=flat&logo=mastodon&logoColor=white)](https://fosstodon.org/@pimalaya)

CLI to manage timers

## Table of contents

- [Features](#features)
- [Installation](#installation)
  - [Pre-built binary](#pre-built-binary)
  - [Cargo](#cargo)
  - [Nix](#nix)
  - [Sources](#sources)
- [Configuration](#configuration)
- [FAQ](#faq)
- [Social](#social)
- [Sponsoring](#sponsoring)

## Features

- Centralized server timer controllable by multiple clients at the same time
- **Multi protocols** (*Unix sockets* and *TCP* only supported for now)
- Cycles customizable via config file (**Pomodoro** style, **52/17** style, custom)
- Server and timer hooks customizable via config file (send system notification or run shell command)

*Comodoro CLI is written in [Rust](https://www.rust-lang.org/), and relies on [cargo features](https://doc.rust-lang.org/cargo/reference/features.html) to enable or disable functionalities. Default features can be found in the `features` section of the [`Cargo.toml`](./Cargo.toml#L18), or on [docs.rs](https://docs.rs/crate/comodoro/latest/features).*

## Installation

### Pre-built binary

Comodoro CLI can be installed with the `install.sh` installer:

*As root:*

```
curl -sSL https://raw.githubusercontent.com/pimalaya/comodoro/master/install.sh | sudo sh
```

*As a regular user:*

```
curl -sSL https://raw.githubusercontent.com/pimalaya/comodoro/master/install.sh | PREFIX=~/.local sh
```

These commands install the latest binary from the GitHub [releases](https://github.com/pimalaya/comodoro/releases) section.

If you want a more up-to-date version than the latest release, check out the [releases](https://github.com/pimalaya/comodoro/actions/workflows/releases.yml) GitHub workflow and look for the *Artifacts* section. You will find a pre-built binary matching your OS. These pre-built binaries are built from the `master` branch.

*Such binaries are built with the default cargo features. If you need more features, please use another installation method.*

### Cargo

Comodoro CLI can be installed with [cargo](https://doc.rust-lang.org/cargo/):

```
cargo install comodoro --locked
```

With only server support:

```
cargo install comodoro --locked --no-default-features --features server
```

You can also use the git repository for a more up-to-date (but less stable) version:

```
cargo install --locked --git https://github.com/pimalaya/comodoro.git
```

### Nix

Comodoro CLI can be installed with [Nix](https://serokell.io/blog/what-is-nix):

```
nix-env -i comodoro
```

You can also use the git repository for a more up-to-date (but less stable) version:

```
nix-env -if https://github.com/pimalaya/comodoro/archive/master.tar.gz
```

*Or, from within the source tree checkout:*

```
nix-env -if .
```

If you have the [Flakes](https://nixos.wiki/wiki/Flakes) feature enabled:

```
nix profile install comodoro
```

*Or, from within the source tree checkout:*

```
nix profile install
```

*You can also run Comodoro directly without installing it:*

```
nix run comodoro
```

### Sources

Comodoro CLI can be installed from sources.

First you need to install the Rust development environment (see the [rust installation documentation](https://doc.rust-lang.org/cargo/getting-started/installation.html)):

```
curl https://sh.rustup.rs -sSf | sh
```

Then, you need to clone the repository and install dependencies:

```
git clone https://github.com/pimalaya/comodoro.git
cd comodoro
cargo check
```

Now, you can build Comodoro:

```
cargo build --release
```

*Binaries are available under the `target/release` folder.*

## Configuration

The wizard is not yet available (it should come soon), meanwhile you can manually edit your own configuration from scratch:

- Copy the content of the documented [`./config.sample.toml`](./config.sample.toml)
- Paste it into a new file `~/.config/comodoro/config.toml`
- Edit, then comment or uncomment the options you want

## FAQ

### How to debug Comodoro CLI?

The simplest way is to use `--debug` and/or `--trace` arguments.

The advanced way is based on environment variables:

- `RUST_LOG=<level>`: determines the log level filter, can be one of `off`, `error`, `warn`, `info`, `debug` and `trace`.
- `RUST_BACKTRACE=1`: enables the full error backtrace, which include source lines where the error originated from.

Logs are written to the `stderr`, which means that you can redirect them easily to a file:

```
comodoro server start --debug 2>/tmp/comodoro.log
```

## Social

- Chat on [Matrix](https://matrix.to/#/#pimalaya:matrix.org)
- News on [Mastodon](https://fosstodon.org/@pimalaya) or [RSS](https://fosstodon.org/@pimalaya.rss)
- Mail at [pimalaya.org@posteo.net](mailto:pimalaya.org@posteo.net)

## Sponsoring

[![nlnet](https://nlnet.nl/logo/banner-160x60.png)](https://nlnet.nl/)

Special thanks to the [NLnet foundation](https://nlnet.nl/) and the [European Commission](https://www.ngi.eu/) that have been financially supporting the project for years:

- 2022: [NGI Assure](https://nlnet.nl/project/Himalaya/)
- 2023: [NGI Zero Entrust](https://nlnet.nl/project/Pimalaya/)
- 2024: [NGI Zero Core](https://nlnet.nl/project/Pimalaya-PIM/) *(still ongoing in 2026)*

If you appreciate the project, feel free to donate using one of the following providers:

[![GitHub](https://img.shields.io/badge/-GitHub%20Sponsors-fafbfc?logo=GitHub%20Sponsors)](https://github.com/sponsors/soywod)
[![Ko-fi](https://img.shields.io/badge/-Ko--fi-ff5e5a?logo=Ko-fi&logoColor=ffffff)](https://ko-fi.com/soywod)
[![Buy Me a Coffee](https://img.shields.io/badge/-Buy%20Me%20a%20Coffee-ffdd00?logo=Buy%20Me%20A%20Coffee&logoColor=000000)](https://www.buymeacoffee.com/soywod)
[![Liberapay](https://img.shields.io/badge/-Liberapay-f6c915?logo=Liberapay&logoColor=222222)](https://liberapay.com/soywod)
[![thanks.dev](https://img.shields.io/badge/-thanks.dev-000000?logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQuMDk3IiBoZWlnaHQ9IjE3LjU5NyIgY2xhc3M9InctMzYgbWwtMiBsZzpteC0wIHByaW50Om14LTAgcHJpbnQ6aW52ZXJ0IiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciPjxwYXRoIGQ9Ik05Ljc4MyAxNy41OTdINy4zOThjLTEuMTY4IDAtMi4wOTItLjI5Ny0yLjc3My0uODktLjY4LS41OTMtMS4wMi0xLjQ2Mi0xLjAyLTIuNjA2di0xLjM0NmMwLTEuMDE4LS4yMjctMS43NS0uNjc4LTIuMTk1LS40NTItLjQ0Ni0xLjIzMi0uNjY5LTIuMzQtLjY2OUgwVjcuNzA1aC41ODdjMS4xMDggMCAxLjg4OC0uMjIyIDIuMzQtLjY2OC40NTEtLjQ0Ni42NzctMS4xNzcuNjc3LTIuMTk1VjMuNDk2YzAtMS4xNDQuMzQtMi4wMTMgMS4wMjEtMi42MDZDNS4zMDUuMjk3IDYuMjMgMCA3LjM5OCAwaDIuMzg1djEuOTg3aC0uOTg1Yy0uMzYxIDAtLjY4OC4wMjctLjk4LjA4MmExLjcxOSAxLjcxOSAwIDAgMC0uNzM2LjMwN2MtLjIwNS4xNTYtLjM1OC4zODQtLjQ2LjY4Mi0uMTAzLjI5OC0uMTU0LjY4Mi0uMTU0IDEuMTUxVjUuMjNjMCAuODY3LS4yNDkgMS41ODYtLjc0NSAyLjE1NS0uNDk3LjU2OS0xLjE1OCAxLjAwNC0xLjk4MyAxLjMwNXYuMjE3Yy44MjUuMyAxLjQ4Ni43MzYgMS45ODMgMS4zMDUuNDk2LjU3Ljc0NSAxLjI4Ny43NDUgMi4xNTR2MS4wMjFjMCAuNDcuMDUxLjg1NC4xNTMgMS4xNTIuMTAzLjI5OC4yNTYuNTI1LjQ2MS42ODIuMTkzLjE1Ny40MzcuMjYuNzMyLjMxMi4yOTUuMDUuNjIzLjA3Ni45ODQuMDc2aC45ODVabTE0LjMxNC03LjcwNmgtLjU4OGMtMS4xMDggMC0xLjg4OC4yMjMtMi4zNC42NjktLjQ1LjQ0NS0uNjc3IDEuMTc3LS42NzcgMi4xOTVWMTQuMWMwIDEuMTQ0LS4zNCAyLjAxMy0xLjAyIDIuNjA2LS42OC41OTMtMS42MDUuODktMi43NzQuODloLTIuMzg0di0xLjk4OGguOTg0Yy4zNjIgMCAuNjg4LS4wMjcuOTgtLjA4LjI5Mi0uMDU1LjUzOC0uMTU3LjczNy0uMzA4LjIwNC0uMTU3LjM1OC0uMzg0LjQ2LS42ODIuMTAzLS4yOTguMTU0LS42ODIuMTU0LTEuMTUydi0xLjAyYzAtLjg2OC4yNDgtMS41ODYuNzQ1LTIuMTU1LjQ5Ny0uNTcgMS4xNTgtMS4wMDQgMS45ODMtMS4zMDV2LS4yMTdjLS44MjUtLjMwMS0xLjQ4Ni0uNzM2LTEuOTgzLTEuMzA1LS40OTctLjU3LS43NDUtMS4yODgtLjc0NS0yLjE1NXYtMS4wMmMwLS40Ny0uMDUxLS44NTQtLjE1NC0xLjE1Mi0uMTAyLS4yOTgtLjI1Ni0uNTI2LS40Ni0uNjgyYTEuNzE5IDEuNzE5IDAgMCAwLS43MzctLjMwNyA1LjM5NSA1LjM5NSAwIDAgMC0uOTgtLjA4MmgtLjk4NFYwaDIuMzg0YzEuMTY5IDAgMi4wOTMuMjk3IDIuNzc0Ljg5LjY4LjU5MyAxLjAyIDEuNDYyIDEuMDIgMi42MDZ2MS4zNDZjMCAxLjAxOC4yMjYgMS43NS42NzggMi4xOTUuNDUxLjQ0NiAxLjIzMS42NjggMi4zNC42NjhoLjU4N3oiIGZpbGw9IiNmZmYiLz48L3N2Zz4=)](https://thanks.dev/soywod)
[![PayPal](https://img.shields.io/badge/-PayPal-0079c1?logo=PayPal&logoColor=ffffff)](https://www.paypal.com/paypalme/soywod)
