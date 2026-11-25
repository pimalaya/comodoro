---
cairn: spec
capability: config
---

# Configuration

How Comodoro finds the account a command runs against. The fields of an account are documented in config.sample.toml; the transports they describe are cairn/spec/protocol.md.

## Requirement: Config never written

Comodoro SHALL NOT write a configuration file: the user writes it, from config.sample.toml.

## Requirement: A bare invocation shows the help

A bare `comodoro` SHALL show the help when a configuration exists, a broken one included, and so SHALL `--account` with no subcommand. Without one it SHALL fail as an account-bound command does, with the welcome or the error. Under `--json` it SHALL instead be a refused command line, through pimalaya-cli's `clap::meet_bare_invocation`, its configuration left unread: the failure object coded `usage-invalid`, `error` reading `A command is required, see --help`, exit 2. Comodoro SHALL NOT prompt to create a configuration.

### Scenario: A bare run under JSON

`comodoro --json`, with or without a configuration, prints `{"code":"usage-invalid","error":"A command is required, see --help",...}` on stdout, nothing on stderr, and exits 2.

## Requirement: Account resolution failures name what is missing

`-a` SHALL name an account as written, `default` being a name like any other; without `-a`, or with an empty one, the account marked `default = true` SHALL be taken. Each of the four ways account resolution fails SHALL name what is missing or wrong and what to do about it: a missing configuration names the path it looked for and points at config.sample.toml, a missing named account lists the accounts the configuration does hold, sorted, adding for the name `default` to omit `-a` to take the account marked default, a missing default names both ways of picking one, and several accounts marked `default = true`, none named, are refused naming them, sorted, rather than one taken. Under `--json` the last three SHALL be coded `account-missing`, `account-default-missing` and `account-default-several`.

### Scenario: Two default accounts

With accounts `a` and `b` both setting `default = true`, `comodoro get` without `-a` fails with "Several accounts are marked default (a, b), keep `default = true` on one, or name one with `-a <NAME>`", and `comodoro -a b get` runs on `b`.

### Scenario: `-a default` names an account

With only account `a`, marked `default = true`, `comodoro --json -a default get` fails coded `account-missing`, listing `a` and saying to omit `-a` to take the account marked default.

## Requirement: The configuration path comes from `-c` only

The configuration path SHALL be overridden by `-c` only, repeatable, the first path the base and the next ones merged on top in order. Each occurrence SHALL be one path, never split on a delimiter, and no environment variable SHALL be read for it. Without `-c`, the default locations SHALL load as layers, $HOME/.comodororc on top.

### Scenario: A Windows path is read whole

`-c C:\Users\me\config.toml` is read from that one path, not from `C` and `\Users\me\config.toml`.

## Requirement: A missing configuration meets a person with the welcome

When no configuration exists at the path `-c` gave or the default one, and stderr is a terminal with `--json` off, a bare `comodoro` and every account-bound command SHALL print the welcome on stderr and exit 1, printing nothing more. The welcome SHALL frame Comodoro in one sentence, name the missing path, give the commands creating it from config.sample.toml, invite the reader to edit it and uncomment the options they need, and give the `server start` command carrying the `-c` paths. When share/doc/comodoro/config.sample.toml exists under the prefix holding the binary's directory or under that directory itself, symlinks resolved, the commands SHALL copy it there with a writable mode, after creating the missing directory; otherwise they SHALL download it with `curl --create-dirs`. Otherwise the missing-configuration error SHALL stand, as JSON under `--json`.

### Scenario: A first run

With no file at $XDG_CONFIG_HOME/comodoro/config.toml, $HOME/.config/comodoro/config.toml or $HOME/.comodororc, `comodoro` run in a terminal prints the welcome naming $XDG_CONFIG_HOME/comodoro/config.toml, the commands creating it from config.sample.toml and `comodoro server start`, and exits 1.

## Requirement: One sample configuration, with a section per preset

The repository SHALL ship config.sample.toml holding one `[accounts.example]` table with dotted keys and no nested table, a hook's notification included (`notify.summary`, `notify.body`). `default = true` SHALL be its one active line outside the presets. Every other field the configuration reads SHALL be commented as `#key = value` on one line, at its default or an example, the other values of an enumeration listed in brackets at the end of its comment rather than as more lines, and no comment SHALL end in brackets that are not such values. Every key SHALL end in one comment tab-aligned at column 40, or past the block's longest line, and every command SHALL be a failing `exit 127` placeholder. It SHALL end with one section per cycle preset (Pomodoro, 52/17, 112/26), each a paragraph of commented code only, its name on one heading line, holding the preset's `cycles` on one line; no `cycles` SHALL be active outside them. The file SHALL hold a working account once one section is uncommented. The test suite SHALL parse config.sample.toml through the loader `-c` uses as written, refused for its missing `cycles` alone, then once per rank with every line uncommented, each field set to its value of that rank (its lines in order, each followed by its bracketed values), each preset taking one rank, so a configuration change that breaks one fails the build.

### Scenario: A field is renamed

When a field the sample's commented code sets, such as `cycles-count`, is renamed in the configuration types and the sample is not updated, the test suite fails naming the sample.

## Requirement: Unknown keys are refused at every level

Every configuration table, the root, an account, `socket`, `tcp`, a cycle, a hook and its `notify`, SHALL refuse a key it does not read, naming it, rather than ignore it. No configuration type SHALL flatten another, which would defeat the refusal. The refusal SHALL be serde's own, `deny_unknown_fields`, and nothing more: a key Comodoro used to read and removed, such as the 0.x root table `presets` or an account's 0.x `preset` and `timer-precision`, SHALL be refused as any unknown key, never by name nor pointing at MIGRATION.md, and no configuration type SHALL hold a field for it. A hook's `cmd` is no such key: it reads as the alias of `command` (cairn/spec/hooks.md).

### Scenario: A misspelled cycle key

An account whose cycles read `{ name = "Work", duraton = 1500 }` fails every command, naming `duraton` and the keys a cycle takes, `name` and `duration`.

### Scenario: A 0.x configuration

A configuration holding `[presets.example]` fails every command naming the unknown field `presets` and the one key the root reads, `accounts`, as it would a typo.
