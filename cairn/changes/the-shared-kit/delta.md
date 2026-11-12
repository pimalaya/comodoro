---
cairn: delta
change: the-shared-kit
---

# Delta

## ADDED Requirements

### Requirement: Account resolution failures name what is missing
(config) Each of the four ways account resolution fails SHALL name what is missing or wrong and what to do about it: a missing configuration names the path it looked for and points at config.sample.toml, a missing named account lists the accounts the configuration does hold, sorted, a missing default names both ways of picking one, and several accounts marked `default = true`, none named, are refused naming them, sorted, rather than one taken.

#### Scenario: Two default accounts
- GIVEN a configuration whose accounts `a` and `b` both set `default = true`
- WHEN `comodoro get` runs without `-a`
- THEN it fails with "Several accounts are marked default (a, b), keep `default = true` on one, or name one with `-a <NAME>`", and `comodoro -a b get` runs on `b`

### Requirement: The configuration path comes from `-c` only
(config) The configuration path SHALL be overridden by `-c` only, repeatable, the first path the base and the next ones merged on top in order. Each occurrence SHALL be one path, never split on a delimiter, and no environment variable SHALL be read for it. Without `-c`, the default locations SHALL load as layers, $HOME/.comodororc on top.

#### Scenario: A Windows path is read whole
- GIVEN `-c C:\Users\me\config.toml`
- WHEN the configuration is loaded
- THEN it is read from that one path, not from `C` and `\Users\me\config.toml`

### Requirement: A missing configuration meets a person with the welcome
(config) When no configuration exists at the path `-c` gave or the default one, and stderr is a terminal with `--json` off, a bare `comodoro` and every account-bound command SHALL print the welcome on stderr and exit 1, printing nothing more. The welcome SHALL frame Comodoro in one sentence, name the missing path, give the commands creating it from config.sample.toml, invite the reader to edit it and uncomment the options they need, and give the `server start` command carrying the `-c` paths. When share/doc/comodoro/config.sample.toml exists under the prefix holding the binary's directory or under that directory itself, symlinks resolved, the commands SHALL copy it there with a writable mode, after creating the missing directory; otherwise they SHALL download it with `curl --create-dirs`. Otherwise the missing-configuration error SHALL stand, as JSON under `--json`.

#### Scenario: A first run
- GIVEN no file at $XDG_CONFIG_HOME/comodoro/config.toml, $HOME/.config/comodoro/config.toml or $HOME/.comodororc
- WHEN `comodoro` runs in a terminal
- THEN the welcome names $XDG_CONFIG_HOME/comodoro/config.toml, the commands creating it from config.sample.toml and `comodoro server start`, and the exit status is 1

### Requirement: Server text is sanitized
(output) A cycle name a server sends SHALL reach the terminal through pimalaya-cli's `sanitize::text`, control characters and terminal escapes replaced, since a TCP server may be remote. `--json` SHALL print it verbatim.

## MODIFIED Requirements

### Requirement: A bare invocation shows the help
(config) A bare `comodoro` SHALL show the help when a configuration exists, a broken one included, and so SHALL `--account` with no subcommand. Without one it SHALL fail as an account-bound command does, with the welcome or the error. Comodoro SHALL NOT prompt to create a configuration.

### Requirement: The JSON contract is described and versioned
(output) `json-schema` SHALL describe the payload of every command printing data, and the failure object every command prints instead under the key `comodoro-error`, checked by a test walking the command tree: only a bare `comodoro` (the help) and `server start` (printing nothing) are left out. Every generated schema SHALL carry an `$id` naming the contract version, the major version of Comodoro, reading `https://pimalaya.org/schemas/comodoro/v<major>/<key>.json`. Every registered payload SHALL be a JSON object, checked by a test. The keys are `comodoro-get`, `comodoro-watch`, `comodoro-start`, `comodoro-pause`, `comodoro-resume`, `comodoro-stop`, `comodoro-set` and `comodoro-error`.

#### Scenario: `--json` output is piped
Each value is printed compact on its own line and flushed, so `watch --json` is newline-delimited JSON a reader receives as each change happens. On a terminal the value is pretty-printed instead.

## REMOVED Requirements

None.
