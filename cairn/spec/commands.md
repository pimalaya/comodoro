---
cairn: spec
capability: commands
---

# Commands

The command tree of Comodoro beside the timer controls, as the family's binaries share it (pimalaya-cli cairn/spec/commands.md).

## Requirement: An account group lists and checks the accounts

Comodoro SHALL carry an `account` group, `accounts` its hidden alias, holding `account list` and `account check`, and no `account capability`, having no unified API. A transport SHALL count as declared by an account when the account holds its table (`socket`, or its 1.x spelling `unix-socket`, and `tcp`) or talks over it when a command names none (cairn/spec/protocol.md, *A command names no transport*), the socket first.

`account list` SHALL read the configuration only, taking no account and reaching nothing, and print pimalaya-cli's `AccountListOutput`: one row per account sorted by name, `backends` the transports it declares (`socket`, `tcp`), `notBuilt` empty, every build serving both.

`account check` SHALL run against the account `-a` names, or the default one, and print pimalaya-cli's `AccountCheckOutput`: a list of one account, holding one row per transport it declares, each row connecting to the server at that transport's address and reading the timer, changing nothing, its `tls` `null`, no transport carrying TLS. A row that fails SHALL carry the code the same failure carries for `get` (`server-unreachable`, `socket-path-invalid`, `server-mismatch` and the others). Every row SHALL print whatever the others were, and a failed row SHALL make the command exit 3 after printing (pimalaya-cli cairn/spec/output.md, *A partial outcome prints its result*). The text SHALL be the kit's: `Account: <name>`, then one line per transport reading `OK` or `FAIL (<error>)`.

Both SHALL run in a REPL session, `account list` listing every account of the session's configuration and `account check` checking the session's account; neither SHALL be an MCP tool (cairn/spec/mcp.md).

### Scenario: Three accounts

With an account `b` marked default and holding its cycles only, an account `a` holding `tcp.port = 1234`, and an account `c` holding `tcp.default = true`, `comodoro --json account list` prints the rows of `a` with `backends` `["socket", "tcp"]`, `b` with `["socket"]` and `default` true, then `c` with `["tcp"]`.

### Scenario: A TCP port nobody serves

With a server bound to the account's socket alone and the account holding a `tcp` table, `comodoro --json account check` prints the `socket` row `ok` and the `tcp` row failed, coded `server-unreachable`, and exits 3.
