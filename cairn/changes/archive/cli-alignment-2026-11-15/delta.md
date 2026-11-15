---
cairn: delta
change: cli-alignment-2026-11-15
---

# Delta

The requirements belong to cairn/spec/hooks.md, config.md, repl.md and mcp.md, each named below. In the spec files a requirement is a `##` heading and a scenario a `###` one.

## ADDED Requirements

None.

## MODIFIED Requirements

### Requirement: A hook runs a command, shows a notification, or both
In hooks.md.

A hook SHALL hold a `command`, read through pimalaya-config's `command` module and also read under `cmd`, its alias wherever the family configures a command (pimalaya-config cairn/spec/command.md), a `notify` with a `summary` and an optional `body`, or both, and SHALL be refused holding neither. With both, the command SHALL run first, then the notification show. A hook failing SHALL be logged and SHALL NOT stop the timer.

#### Scenario: A 1.x hook command

An account whose hooks hold `on-work-end.cmd = "true"` loads, and runs `true` when the Work cycle ends, as `on-work-end.command = "true"` does.

### Requirement: A removed hook spelling is refused by name
In hooks.md.

A hook name `on-<cycle>-running`, `<cycle>` one of the account's, SHALL be refused when the account is resolved, naming `on-<cycle>-tick` and MIGRATION.md.

Its scenario is unchanged.

### Requirement: Unknown keys are refused at every level
In config.md.

Every configuration table, the root, an account, `socket`, `tcp`, a cycle, a hook and its `notify`, SHALL refuse a key it does not read, naming it, rather than ignore it. No configuration type SHALL flatten another, which would defeat the refusal. A key Comodoro used to read and removed SHALL be refused naming what replaced it and MIGRATION.md: the 0.x root table `presets` naming `accounts`, an account's 0.x `preset` naming the presets of config.sample.toml, its 0.x `timer-precision` naming `precision`, and the 2.x hook name `on-<cycle>-running` (cairn/spec/hooks.md, "A removed hook spelling is refused by name"). A hook's `cmd` is no such key: it reads as the alias of `command` (cairn/spec/hooks.md).

Its scenarios are unchanged.

### Requirement: A session runs the CLI over one account
In repl.md.

`comodoro repl` SHALL start a session bound to one account and its configuration, and run pimalaya-cli's shared REPL loop over it: read one command per stdin line, split as a shell splits it, a blank line or a line whose first word starts a `#` comment skipped, parse it with the one-shot grammar plus `--json`, subcommand names inferred from a unique prefix, and run it, until `quit`, `exit` or EOF. A line starting a session, `repl` or `mcp` by its name, an alias or a prefix, SHALL be refused by pimalaya-cli before Comodoro runs it, with `Cannot start a session inside a session` coded `usage-invalid`, and `watch` and `server start` with `` `<command>` runs until interrupted, run it outside a session ``, pimalaya-cli installing no Ctrl-C handler. A line whose quote is not closed, a line that is not UTF-8, a parse error or a command error SHALL be printed, on stdout as the failure object when piped, and the loop SHALL continue.

Its scenario is unchanged.

### Requirement: Help keeps JSON lines whole
In repl.md.

A line asking for help or the version SHALL print clap's text on stdout in a text session, and on stderr when the session's `--json` or a `--json` word of the line is set, so stdout keeps one JSON value per line. Its usage line SHALL name the binary before the command, as `Usage: comodoro set [OPTIONS] <SECONDS> [TRANSPORT]`. The version SHALL be the one the command line prints, `-V` the short one and `--version` the long one, which the `Command` enum carries.

#### Scenario: The version in a text session

In a text session, the lines `-V` and `--version` print `comodoro <version>`, then the long version the command line prints, opening with `comodoro v<version>` and the build's features, and the session goes on.

### Requirement: A tool's input is the command's grammar
In mcp.md.

A tool's `inputSchema` SHALL be generated from its clap command by pimalaya-cli's `mcp::grammar::Grammar`, each property named as the CLI spells the argument (`transport`, and `seconds` for `set`), with `additionalProperties: false`, and a call SHALL be parsed back by that same command from the argv the grammar writes. An argument the `inputSchema` does not declare, a required argument missing, and a value the command's grammar refuses, SHALL be refused before the command runs as a tool error whose failure object is coded `usage-invalid`: the first naming the arguments the tool takes, the others naming the argument as the tool's input does, never clap's usage text, through `Grammar::usage_error` (``Missing argument `<name>` ``, ``Invalid value `<value>` for `<name>`: <reason>``).

#### Scenario: A negative duration

A `set` call with `seconds: -1` answers `isError` with the failure object coded `usage-invalid` reading ``Invalid value `-1` for `seconds`: invalid digit found in string``, and the server reads on.

## REMOVED Requirements

None.
