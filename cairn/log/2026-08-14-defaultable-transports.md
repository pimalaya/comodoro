---
cairn: log
change: defaultable-transports
landed: 2026-08-14
---

# Let the configuration hold addresses, and the command hold the choice

## Why

Found while reading the account model one last time before tagging: `socket` was the one transport an account could not decline. Its table carried a full default, so every account bound a socket file whether it wanted one or not, and an account meant to be reached over TCP alone still dropped a file in `$XDG_RUNTIME_DIR` for nobody. The wizard had already hit the wall and papered over it, writing `tcp.default = true` for a TCP-only account, which only changes which transport commands prefer.

## What landed

The first attempt made `socket` optional, so leaving the table out meant no socket. It worked, and it moved the wart rather than removing it: with the path already optional, the most ordinary account there is, a socket at the default path, became the one needing the strangest line, `socket = {}`.

That was the tell. One table was answering two questions: where a transport is, and whether it runs. The first is a description and belongs to the configuration; the second is an act and belongs to `server start`.

So both tables are fully defaultable now, `tcp.port` gaining the 9999 the sample had illustrated all along, and neither is optional. An account holding only its `cycles` describes both transports at both default addresses. `server start` binds the transport it is given, and the default one alone when given none, so `comodoro server start` opens no port and `comodoro server start tcp` binds no socket.

## What it cost

Resolving an address stopped being fallible, and three error paths went with it: no account can be missing a transport, so no command can fail for want of a table. The empty-address warning went too, since a server always binds exactly what it was told.

The wizard lost its second question. It asked which endpoints to serve over, which is now the server command's business, so the generated account is its cycles and nothing else: no address of this machine is written into a file that may be copied to another.

The promise that a server "binds every transport the account configures" is gone. Serving both at once is now `comodoro server start socket tcp`, which says what it does.
