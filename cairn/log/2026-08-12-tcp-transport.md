---
cairn: log
change: tcp-transport
landed: 2026-08-12
---

# Restored the TCP transport and the v1 configuration shape

## Why

The JSON-RPC change collapsed Comodoro to a single transport and took three things with it in one move: the `tcp` table, the `unix-socket` table name and the `[PROTOCOL]` argument. The first was a deliberate call, the second was collateral, and the second is what actually broke users: `deny_unknown_fields` rejects an unknown key rather than ignoring it, so a v1 account file stopped loading on its very first line of transport configuration, whether or not it ever ran TCP.

A wire protocol is worth breaking at a major version. A configuration file is worth breaking only when the new shape is better, which was true of `unix-socket` becoming `socket` with an optional path, and not true of the file failing to parse.

The original security argument still holds where it started: a TCP listener is unauthenticated, so whoever reaches the port drives the timer. That argues for keeping TCP opt-in and loopback-by-default, which is what v1 already did, not for making v1 files unloadable.

## What landed

Two transports again. The `socket` module became `transport`, holding `TimerAddress` for where a server listens, `TimerStream` for one connection over either transport, and `TimerListener` for accepting them. The stale-socket cleanup moved out of the server and into `TimerListener::bind`, next to the binding it guards.

`TimerServer` holds a list of addresses rather than one path, binds them all before spawning anything, and runs one accept thread per listener into the same timer. A subscriber on the socket receives what a TCP client's calls emit, since the fan-out never knew which transport a connection arrived on.

The configuration gained back `tcp` with its v1 fields and `socket.default`, and `socket` accepts `unix-socket` as an alias. Client commands take back the optional positional transport, `server start` the list of them, and both accept either spelling. Omitting them keeps the v1 behaviour.

`TimerServer::new` went away with the change of shape: two public fields do not need a constructor.

## What it cost

The `[PROTOCOL]` argument came back as `[TRANSPORT]`, which is what it always described, and `set` had to order its arguments as `set <SECONDS> [TRANSPORT]` since clap refuses an optional positional standing before a required one. That is the one command whose argument order differs from the rest.

The security trade sits where v1 left it: a `tcp` table with a `port` opens an unauthenticated listener, and only the loopback default keeps it off the network. The spec now says so where the transport is defined, rather than leaving it to the reader.

## Capabilities moved

- protocol: the transport requirement, previously socket-only.
