---
cairn: tasks
change: tcp-transport
---

# Tasks

- [x] Replace src/socket.rs with src/transport.rs, holding `TimerAddress`, `TimerStream` and `TimerListener`
- [x] Move the stale-socket cleanup from the server into `TimerListener::bind`
- [x] Give `TimerServer` a list of addresses and one accept thread per listener
- [x] Connect `TimerClient` to a `TimerAddress` rather than to a path
- [x] Restore `ComodoroTcpConfig`, `socket.default` and the `unix-socket` alias
- [x] Add `ComodoroTransport` and resolve an account to one address, or to the addresses it binds
- [x] Restore the positional transport argument on the client commands and on `server start`
- [x] Update the example, the end-to-end tests and the crate documentation
- [x] Update config.sample.toml, README.md and CHANGELOG.md
- [x] Fold the delta into cairn/spec/protocol.md and write the log entry
