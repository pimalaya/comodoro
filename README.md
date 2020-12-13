# 🍅 🐪 Comodoro [![gh-actions](https://github.com/soywod/comodoro/workflows/CI/badge.svg)](https://github.com/soywod/comodoro/actions?query=workflow%3ACI)

[Pomodoro](https://en.wikipedia.org/wiki/Pomodoro_Technique) timer, written in
[OCaml](https://ocaml.org/index.fr.html).

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
  * [Run](#run)
  * [Watch](#watch)
* [Contributing](#contributing)
* [Changelog](https://github.com/soywod/comodoro/blob/master/CHANGELOG.md#changelog)
* [Credits](#credits)

## Concept

Comodoro is a Pomodoro timer CLI. It helps you fight procrastination by
spliting work times from break times.

The timer can be divided into 6 steps:

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

TODO

### Completion

TODO

## Configuration

Comodoro is customizable via a [TOML](https://github.com/toml-lang/toml) file
(`~/.config/comodoro/config.toml`):

```toml
# Commands to execute when entering break times (short or long)
# Default: []
exec-on-break = ["notify-send Comodoro 'BREAK TIME'"]

# Commands to execute when re-entering work times (not the first one)
# Default: []
exec-on-resume = ["notify-send Comodoro 'WORK TIME'"]
```

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

### Run

Run the timer loop (blocking).

```bash
comodoro run
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
