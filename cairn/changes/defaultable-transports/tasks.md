---
cairn: tasks
change: defaultable-transports
---

# Tasks

- [x] Give `socket.path` and `tcp.port` defaults, and both tables a `Default` matching them
- [x] Take both tables out of `Option`, so an account describes both transports
- [x] Bind the default transport alone when `server start` is given none
- [x] Make address resolution infallible, and drop the errors that guarded a missing table
- [x] Render only what departs from a default, so a generated account holds its cycles
- [x] Drop the wizard's endpoint question, which the server command now answers
- [x] Cover the defaults, the `default` tie-break and what a server binds
- [x] Fold the delta into cairn/spec/protocol.md and write the log entry
- [x] Update CHANGELOG.md, README.md and config.sample.toml
