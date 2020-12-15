# 🍅 🐪 Comodoro [![gh-actions](https://github.com/soywod/comodoro/workflows/CI/badge.svg)](https://github.com/soywod/comodoro/actions?query=workflow%3ACI)

[Pomodoro](https://en.wikipedia.org/wiki/Pomodoro_Technique) timer CLI, written
in [OCaml](https://ocaml.org/index.fr.html).

## Table of contents

* [Concept](#concept)
* [Installation](#installation)
  * [From binaries](#from-binaries)
  * [From sources](#from-sources)
  * [Completion](#completion)
* [Configuration](#configuration)
* [Usage](#usage)
  * [Start](#start)
  * [Stop](#stop)
  * [Watch](#watch)
* [Contributing](#contributing)
* [Changelog](https://github.com/soywod/comodoro/blob/master/CHANGELOG.md#changelog)
* [Credits](#credits)

## Concept

Comodoro is a Pomodoro timer CLI. It helps you fight procrastination by
spliting work times from break times.

The timer is divided into 6 periods:

1. Work time (25 min)
2. Short break time (5 min)
3. Work time (25 min)
4. Short break time (5 min)
5. Work time (25 min)
6. Long break time (15 min)

Hooks can be set up for each period, to integrate any kind of workflow.

## Installation

### From binaries

```bash
curl -sSL https://raw.githubusercontent.com/soywod/comodoro/master/install.sh | bash
```

*Note: Linux, OSX and Windows are supported. See the [releases
section](https://github.com/soywod/comodoro/releases).*

### From sources

First install [`opam`](https://opam.ocaml.org/):

```bash
sh <(curl -sL https://raw.githubusercontent.com/ocaml/opam/master/shell/install.sh)
```

Then build from sources:

```bash
git clone https://github.com/soywod/comodoro.git
cd comodoro
opam install .
```

The executable is available at `_build/default/bin/main.exe`. To have globally
access you can link it this way:

```bash
ln -s `pwd`/_build/default/bin/main.exe /usr/local/bin/comodoro
```

### Completion

TODO

## Configuration

`$XDG_CONFIG_HOME/comodoro/config.toml`:

```toml
# Commands to execute when entering break times (short or long)
# Default: []
exec-on-break = ["notify-send Comodoro 'BREAK TIME'"]

# Commands to execute when re-entering work times (except the first one)
# Default: []
exec-on-resume = ["notify-send Comodoro 'WORK TIME'"]
```

*Note: `$XDG_CONFIG_HOME` is usually set to `~/.config`.*

*Note: read more about the TOML file format
[here](https://github.com/toml-lang/toml).*

## Usage

### Start

Start the timer.

```bash
comodoro start
```

### Stop

Stop the timer.

```bash
comodoro stop
```

### Watch

Watch the timer (blocking).

```bash
comodoro watch
```

## Contributing

Git commit messages follow the [Angular
Convention](https://gist.github.com/stephenparish/9941e89d80e2bc58a153), but
contain only a subject.

  > Use imperative, present tense: “change” not “changed” nor
  > “changes”<br>Don't capitalize first letter<br>No dot (.) at the end

Code should be as clean as possible, variables and functions use the snake case
convention. A line should be as short as possible to improve readability.

Tests should be added for each new functionality. Be sure to run tests before
proposing a pull request.

## Credits

- [Francesco Cirillo](https://francescocirillo.com/), the creator of the Pomodoro technique
- [Wikipedia](https://en.wikipedia.org/wiki/Pomodoro_Technique)
- [pomodoro-technique.fr](http://www.pomodoro-technique.fr/)
- [pymodoro](https://github.com/rogeralmeida/pymodoro), a python CLI
