---
cairn: delta
change: tcp-transport
---

## MODIFIED Requirements

### Requirement: Transport is a Unix domain socket or TCP

Two transports carry the same JSON-RPC 2.0 payload, and a peer picks one per connection. The specification leaves the transport to the application, so nothing above the socket changes with the choice.

The local socket is the one every account has. Its path comes from `socket.path`, defaulting to comodoro.sock inside `$XDG_RUNTIME_DIR`, or inside the platform temporary directory when that variable is unset. Windows reaches the same path-addressed socket through uds_windows. Filesystem permissions come free and no port is opened, which is why it stays the default.

TCP exists for the cases a socket cannot serve, and is configured by a `tcp` table carrying `host`, defaulting to 127.0.0.1, and `port`. An account without that table opens no port at all. The listener is unauthenticated, so whoever reaches the port drives the timer: binding it to anything beyond loopback puts the timer on the network as-is.

The `socket` table is also spelled `unix-socket`, the name Comodoro 1.x used, so a 1.x account file loads unchanged.

#### Scenario: A stale socket file is left by a crashed server

Binding first tries to connect to the existing path. A refused connection means the file is stale, so it is removed and binding proceeds. A successful connection means a live server owns it, so binding fails.

#### Scenario: A command names no transport

The client uses the transport whose table carries `default = true`, and the local socket when neither does. `socket.default` wins over `tcp.default` when both are set, since the socket is the transport that always exists.

The server binds every transport the account configures, which is the local socket alone unless a `tcp` table is present.

#### Scenario: A command names a transport the account does not configure

Only TCP can be missing, since the socket always resolves. Asking for it without a `tcp` table fails before any connection is attempted, naming the missing configuration.

#### Scenario: A server binds several transports

One listener per address, each with its own accept thread, all serving the same timer. A client connected over TCP and a client connected over the socket see the same state, and a subscriber on either receives the events the other's calls emit.

Every address is bound before the first connection is accepted, so a server that cannot bind one of them starts none of them.
