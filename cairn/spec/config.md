---
cairn: spec
capability: config
---

# Configuration

How Comodoro finds the account a command runs against. The fields of an account are documented in config.sample.toml; the transports they describe are cairn/spec/protocol.md.

## Requirement: Config never written

Comodoro SHALL NOT write a configuration file: the user writes it, from config.sample.toml.

## Requirement: A bare invocation shows the help

A bare `comodoro` SHALL show the help when a configuration exists, a broken one included, and so SHALL `--account` with no subcommand. Without one it SHALL fail as an account-bound command does, with the welcome or the error. Comodoro SHALL NOT prompt to create a configuration.

## Requirement: Account resolution failures name what is missing

Each of the four ways account resolution fails SHALL name what is missing or wrong and what to do about it: a missing configuration names the path it looked for and points at config.sample.toml, a missing named account lists the accounts the configuration does hold, sorted, a missing default names both ways of picking one, and several accounts marked `default = true`, none named, are refused naming them, sorted, rather than one taken.

### Scenario: Two default accounts

With accounts `a` and `b` both setting `default = true`, `comodoro get` without `-a` fails with "Several accounts are marked default (a, b), keep `default = true` on one, or name one with `-a <NAME>`", and `comodoro -a b get` runs on `b`.

## Requirement: The configuration path comes from `-c` only

The configuration path SHALL be overridden by `-c` only, repeatable, the first path the base and the next ones merged on top in order. Each occurrence SHALL be one path, never split on a delimiter, and no environment variable SHALL be read for it. Without `-c`, the default locations SHALL load as layers, $HOME/.comodororc on top.

### Scenario: A Windows path is read whole

`-c C:\Users\me\config.toml` is read from that one path, not from `C` and `\Users\me\config.toml`.

## Requirement: A missing configuration meets a person with the welcome

When no configuration exists at the path `-c` gave or the default one, and stderr is a terminal with `--json` off, a bare `comodoro` and every account-bound command SHALL print the welcome on stderr and exit 1, printing nothing more. The welcome SHALL frame Comodoro in one sentence, name the missing path, give the commands creating it from config.sample.toml, invite the reader to edit it and uncomment the options they need, and give the `server start` command carrying the `-c` paths. When share/doc/comodoro/config.sample.toml exists under the prefix holding the binary's directory or under that directory itself, symlinks resolved, the commands SHALL copy it there with a writable mode, after creating the missing directory; otherwise they SHALL download it with `curl --create-dirs`. Otherwise the missing-configuration error SHALL stand, as JSON under `--json`.

### Scenario: A first run

With no file at $XDG_CONFIG_HOME/comodoro/config.toml, $HOME/.config/comodoro/config.toml or $HOME/.comodororc, `comodoro` run in a terminal prints the welcome naming $XDG_CONFIG_HOME/comodoro/config.toml, the commands creating it from config.sample.toml and `comodoro server start`, and exits 1.
