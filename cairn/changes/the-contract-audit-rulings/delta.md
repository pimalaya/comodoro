---
cairn: delta
change: the-contract-audit-rulings
---

# Delta

## ADDED Requirements

### Requirement: An account group lists and checks the accounts
(commands) Comodoro SHALL carry an `account` group, `accounts` its hidden alias, holding `account list` and `account check`. A transport SHALL count as declared when the account holds its table or talks over it when a command names none, the socket first. `account list` SHALL read the configuration only and print pimalaya-cli's `AccountListOutput`, `backends` the declared transports. `account check` SHALL print pimalaya-cli's `AccountCheckOutput`, a list of one account holding one row per declared transport, each connecting and reading the timer, `tls` `null`, a failed row coded as `get` codes it and exiting 3 after every row printed. Both SHALL run in a REPL session; neither SHALL be an MCP tool.

#### Scenario: Three accounts
- GIVEN an account `b` marked default with its cycles only, `a` holding `tcp.port = 1234`, `c` holding `tcp.default = true`
- WHEN `comodoro --json account list` runs
- THEN it prints `a` with `["socket", "tcp"]`, `b` with `["socket"]`, `c` with `["tcp"]`

#### Scenario: A TCP port nobody serves
- GIVEN a server bound to the account's socket alone and the account holding a `tcp` table
- WHEN `comodoro --json account check` runs
- THEN the `socket` row is `ok`, the `tcp` row fails coded `server-unreachable`, and the exit code is 3

### Requirement: `server start` stops on a signal
(output) `server start` SHALL run until a termination signal stops it through pimalaya-cli's `signal::Shutdown`, then stop running hooks, remove the socket files it bound, print `{"stoppedBy": "signal"}` (`Server stopped` as text) and exit 0; a second signal ends it with 130.

#### Scenario: A service manager stops the server
- GIVEN `comodoro --json server start` running
- WHEN it receives SIGTERM
- THEN it prints `{"stoppedBy":"signal"}`, exits 0, and its socket is gone

## MODIFIED Requirements

### Requirement: The timer is one object, and it says what it runs
(protocol) `startedAt` SHALL travel as an RFC 3339 instant in UTC in whole seconds, `YYYY-MM-DDTHH:MM:SSZ`, `null` unless the timer runs; a peer SHALL refuse any other form, epoch seconds included, as another version. (Was: epoch seconds.)

### Requirement: Transport is a Unix domain socket or TCP
(protocol) Binding an address a live server or another program holds SHALL fail coded `address-busy` by the CLI, and a socket path the platform refuses SHALL fail binding and connecting coded `socket-path-invalid`, no file removed for it; a server stopped by a signal removes its socket files.

### Requirement: A JSON error carries a stable code
(output) The table gains `address-busy` and `socket-path-invalid`, and `usage-invalid` covers a REPL line running `watch` or `server start`. Every context a failure carries SHALL read `Cannot ...`.

### Requirement: A refused command line exits 2
(output) Exit 3 is the partial outcome of `account check` printing a failed row. (Was: Comodoro has no partial outcome.)

### Requirement: The JSON contract is described and versioned
(output) The keys gain `comodoro-account-list`, `comodoro-account-check` and `comodoro-server-start`; `server start` is no longer left out.

### Requirement: The timer prints as one line
(output) `TimerOutput` carries the timer's keys as the wire's timer does, `startedAt` included.

### Requirement: Unknown keys are refused at every level
(config) The refusal SHALL be serde's `deny_unknown_fields` alone: a removed key (`presets`, `preset`, `timer-precision`) SHALL be refused as any unknown key, never by name nor pointing at MIGRATION.md, and no configuration type SHALL hold a field for it.

#### Scenario: A 0.x configuration
- GIVEN a configuration holding `[presets.example]`
- WHEN any command runs
- THEN it fails naming the unknown field `presets` and the key the root reads, `accounts`

### Requirement: A hook is named after the event that fires it
(hooks) `on-<cycle>-running`, the 2.x name of `on-<cycle>-tick`, SHALL be refused as any name no event fires.

#### Scenario: A v2 tick hook
- GIVEN an account whose cycle is `Work` and whose hooks hold `on-work-running`
- WHEN any command runs
- THEN it fails as for any name no event fires, naming neither `on-work-tick` nor MIGRATION.md

### Requirement: A session runs the CLI over one account
(repl) The refusal of `watch` and `server start` SHALL be coded `usage-invalid`.

### Requirement: Writing is opt-in
(mcp) The `account` commands SHALL NOT be tools.

### Requirement: A bare invocation shows the help
(config) Under `--json` a bare `comodoro` SHALL be a refused command line through pimalaya-cli's `clap::meet_bare_invocation`, its configuration left unread: the failure object coded `usage-invalid`, exit 2.

#### Scenario: A bare run under JSON
- GIVEN a configuration or none
- WHEN `comodoro --json` runs
- THEN stdout holds the failure object coded `usage-invalid` reading `A command is required, see --help`, and the exit code is 2

## REMOVED Requirements

### Requirement: A removed hook spelling is refused by name
(hooks) Replaced by the generic refusal of a hook name no event fires (owner ruling A5).
