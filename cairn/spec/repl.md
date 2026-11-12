---
cairn: spec
capability: repl
---

# REPL

`comodoro repl` keeps one process alive for one account and runs the CLI's own commands over its stdin. The loop is pimalaya-cli's shared REPL (pimalaya-cli cairn/spec/repl.md).

## Requirement: A session runs the CLI over one account

`comodoro repl` SHALL start a session bound to one account and its configuration, and run pimalaya-cli's shared REPL loop over it: read one command per stdin line, split as a shell splits it, a blank line or a line whose first word starts a `#` comment skipped, parse it with the one-shot grammar plus `--json`, subcommand names inferred from a unique prefix, and run it, until `quit`, `exit` or EOF. `repl` and `mcp` SHALL be refused inside it with `Cannot start a session inside a session`, and `watch` and `server start` with `` `<command>` runs until interrupted, run it outside a session ``, pimalaya-cli installing no Ctrl-C handler. A line whose quote is not closed, a line that is not UTF-8, a parse error or a command error SHALL be printed, on stdout as the failure object when piped, and the loop SHALL continue.

### Scenario: A status bar command in a session

In a piped session, the lines `watch` and `get` answer that `watch` runs until interrupted, then print the timer.

## Requirement: Each line connects anew

Each line SHALL connect to the server on its own, so a server stopped or restarted between two lines is met by the next as a one-shot run meets it.

## Requirement: A piped session answers once per line

When stdin is not a terminal, every line SHALL produce one flushed answer on stdout: the result, or the failure object (`comodoro-error` under `--json`). Nothing SHALL be added after a result, every text output ending with its own newline. When stdin is a terminal, the prompt `comodoro> ` SHALL render on stderr.

## Requirement: A line switches itself to JSON

A line carrying `--json` SHALL print its answer as JSON, the lines after it keeping the session's format. `--json` given to `repl` SHALL make every line JSON.

## Requirement: Session flags are refused by name

A line setting a flag the session fixes when it starts, `-a`, `--account`, `-c`, `--config`, `--log-level`, `--log` or `--log-file`, SHALL fail with ``Cannot set `<flag>` inside a session, start a new session with it instead``, the flag recognised alone, as `--<long>=<value>` or as a short flag with its value attached, before any `--`.

## Requirement: Help keeps JSON lines whole

A line asking for help SHALL print clap's text on stdout in a text session, and on stderr when the session's `--json` or a `--json` word of the line is set, so stdout keeps one JSON value per line. Its usage line SHALL name the binary before the command, as `Usage: comodoro set [OPTIONS] <SECONDS> [TRANSPORT]`.
