---
cairn: log
change: jsonrpc-protocol
landed: 2026-08-11
---

# Adopted JSON-RPC 2.0 and dropped the coroutine layer

## Why

Comodoro had been built the I/O-free way, modelling the clock and the socket as resumable coroutines so several runtimes could share the logic. Once io-time and io-socket were absorbed, the accounting became visible: the coroutine layer was 1,005 lines of a 1,936-line crate, and the state machine it existed to protect was 513 lines that already read no clock. It had exactly one runtime, and would only ever have one, since the server is a blocking daemon and the client a short-lived process.

The layer also leaked. `TimerRequestSend::resume` took a `TimerStreamOutput` and returned a `TimerStreamInput`, so anyone holding the client had to think in reads, writes and EOF. What deserved to be portable was the payload, not the socket.

## What landed

The `coroutines`, `runtimes` and `io` modules are gone, along with the `TimeInput` and `TimerStream*` families. `Timer` is untouched and still takes `now: u64`.

In their place the crate speaks JSON-RPC 2.0, framed as NDJSON over a Unix domain socket. The envelope is a hand-rolled `jsonrpc20` module (the existing crates are all built for async servers and would have dragged tokio into a daemon serving a few requests an hour), and the Comodoro method surface is `protocol`. Both stayed no_std, so the crate keeps a genuine portable core: the contract rather than socket state machines.

Adopting a request-response standard made the missing capability obvious, so subscriptions landed with it. A connection calling `timer.subscribe` now receives a notification on every change, which is what `comodoro watch` and any status bar wanted instead of polling.

TCP went away. It already defaulted to loopback, so it granted no reach anyone had, while leaving an unauthenticated network listener in a desktop daemon. Windows reaches the same path-addressed socket through uds_windows, matching the rest of the Pimalaya stack. With one transport left, the `[PROTOCOL]` argument disappeared from every command and the transport configuration collapsed into an optional `socket.path`.

## What it cost

Subscriptions forced the server's shape. The accept loop used to serve each connection to completion before accepting the next, which a long-lived subscriber would have wedged. Each connection now gets a reader thread and a writer thread, with every outgoing line passing through one channel per connection, which serializes writes without locking the socket.

Two latent bugs surfaced and were fixed on the way: that serial accept loop, and a poisoned timer mutex making every later request call `process::exit(1)`.

## Capabilities moved

- protocol: created, holding the whole wire contract.
