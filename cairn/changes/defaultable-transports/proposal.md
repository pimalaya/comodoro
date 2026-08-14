---
cairn: change
id: defaultable-transports
status: landed
created: 2026-08-14
---

# Let the configuration hold addresses, and the command hold the choice

## Why

The `socket` table both addresses a transport and switches it on, and it cannot do the second job: it defaults to a complete configuration, so every account binds a socket file whether it wants one or not. An account meant to be reached over TCP alone still drops a file in `$XDG_RUNTIME_DIR` for nobody.

Making the table optional fixes that and buys a wart: with the path already optional, an account wanting a socket at its default path has nothing to write, so it has to write `socket = {}`. The most ordinary configuration there is becomes the one that needs the strangest line.

The wart is the tell. Two questions are being answered by one table: where a transport is, and whether it runs. The first belongs to the configuration, which is a description. The second belongs to `server start`, which is an act.

## What

Both tables become fully defaultable. `socket.path` keeps its platform default, and `tcp.port` gains one, 9999, the value the sample has illustrated all along. Neither table is optional any more, and neither has to be written: an account holding only its `cycles` describes both transports, at both default addresses.

Which transport a server binds becomes what `server start [TRANSPORTS]` says, and nothing else. Given none, it binds the default transport alone, so `comodoro server start` opens no port and `comodoro server start tcp` binds no socket. Given several, it binds those.

Which transport a client talks over stays what `socket.default` and `tcp.default` say, the socket winning when neither claims it, and what the `[TRANSPORT]` argument overrides per command.

Resolving an address stops being fallible, since every transport has one. Nothing in the account can be missing, so no command can fail for want of a transport table.
