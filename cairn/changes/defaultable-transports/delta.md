---
cairn: delta
change: defaultable-transports
---

## MODIFIED Requirements

### Requirement: Transport is a Unix domain socket or TCP

Both transports are addressable by default, and a table adjusts an address rather than switching a transport on. The local socket takes its `socket.path`, defaulting to comodoro.sock inside `$XDG_RUNTIME_DIR`, or inside the platform temporary directory when that variable is unset. Windows reaches the same path-addressed socket through uds_windows. Filesystem permissions come free and no port is opened, which is why it is the transport a command falls back to.

TCP exists for the cases a socket cannot serve, and takes its `tcp.host`, defaulting to 127.0.0.1, and `tcp.port`, defaulting to 9999. The listener is unauthenticated, so whoever reaches the port drives the timer: binding it to anything beyond loopback puts the timer on the network as-is. Nothing opens that port until a server is told to bind it.

The `socket` table is also spelled `unix-socket`, the name Comodoro 1.x used, so a 1.x account file loads unchanged.

#### Scenario: A command names no transport

The client uses the transport whose table carries `default = true`, and the local socket when neither does. `socket.default` wins over `tcp.default` when both are set, since the socket is the transport that opens no port.

The server binds that same default transport, and it alone. Serving both at once is asking for both by name.

#### Scenario: A command names a transport the account says nothing about

It resolves to that transport at its default address, since an account describing no transport describes both. Naming a transport is choosing between addresses, never asking whether one exists, so no command fails for want of a transport table.
