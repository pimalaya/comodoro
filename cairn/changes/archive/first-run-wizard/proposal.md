---
cairn: change
id: first-run-wizard
status: landed
created: 2026-08-14
---

# Meet the newcomer with a wizard rather than with an error

## Why

Comodoro ships no wizard, so a first run ends at `Config file not found`. The message names no path, points at nothing, and leaves someone who just installed the binary to find config.sample.toml on their own. `Account not found` is no better: it says neither which accounts exist nor how to pick one.

The bare binary is where this lands, since running a command with no argument to see what it does is what a newcomer does first. Today that prints the help, which is right for someone who is set up and useless for someone who is not.

The 1.x README promised a wizard "coming soon". Three years later the promise is still cheaper to keep than to withdraw: Comodoro has nothing to discover, so a wizard is two questions and a file.

## What

A `configure` command generates one account from the three cycle presets config.sample.toml documents, and asks nothing else: the account name comes from the preset, suffixed until free, and the endpoints question offers the local socket and TCP with both ticked.

The wizard generates, it never edits. It writes a file that does not exist, appends a plain text block to one that does, and prints the account on stdout whenever it cannot write, so `comodoro configure > config.toml` works. Appending never re-serializes the document, so comments and formatting survive, and two invariants guard it: the name must be free, since a second `[accounts.<name>]` table makes the whole file unparseable, and the account claims `default` only when no other one does, since two defaults resolve to whichever the account map yields first.

A bare `comodoro` and any command needing an account both raise the same offer when they find no configuration, behind a welcome naming the file they looked for. The offer is a hook, not a gate: a command carries on afterwards either way, so accepting gives it a chance to work and declining leaves it to fail on the configuration it still has not got. A bare invocation has nothing to carry on to, so it falls back to the help.

Nothing prompts when stdin is not a terminal or when `--json` is set: a cron job cannot answer and a script wants a failure it can read.
