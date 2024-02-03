# ⏳ Comodoro [![GitHub release](https://img.shields.io/github/v/release/soywod/comodoro?color=success)](https://github.com/soywod/comodoro/releases/latest) [![Matrix](https://img.shields.io/matrix/pimalaya.comodoro:matrix.org?color=success&label=chat)](https://matrix.to/#/#pimalaya.comodoro:matrix.org)

Welcome to [**Comodoro CLI**](https://pimalaya.org/comodoro/cli/latest/), the Command-Line Interface to manage your personal time based on [time-lib](https://crates.io/crates/time-lib).

## Features

- Centralized server timer controllable by multiple clients at the same time
- Multi protocols (only [TCP](https://pimalaya.org/comodoro/cli/latest/configuration/protocols/tcp.html) for now, but you can build your own)
- [Cycles](https://pimalaya.org/comodoro/cli/latest/configuration/cycles.html) customizable via config file (Pomodoro style, 52/17 style, custom)
- Server and timer [hooks](https://pimalaya.org/comodoro/cli/latest/configuration/hooks.html) customizable via config file
- …and more!

## Installation

<table align="center">
<tr>
<td width="50%">
<a href="https://repology.org/project/comodoro/versions">
<img src="https://repology.org/badge/vertical-allrepos/comodoro.svg" alt="Packaging status" />
</a>
</td>
<td width="50%">

```bash
# Cargo
$ cargo install comodoro

# Nix
$ nix-env -i comodoro
```

*See the [documentation](https://pimalaya.org/comodoro/cli/installation.html) for other installation methods.*

</td>
</tr>
</table>

## Configuration

*Please read the [documentation](https://pimalaya.org/comodoro/cli/latest/configuration/).*

## Contributing

*Please read the [contributing guide](https://github.com/soywod/comodoro/blob/master/CONTRIBUTING.md) for more detailed information.*

A **bug tracker** is available on [SourceHut](https://todo.sr.ht/~soywod/pimalaya). <sup>[[send an email](mailto:~soywod/pimalaya@todo.sr.ht)]</sup>

A **mailing list** is available on [SourceHut](https://lists.sr.ht/~soywod/pimalaya). <sup>[[send an email](mailto:~soywod/pimalaya@lists.sr.ht)] [[subscribe](mailto:~soywod/pimalaya+subscribe@lists.sr.ht)] [[unsubscribe](mailto:~soywod/pimalaya+unsubscribe@lists.sr.ht)]</sup>

If you want to **report a bug**, please send an email at [~soywod/pimalaya@todo.sr.ht](mailto:~soywod/pimalaya@todo.sr.ht).

If you want to **propose a feature** or **fix a bug**, please send a patch at [~soywod/pimalaya@lists.sr.ht](mailto:~soywod/pimalaya@lists.sr.ht). The simplest way to send a patch is to use [git send-email](https://git-scm.com/docs/git-send-email), follow [this guide](https://git-send-email.io/) to configure git properly.

If you just want to **discuss** about the project, feel free to join the [Matrix](https://matrix.org/) workspace [#pimalaya.comodoro](https://matrix.to/#/#pimalaya.comodor:matrix.org) or contact me directly [@soywod](https://matrix.to/#/@soywod:matrix.org). You can also use the mailing list.

## Sponsoring

[![nlnet](https://nlnet.nl/logo/banner-160x60.png)](https://nlnet.nl/project/Himalaya/index.html)

Special thanks to the [NLnet foundation](https://nlnet.nl/project/Himalaya/index.html) and the [European Commission](https://www.ngi.eu/) that helped the project to receive financial support from:

- [NGI Assure](https://nlnet.nl/assure/) in 2022
- [NGI Zero Entrust](https://nlnet.nl/entrust/) in 2023

If you appreciate the project, feel free to donate using one of the following providers:

[![GitHub](https://img.shields.io/badge/-GitHub%20Sponsors-fafbfc?logo=GitHub%20Sponsors)](https://github.com/sponsors/soywod)
[![PayPal](https://img.shields.io/badge/-PayPal-0079c1?logo=PayPal&logoColor=ffffff)](https://www.paypal.com/paypalme/soywod)
[![Ko-fi](https://img.shields.io/badge/-Ko--fi-ff5e5a?logo=Ko-fi&logoColor=ffffff)](https://ko-fi.com/soywod)
[![Buy Me a Coffee](https://img.shields.io/badge/-Buy%20Me%20a%20Coffee-ffdd00?logo=Buy%20Me%20A%20Coffee&logoColor=000000)](https://www.buymeacoffee.com/soywod)
[![Liberapay](https://img.shields.io/badge/-Liberapay-f6c915?logo=Liberapay&logoColor=222222)](https://liberapay.com/soywod)
