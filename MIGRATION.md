# Migration guide

One section per major version, the newest first: what to change in a configuration, a script and a JSON consumer when upgrading. The CHANGELOG lists every change; these are the ones to act on.

## From v2 to v3

v3 brings Comodoro onto the standard the Pimalaya CLIs share: no configuration wizard, one configuration path per `-c`, a timer printed by every command, kebab-case values on the wire, hook variables, a session and an MCP server. A v2 configuration loads as it is, unless it marks several accounts default, holds a hook name no event fires, a hook doing nothing, a notification naming a variable its hook does not fill, an empty `cycles`, or a cycle key other than `name` and `duration`.

### Upgrading a running timer

- **Restart the server after upgrading**, and any status bar running `comodoro watch`. The wire values changed (below), so a v3 client and a v2 server cannot read each other: a v3 command meeting a v2 server fails with "Cannot read the timer server's answer, it may run another Comodoro version: restart it with this one", coded `server-mismatch` under `--json`. A remote server reached over TCP needs upgrading too.
- **The per-second notification is `timer.ticked`**, where v2 sent `timer.running`: a client of your own subscribing over JSON-RPC renames it. The timer's `state` still reads `running`.
- **The `timer.get` result spells `startedAt`**, where v2 spelled `started_at`, and its values are kebab-case: `state` is `running`, `paused` or `stopped`, and the schedule's `loops` is a number of loops or `null`, where v2 sent `"Running"` and `"Infinite"` or `{"Fixed": 4}`. The protocol keeps epoch seconds for `startedAt`.
- **Every object on the wire refuses a key it does not read**: the timer, its schedule, a cycle and an event inside a result, as the parameters and results around them already did. A server of your own sends only the documented keys: a v3 client meeting a result with an unknown key fails as `server-mismatch`, and ignores a notification whose cycle carries one.

A client of your own, reading over JSON-RPC:

```jsonc
// v2
{"jsonrpc":"2.0","method":"timer.running","params":{"cycle":{"name":"Work","duration":1499}}}
{"jsonrpc":"2.0","id":1,"result":{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":"Infinite"},"state":"Running","cycle":{"name":"Work","duration":1499},"started_at":1786711298,"elapsed":0}}
// v3
{"jsonrpc":"2.0","method":"timer.ticked","params":{"cycle":{"name":"Work","duration":1499}}}
{"jsonrpc":"2.0","id":1,"result":{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":null},"state":"running","cycle":{"name":"Work","duration":1499},"startedAt":1786711298,"elapsed":0}}
```

### Configuration and first run

- **The wizard is gone.** Bare `comodoro` prints the help, `comodoro configure` (alias `wizard`) and its `comodoro-configure` schema no longer exist, and no command offers to create a configuration. Write it from config.sample.toml, which releases and packages ship as share/doc/comodoro/config.sample.toml: run at a terminal without a configuration, Comodoro prints the commands copying it in place.
- **The sample holds its cycles in sections.** A copied config.sample.toml sets no `cycles` until one of its presets, Pomodoro, 52/17 or 112/26, is uncommented.
- **`-c` takes one path per flag.** A value is no longer split on `:`, so a Windows path stays whole. Repeat the flag to merge several files: `-c public.toml -c private.toml`.
- **`COMODORO_CONFIG` is no longer read.** Pass `-c "$COMODORO_CONFIG"` for one path, and one `-c` per path for what was a `:`-separated list.
- **Every default path is read, as layers.** $XDG_CONFIG_HOME/comodoro/config.toml, $HOME/.config/comodoro/config.toml and $HOME/.comodororc are each deep-merged on top of the ones before, where v2 read the first one found. An rc file left beside a config.toml now overrides it: delete the one you do not use.
- **Several `default = true` accounts are refused**, naming them, where v2 picked one at random. Keep it on one account.
- **An account with no cycle is refused** when a command resolves it, where v2 loaded it and the server failed once started.
- **A cycle key other than `name` and `duration` is refused**, naming it, where v2 ignored it; every other table already refused a key it does not read.
- **The 0.x keys are refused by name**: `presets`, `preset` and `timer-precision`, which v2 refused as any unknown key. See "From v0.1 to v1" below.

```sh
# v2
COMODORO_CONFIG=public.toml:private.toml comodoro get
comodoro -c public.toml:private.toml get
# v3
comodoro -c public.toml -c private.toml get
```

```toml
# v2: two defaults, an extra cycle key
[accounts.work]
default = true
cycles = [{ name = "Work", duration = 1500, color = "red" }, { name = "Rest", duration = 300 }]

[accounts.study]
default = true
cycles = [{ name = "Study", duration = 3120 }, { name = "Rest", duration = 1020 }]

# v3: one default, only `name` and `duration`
[accounts.work]
default = true
cycles = [{ name = "Work", duration = 1500 }, { name = "Rest", duration = 300 }]

[accounts.study]
cycles = [{ name = "Study", duration = 3120 }, { name = "Rest", duration = 1020 }]
```

### Hooks

- **`on-<cycle>-running` is now `on-<cycle>-tick`**, the hook firing every second a cycle runs: rename it. The old name is refused naming the new one.
- **A 1.x `cmd` key is refused** naming `command`, its spelling since 2.x, which refused it as an unknown key.
- **A hook name no event fires is refused**, naming the account's cycles and the events, where v2 loaded it and never ran it. A typo like `on-wrok-end` now fails every command: fix it.
- **A hook with neither `command` nor `notify` is refused**, where v2 loaded it and did nothing: give it one, or delete it.
- **A hook can run a command and show a notification both**, the command first, and fills `$cycleName` and `$cycleDuration` (the remaining seconds): one `notify.summary = "$cycleName ended"` serves every cycle, and a command reads them from its environment. A notification naming another variable is refused, and so is `$cycleName` on `on-timer-start` and `on-timer-stop`, which fill none. A notification is now templated, so a literal `$` in it is written `$$`.

```toml
# v2
hooks.on-work-running.command = "echo tick >> /tmp/comodoro.log"
hooks.on-work-end.notify.summary = "Work ended"
hooks.on-rest-end.notify.summary = "Rest ended"
hooks.on-work-begin.notify.summary = "Earn $5 an hour"

# v3
hooks.on-work-tick.command = "echo tick $cycleDuration >> /tmp/comodoro.log"
hooks.on-work-end.notify.summary = "$cycleName ended"
hooks.on-rest-end.notify.summary = "$cycleName ended"
hooks.on-work-begin.notify.summary = "Earn $$5 an hour"
```

### Scripts

- **The controls print the timer.** `start`, `pause`, `resume`, `stop` and `set` print the timer as it is after their change, as `get` does, where v2 printed a sentence such as "Timer successfully started": the one-line status as text (`[Work] 25min`, `[Work] paused`, `OFF`), the timer object under `--json`. A script checking the sentence checks the exit status instead, 0 on success.
- **`completion`, `manual` and `json-schema` print to stdout**, or write into the directory `--dir` names, where v2's `manual` and `json-schema` took the directory as a positional argument and `completion` wrote its file to the working directory. Each takes a list of what to generate (shells, or command names such as `comodoro-get`), every item when given none, which requires `--dir`.
- **A refused command line still exits 2**, and under `--json` now prints the failure object coded `usage-invalid` on stdout, where v2 printed clap's text on stderr (below).
- **`-a default` names the account called `default`**, where v2 took the account marked `default = true` whatever its name: omit `-a` to take that one. With no account called `default`, `-a default` fails saying to omit `-a`.

```sh
# v2
comodoro start | grep -q successfully && echo started
comodoro -a default get   # the account marked default
comodoro manual ./man
comodoro json-schema ./schemas
comodoro completion bash   # wrote a file in the working directory
# v3
comodoro start >/dev/null && echo started
comodoro get              # the account marked default
comodoro manual --dir ./man
comodoro json-schema --dir ./schemas
comodoro completion bash > comodoro.bash
```

### JSON output

- **`startedAt` is an RFC 3339 instant** under `--json`, in UTC (`2026-08-14T12:41:38Z`), where v2 printed epoch seconds under `started_at`. The JSON-RPC protocol keeps epoch seconds.
- **Values are kebab-case**, on the wire and under `--json` alike: `state` is `running`, `paused` or `stopped`, and the schedule's `loops` is a number of loops, or `null` when the timer loops forever, where v2 printed `"Infinite"` or `{"Fixed": 4}`.
- **`watch` streams events**: each change is one `timer-changed` event line, the timer's keys beside `"event": "timer-changed"`, flushed as it comes, where v2 printed bare timer objects glued together and held in the buffer. A status bar script selects `.event == "timer-changed"` lines. A signal (SIGINT, SIGTERM, Ctrl-C) now ends it with one last line, `{"stoppedBy":"signal"}` (`Watch stopped` as text), and exit 0, where v2 died silently; the server going away fails coded `server-closed` and exits 1, where v2 exited 0. The text lines reach a pipe too, as before.
- **A failure names its cause** in the `--json` failure object: `code` is `server-unreachable` when no server listens, `server-closed`, `server-error` or `server-mismatch`, and `account-missing`, `account-default-missing` or `account-default-several` when `-a` names no account, no account is marked default or several are, where v2 printed `null`; a `watch` whose server goes away fails coded `server-closed`.
- **A refused command line is a failure object** under `--json`: an unknown flag or command, a missing argument or a bad value prints the failure object coded `usage-invalid` on stdout and exits 2, where v2 printed clap's text on stderr; a refused REPL line and a refused MCP tool call carry the same code.
- **The JSON Schemas carry an `$id`** naming the contract version, and the schedule and the cycle say `additionalProperties: false`.

```jsonc
// v2: comodoro get --json
{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":"Infinite"},"state":"Running","cycle":{"name":"Work","duration":1493},"started_at":1786711298,"elapsed":0}
// v3: comodoro get --json
{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":null},"state":"running","cycle":{"name":"Work","duration":1493},"startedAt":"2026-08-14T12:41:38Z","elapsed":0}

// v2: comodoro watch --json, piped
{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":"Infinite"},"state":"Running","cycle":{"name":"Work","duration":1499},"started_at":1786711298,"elapsed":0}{"schedule":...}
// v3: comodoro watch --json, piped, then SIGTERM
{"event":"timer-changed","schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":null},"state":"running","cycle":{"name":"Work","duration":1499},"startedAt":"2026-08-14T12:41:38Z","elapsed":0}
{"stoppedBy":"signal"}

// v3: comodoro --json get --nope, exit 2, on stdout
{"code":"usage-invalid","error":"unexpected argument '--nope' found\n\n...","sources":[],"backtrace":null}
```

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
comodoro watch --json | jq -r --unbuffered 'select(.event == "timer-changed") | "\(.cycle.name) \(.cycle.duration)s"'
# a refused command line, told apart from a failed one
comodoro --json get; [ $? -eq 2 ] && echo "bad command line"
```

### Library

- **`TimerEvent::Running` is `TimerEvent::Ticked`**, and `TimerEvent::RUNNING` is `TimerEvent::TICKED` (`"timer.ticked"`).
- **`AccountConfig::cycles` is a `Vec<CycleConfig>`**, the configuration's own cycle, where it was a `Vec<TimerCycle>`; `TimerCycle::from` converts one.
- **`Config` and `AccountConfig` gain fields** refusing the 0.x keys (`presets`, `preset`, `timer_precision`), typed so they never hold a value: a pattern destructuring them names them or ends in `..`.
- **`TimerCycle`, `TimerSchedule`, `Timer` and `TimerEvent` refuse unknown keys** when deserialized.

```rust
// v2
let cycles: Vec<TimerCycle> = account_config.cycles;
if let TimerEvent::Running(cycle) = event {}
// v3
let cycles: Vec<TimerCycle> = account_config.cycles.into_iter().map(TimerCycle::from).collect();
if let TimerEvent::Ticked(cycle) = event {}
```

### New commands

- **`comodoro repl`** runs commands over one account, one per stdin line, as the other Pimalaya CLIs do; `watch` and `server start` are refused in it.
- **`comodoro mcp`** serves the timer to an agent over the Model Context Protocol, `get` always and the controls under `--allow-write`.

## From v1 to v2

v2 moved the wire to JSON-RPC 2.0 and reworked the transports. A v1 configuration loads as it is, unless a hook spells `cmd`.

- **Restart the server after upgrading.** v2 speaks JSON-RPC 2.0, so a v2 client and a v1 server cannot talk.
- **The `unix-socket` table is `socket`.** `unix-socket` stays an accepted alias, so nothing breaks; rename it at leisure. Every transport field has a default, so the tables are optional.
- **`server start` binds the transports it is given**, the default one when given none, where v1 bound every transport the account configured: a server serving both runs `comodoro server start socket tcp`.
- **The `[PROTOCOL]` argument is `[TRANSPORT]`**, taking `socket` (alias `unix-socket`) or `tcp`.
- **A hook's `cmd` is refused**: spell it `command`. v1 read both.
- **`--debug` and `--trace` are `--log-level debug` and `--log-level trace`**, beside a new `--log-file <PATH>`.
- **`manuals` and `completions` are `manual` and `completion`**; v3 accepts the plural again, hidden.
- **The cargo features `std`, `timer` and `command` are gone**: shell hooks always build, and `vendored` forwards to notify-rust.
- **`get --json` prints the v2 timer object**: the schedule replaces the `config` and `cycles-count` members, and the time a run started travels.
- **The library is rewritten**: nothing of the v1 API resolves; the TOML and the documented wire shapes are what v2 keeps stable.

```toml
# v1
[accounts.example]
unix-socket.path = "/tmp/comodoro.sock"
unix-socket.default = true
tcp.port = 9999
cycles = [{ name = "Work", duration = 1500 }, { name = "Rest", duration = 300 }]
hooks.on-work-end.cmd = "notify-send 'Work ended'"

# v2
[accounts.example]
socket.path = "/tmp/comodoro.sock"
socket.default = true
tcp.port = 9999
cycles = [{ name = "Work", duration = 1500 }, { name = "Rest", duration = 300 }]
hooks.on-work-end.command = "notify-send 'Work ended'"
```

```sh
# v1
comodoro server start
comodoro --debug start unix-socket
# v2
comodoro server start socket tcp
comodoro --log-level debug start socket
```

```jsonc
// v1: comodoro get --json
{"config":{"cycles":[{"name":"Work","duration":1500}],"cycles-count":"infinite"},"state":"running","cycle":{"name":"Work","duration":1493},"cycles-count":"infinite","elapsed":7}
// v2: comodoro get --json
{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":"Infinite"},"state":"Running","cycle":{"name":"Work","duration":1493},"started_at":1786711298,"elapsed":0}
```

## From v0.1 to v1

v1 renamed the configuration's presets to accounts, picked by `-a` rather than by a positional argument.

- **`[presets.<name>]` is `[accounts.<name>]`**, and `default = true` marks the account a command takes when `-a` names none. v3 refuses `presets` naming `accounts`.
- **`preset = "pomodoro"` or `"52/17"` is gone**: write the cycles out in `cycles`. config.sample.toml holds Pomodoro, 52/17 and 112/26 as sections to copy. v3 refuses `preset` naming them.
- **`timer-precision` is `precision`**, taking the same values. v3 refuses `timer-precision` naming `precision`.
- **A hook's `cmd` is `command`**; v1 still read `cmd`, v2 stopped.
- **The `timer` subcommand is gone**: the controls are top-level and take the account through `-a`, the transport as an optional argument. The server takes the account the same way.
- **The cargo features `hook-command` and `hook-notify` are `command` and `notify`**, and `hooks` and `tcp` are gone.

```toml
# v0.1
[presets.pomodoro]
preset = "pomodoro"
timer-precision = "second"
tcp.host = "localhost"
tcp.port = 1234
hooks.on-work-begin.cmd = "notify-send 'Work started'"

# v1
[accounts.pomodoro]
default = true
cycles = [{ name = "Work", duration = 1500 }, { name = "Rest", duration = 300 }, { name = "Work", duration = 1500 }, { name = "Rest", duration = 300 }, { name = "Work", duration = 1500 }, { name = "Long rest", duration = 1800 }]
precision = "second"
tcp.host = "localhost"
tcp.port = 1234
hooks.on-work-begin.command = "notify-send 'Work started'"
```

```sh
# v0.1
comodoro server start pomodoro tcp
comodoro timer start pomodoro tcp
comodoro timer get pomodoro tcp
# v1
comodoro -a pomodoro server start tcp
comodoro -a pomodoro start tcp
comodoro -a pomodoro get tcp
```
