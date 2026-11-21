---
cairn: change
id: the-contract-audit-rulings
status: landed
created: 2026-11-20
---

# The contract audit rulings

## Why

The CLI contract audit of 2026-11-20 (findings COM-X1 to COM-X9) found comodoro short of the family contract in ways the 3.0 major freezes: no `account` group although comodoro has accounts (owner ruling D1, pimalaya-cli cairn/spec/commands.md); `server start` dying with 130 or 143 on the signal a service manager stops it with, silent and leaving its socket, where pimalaya-cli cairn/spec/output.md wants a long-lived command to stop cleanly and print its result; by-name refusals of removed configuration keys and their public library fields (`Config::presets`, `AccountConfig::preset`, `AccountConfig::timer_precision`), against the owner decision of 2026-11-14, restated 2026-11-16 and as ruling A5: serde `deny_unknown_fields` only; uncoded failures a caller acts on (a session's refusal of `watch` and `server start`, a busy socket, a socket path too long read as `server-unreachable`) and messages outside the family's "Cannot ..." form; and a JSON-RPC wire still carrying epoch seconds for `startedAt` while `--json` prints RFC 3339, v3 already breaking the wire. The owner ruled "Go for all" on 2026-11-20.

## What

- COM-X1: an `account` group, `account list` (pimalaya-cli `AccountListOutput`, `backends` the transports an account declares) and `account check` (pimalaya-cli `AccountCheckOutput`, a list of one account, one row per declared transport, `socket` then `tcp`, each connecting and reading the timer, `tls` `null`, exit 3 on a failed row). A transport is declared by its table or by being the default one, which needs `AccountConfig::socket` and `AccountConfig::tcp` as `Option`s. `Account` carries its `name` and `transports`, built by `Account::new(name, config)`. Not MCP tools, as in himalaya, ortie, cardamum and calendula, which list no account tool but `account capability`.
- COM-X2: `server start` installs pimalaya-cli's `signal::Shutdown`, polls it between timer events, removes the socket files it bound, prints `TimerServerStartOutput`, `{"stoppedBy": "signal"}` (`Server stopped`), registered as `comodoro-server-start`, and exits 0.
- A5 / COM-X3: the by-name refusals and their fields and types go (src/cli/config.rs, src/cli/hook.rs); serde refuses the 0.x keys as unknown fields, and `on-<cycle>-running` falls to the refusal of a hook name no event fires.
- COM-X5: the REPL's refusal of `watch` and `server start` is coded `usage-invalid`.
- COM-X6: `TimerTransportError` in the library, `AddressBusy` (a live server on the socket, `EADDRINUSE` on either transport) and `SocketPathInvalid` (the platform refusing a Unix socket path, `InvalidInput`), coded `address-busy` and `socket-path-invalid`; the transport, client and watch contexts read "Cannot ...".
- COM-X9: `Timer::started_at` travels as an RFC 3339 instant in UTC (`YYYY-MM-DDTHH:MM:SSZ`), written and read by a no_std module (src/timer/rfc3339.rs), which `TimerOutput` uses too, so jiff leaves the dependencies.
- Kit item B: a bare `comodoro` goes through pimalaya-cli's `clap::meet_bare_invocation` in place of `cli::meet_bare_invocation`, so a bare `comodoro --json` is the `usage-invalid` failure object, exit 2, the text run unchanged.
- COM-X7, COM-X8: CHANGELOG breaking markers and one pimalaya-cli line; MIGRATION.md loses its "From v0.1 to v1" section and gains a v2 to v3 entry per breaking change.
