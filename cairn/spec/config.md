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

## Requirement: One sample configuration, with a section per preset

The repository SHALL ship config.sample.toml holding one `[accounts.example]` table with dotted keys and no nested table. It SHALL list the fields every account shares and each of their values, the default active and the others commented as `#key = value`, every line ending in one sentence tab-aligned at column 40, or past the block's longest line, and every command a failing `exit 127` placeholder. It SHALL end with one section per cycle preset (Pomodoro, 52/17, 112/26), each a paragraph of commented code only, headed by its name, holding the preset's `cycles` on one line; no `cycles` SHALL be active outside them. The file SHALL hold a working account once one section is uncommented. The test suite SHALL parse config.sample.toml through the loader `-c` uses with each section uncommented whole, then on top of it each commented line of code outside the sections, uncommented in place of the active lines of its field, so a configuration change that breaks one fails the build.

### Scenario: A field is renamed

When a field the sample's commented code sets, such as `cycles-count`, is renamed in the configuration types and the sample is not updated, the test suite fails naming the sample.

## Requirement: Unknown keys are refused at every level

Every configuration table, the root, an account, `socket`, `tcp`, a cycle, a hook and its `notify`, SHALL refuse a key it does not read, naming it, rather than ignore it. No configuration type SHALL flatten another, which would defeat the refusal. A key Comodoro used to read and removed SHALL be refused naming what replaced it and MIGRATION.md: the 0.x root table `presets` naming `accounts`, an account's 0.x `preset` naming the presets of config.sample.toml, its 0.x `timer-precision` naming `precision`, and the removed hook spellings (cairn/spec/hooks.md, "A removed hook spelling is refused by name").

### Scenario: A misspelled cycle key

An account whose cycles read `{ name = "Work", duraton = 1500 }` fails every command, naming `duraton` and the keys a cycle takes, `name` and `duration`.

### Scenario: A 0.x configuration

A configuration holding `[presets.example]` fails every command with "The table `presets` is now `accounts`, see MIGRATION.md".
