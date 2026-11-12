# Migration guide

## From v2 to v3

v3 brings Comodoro onto the standard the Pimalaya CLIs share: no configuration wizard, one configuration path per `-c`, a timer printed by every command, kebab-case values on the wire, hook variables, a session and an MCP server. A v2 configuration loads as it is, unless it marks several accounts default, holds a hook name no event fires, or an empty `cycles`.

The CHANGELOG lists every change; these are the ones to act on.

### Upgrading a running timer

- **Restart the server after upgrading**, and any status bar running `comodoro watch`. The wire values changed (below), so a v3 client and a v2 server cannot read each other: a v3 command meeting a v2 server fails with "Cannot read the timer server's answer, it may run another Comodoro version: restart it with this one", coded `server-mismatch` under `--json`. A remote server reached over TCP needs upgrading too.

### Configuration and first run

- **The wizard is gone.** Bare `comodoro` prints the help, `comodoro configure` (alias `wizard`) and its `comodoro-configure` schema no longer exist, and no command offers to create a configuration. Write it from config.sample.toml, which releases and packages ship as share/doc/comodoro/config.sample.toml: run at a terminal without a configuration, Comodoro prints the commands copying it in place.
- **The sample holds its cycles in sections.** A copied config.sample.toml sets no `cycles` until one of its presets, Pomodoro, 52/17 or 112/26, is uncommented.
- **`-c` takes one path per flag.** A value is no longer split on `:`, so a Windows path stays whole. Repeat the flag to merge several files: `-c public.toml -c private.toml`.
- **`COMODORO_CONFIG` is no longer read.** Pass `-c "$COMODORO_CONFIG"` for one path, and one `-c` per path for what was a `:`-separated list.
- **Every default path is read, as layers.** $XDG_CONFIG_HOME/comodoro/config.toml, $HOME/.config/comodoro/config.toml and $HOME/.comodororc are each deep-merged on top of the ones before, where v2 read the first one found. An rc file left beside a config.toml now overrides it: delete the one you do not use.
- **Several `default = true` accounts are refused**, naming them, where v2 picked one at random. Keep it on one account.
- **An account with no cycle is refused** when a command resolves it, where v2 loaded it and the server failed once started.

### Hooks

- **`on-<cycle>-running` is now `on-<cycle>-tick`**, the hook firing every second a cycle runs: rename it.
- **A hook name no event fires is refused**, naming the account's cycles and the events, where v2 loaded it and never ran it. A typo like `on-wrok-end` now fails every command: fix it.
- **A hook can run a command and show a notification both**, the command first, and fills `$cycleName` and `$cycleDuration` (the remaining seconds): one `notify.summary = "$cycleName ended"` serves every cycle, and a command reads them from its environment. A notification naming another variable is refused, and so is `$cycleName` on `on-timer-start` and `on-timer-stop`, which fill none.

### Output

- **The controls print the timer.** `start`, `pause`, `resume`, `stop` and `set` print the timer as it is after their change, as `get` does, where v2 printed a sentence such as "Timer successfully started": the one-line status as text (`[Work] 25min`, `[Work] paused`, `OFF`), the timer object under `--json`. A script checking the sentence checks the exit status instead, 0 on success.
- **`startedAt` is an RFC 3339 instant** under `--json`, in UTC (`2026-11-12T09:00:00Z`), where v2 printed epoch seconds. The JSON-RPC protocol keeps epoch seconds.
- **Values are kebab-case**, on the wire and under `--json` alike: `state` is `running`, `paused` or `stopped`, and the schedule's `loops` is a number of loops, or `null` when the timer loops forever, where v2 printed `"Infinite"` or `{"Fixed": 4}`.
- **A failure names its cause** in the `--json` failure object: `code` is `server-unreachable` when no server listens, `server-closed`, `server-error` or `server-mismatch`, where v2 printed `null`.

Reading the new output with jq:

```sh
# the state, then the seconds left in the current cycle
comodoro get --json | jq -r '.state, .cycle.duration'
# the cycle name while the timer runs, nothing otherwise
comodoro get --json | jq -r 'select(.state == "running") | .cycle.name'
# when the timer started, as the epoch seconds v2 printed
comodoro get --json | jq '.startedAt | if . then fromdateiso8601 else null end'
# how many loops the timer runs, endless when unbounded
comodoro get --json | jq '.schedule.loops // "endless"'
# a status bar line on every change
comodoro watch --json | jq -r --unbuffered '"\(.cycle.name) \(.cycle.duration)s"'
```

### New commands

- **`comodoro repl`** runs commands over one account, one per stdin line, as the other Pimalaya CLIs do; `watch` and `server start` are refused in it.
- **`comodoro mcp`** serves the timer to an agent over the Model Context Protocol, `get` always and the controls under `--allow-write`.
