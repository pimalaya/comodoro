# Migration guide

## From v2 to v3

v3 aligns Comodoro on the other Pimalaya CLIs. A v2 configuration loads as it is unless it marks several accounts default, holds a hook no event fires or a hook doing nothing, or a cycle key other than `name` and `duration`. The [CHANGELOG](./CHANGELOG.md) lists every change; these are the ones to act on.

### Configuration

- **The wizard is gone**, with `comodoro configure`. Start from [config.sample.toml](./config.sample.toml), which packages ship as share/doc/comodoro/config.sample.toml, and uncomment one of its cycle presets.
- **`-c` takes one path per flag**, no longer split on `:`: write `-c public.toml -c private.toml`.
- **`COMODORO_CONFIG` is no longer read**: pass `-c "$COMODORO_CONFIG"`.
- **Every default path is read**, each deep-merged on top of the previous one, where v2 read the first found: delete the file you do not use.
- **Several `default = true` accounts are refused**: keep it on one.
- **`-a default` names the account called `default`**: omit `-a` to take the account marked default.
- **An account without `cycles` is refused**, and so is a cycle key other than `name` and `duration`: remove it.

### Hooks

- **`on-<cycle>-running` is `on-<cycle>-tick`**: rename it.
- **A hook name no event fires is refused**, a typo such as `on-wrok-end` included: fix the name or delete the hook.
- **A hook with neither `command` nor `notify` is refused**: give it one or delete it.
- **A notification is a template**: write a literal `$` as `$$`. A cycle hook fills `$cycleName` and `$cycleDuration`, a timer hook none.

  ```toml
  # v2
  hooks.on-work-running.command = "echo tick >> /tmp/comodoro.log"
  hooks.on-work-end.notify.summary = "Work ended, earn $5"
  # v3
  hooks.on-work-tick.command = "echo tick $cycleDuration >> /tmp/comodoro.log"
  hooks.on-work-end.notify.summary = "$cycleName ended, earn $$5"
  ```

### Commands

- **The controls print the timer**, as `get` does, instead of a sentence such as "Timer successfully started": check the exit status.
- **The remaining time prints as `25m` or `1h 30m 5s`**, instead of `25min` or `1h 30min 5s`. A status bar parsing it moves to `--json`, which counts seconds.
- **`completion`, `manual` and `json-schema` print to stdout**, or write under `--dir`: `comodoro manual --dir ./man` instead of `comodoro manual ./man`, `comodoro completion bash > comodoro.bash`.
- **`server start` and `watch` stop on a signal with exit 0**, printing `Server stopped` or `Watch stopped`, where v2 died with 130 or 143: a service treating 143 as a clean stop treats 0 as one.

### Output

- **`started_at` is `startedAt`**, an RFC 3339 instant in UTC instead of epoch seconds.

  ```sh
  # the epoch seconds v2 printed
  comodoro get --json | jq '.startedAt | if . then fromdateiso8601 else null end'
  ```

- **Values are kebab-case**: `state` is `running`, `paused` or `stopped`, and `loops` is a number or `null` instead of `{"Fixed": n}` or `"Infinite"`.
- **`watch --json` streams `timer-changed` events**, one per line, then `{"stoppedBy":"signal"}`: select `.event == "timer-changed"`.
- **A failure carries a stable `code`** under `--json`, such as `server-unreachable`: match on it rather than on the wording of `error`.
- **A refused command line exits 2 coded `usage-invalid`**, printed on stdout under `--json` instead of clap's text on stderr. A bare `comodoro --json` is one.

### Protocol

- **Restart the server after upgrading**, and every `comodoro watch`: a v3 client fails on a v2 server coded `server-mismatch`.
- **A client of your own follows the new wire**: `timer.running` is `timer.ticked`, and the timer reads as under `--json` above. An object carrying a key it does not read is refused.

  ```jsonc
  // v2
  {"jsonrpc":"2.0","method":"timer.running","params":{"cycle":{"name":"Work","duration":1499}}}
  {"jsonrpc":"2.0","id":1,"result":{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":"Infinite"},"state":"Running","cycle":{"name":"Work","duration":1499},"started_at":1786711298,"elapsed":0}}
  // v3
  {"jsonrpc":"2.0","method":"timer.ticked","params":{"cycle":{"name":"Work","duration":1499}}}
  {"jsonrpc":"2.0","id":1,"result":{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":null},"state":"running","cycle":{"name":"Work","duration":1499},"startedAt":"2026-08-14T12:41:38Z","elapsed":0}}
  ```

### Building

- **The library is gone**, with the `client`, `server`, `cli` and `schemars` features: build with no feature flag, `--no-default-features` still leaving the notifications out. A program of your own speaks [the JSON-RPC protocol](./cairn/spec/protocol.md) to `comodoro server start`.

## From v1 to v2

v2 moved the wire to JSON-RPC 2.0 and reworked the transports. A v1 configuration loads as it is, unless a hook spells `cmd` (v3 reads it again).

- **Restart the server after upgrading.** v2 speaks JSON-RPC 2.0, so a v2 client and a v1 server cannot talk.
- **The `unix-socket` table is `socket`.** `unix-socket` stays an accepted alias, so nothing breaks; rename it at leisure. Every transport field has a default, so the tables are optional.
- **`server start` binds the transports it is given**, the default one when given none, where v1 bound every transport the account configured: a server serving both runs `comodoro server start socket tcp`.
- **The `[PROTOCOL]` argument is `[TRANSPORT]`**, taking `socket` (alias `unix-socket`) or `tcp`.
- **A hook's `cmd` is refused**: spell it `command`. v1 read both; v3 reads `cmd` again, as the alias of `command`.
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
