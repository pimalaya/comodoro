---
cairn: change
id: tcp-transport
status: landed
created: 2026-08-12
---

# Restore the TCP transport and the v1 configuration shape

## Why

The JSON-RPC change collapsed Comodoro down to a single transport, dropping the `tcp` table, the `unix-socket` table name and the `[PROTOCOL]` argument in one move. That broke every v1 configuration file: an account written for v1 now fails to parse on `unix-socket`, since `deny_unknown_fields` rejects the key rather than ignoring it.

Comodoro 2.0 is worth a break in the wire protocol, but not worth a break in the configuration file for users who never ran a TCP listener in the first place. A v1 configuration should keep working, and the accounts that did run TCP should keep running it.

The removal argument still holds where it started: a TCP listener is unauthenticated, so anyone who can reach the port drives the timer. That is an argument for keeping TCP opt-in and loopback-by-default, which is what v1 already did, not for making a v1 file unloadable.

## What

Two transports again, selected per command.

The account configuration gains back `tcp`, with the v1 fields `host` (defaulting to 127.0.0.1), `port` and `default`. It stays absent by default, so no account opens a port unless it says so.

The socket table gains back `default`, and accepts `unix-socket` as an alias of `socket`, which is what makes a v1 file load unchanged. The v2 spelling stays the canonical one.

Client commands take back the optional positional transport argument, and `server start` takes back the list of transports to bind. Both accept `socket` and its `unix-socket` alias. Omitting the argument keeps the v1 behaviour: the client picks the transport marked `default`, and the server binds every transport the account configures.

Under the CLI, the socket-only types generalise. `TimerAddress` says where a server listens, `TimerStream` is a connection over either transport, and `TimerListener` accepts on either. `TimerServer` holds a list of addresses rather than one path, and binds one listener per address.
