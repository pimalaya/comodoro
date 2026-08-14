---
cairn: spec
capability: protocol
---

# Protocol

How a Comodoro client and a Comodoro server talk to each other. This is the contract third parties implement, so it is current truth rather than history: see cairn/log/ for how it got here.

## Requirement: The wire protocol is JSON-RPC 2.0

Client and server exchange [JSON-RPC 2.0](https://www.jsonrpc.org/specification) messages. The specification defines the payload and explicitly leaves the transport to the application, which is why it was chosen: the method surface is implementable in any language without reading the Rust.

The envelope lives in src/jsonrpc20.rs and the Comodoro method surface in src/protocol.rs. Both are no_std and free of I/O.

### Scenario: A peer announces another version

A message whose `jsonrpc` member is not exactly `2.0` fails to deserialize, rather than being dispatched and failing later.

### Scenario: A response carries both a result and an error

Writing such a response is unrepresentable, since the outcome is one enum flattened into the response. Reading one resolves to its `result`: refusing to parse a peer's answer helps nobody.

## Requirement: Framing is NDJSON

One compact JSON value per line. Compact JSON contains no raw newline, so a line break is an unambiguous separator, and the stream stays readable with `jq` and writable with `socat`.

## Requirement: Transport is a Unix domain socket or TCP

Two transports carry the same JSON-RPC 2.0 payload, and a peer picks one per connection. The specification leaves the transport to the application, so nothing above the socket changes with the choice.

The local socket is the one every account has. Its path comes from `socket.path`, defaulting to comodoro.sock inside `$XDG_RUNTIME_DIR`, or inside the platform temporary directory when that variable is unset. Windows reaches the same path-addressed socket through uds_windows. Filesystem permissions come free and no port is opened, which is why it stays the default.

TCP exists for the cases a socket cannot serve, and is configured by a `tcp` table carrying `host`, defaulting to 127.0.0.1, and `port`. An account without that table opens no port at all. The listener is unauthenticated, so whoever reaches the port drives the timer: binding it to anything beyond loopback puts the timer on the network as-is.

The `socket` table is also spelled `unix-socket`, the name Comodoro 1.x used, so a 1.x account file loads unchanged.

### Scenario: A stale socket file is left by a crashed server

Binding first tries to connect to the existing path. A refused connection means the file is stale, so it is removed and binding proceeds. A successful connection means a live server owns it, so binding fails.

### Scenario: A command names no transport

The client uses the transport whose table carries `default = true`, and the local socket when neither does. `socket.default` wins over `tcp.default` when both are set, since the socket is the transport that always exists.

The server binds every transport the account configures, which is the local socket alone unless a `tcp` table is present.

### Scenario: A command names a transport the account does not configure

Only TCP can be missing, since the socket always resolves. Asking for it without a `tcp` table fails before any connection is attempted, naming the missing configuration.

### Scenario: A server binds several transports

One listener per address, each with its own accept thread, all serving the same timer. A client connected over TCP and a client connected over the socket see the same state, and a subscriber on either receives the events the other's calls emit.

Every address is bound before the first connection is accepted, so a server that cannot bind one of them starts none of them.

## Requirement: Methods are named after the imperative that performs them

`timer.get`, `timer.start`, `timer.pause`, `timer.resume`, `timer.stop`, `timer.set`, `timer.subscribe` and `timer.unsubscribe`.

`timer.set` takes a `duration` in seconds. Every other method takes no parameters. `timer.get` results in the timer, the subscription methods in `{"subscribed": bool}`, and the rest in `{"events": [...]}`.

## Requirement: The timer is one object, and it says what it runs

A timer carries its `schedule`, its `state`, its current `cycle`, the `started_at` it was last started or resumed at, and the `elapsed` seconds accumulated before the last pause or stop.

```json
{
  "schedule": {
    "cycles": [{ "name": "Work", "duration": 1500 }, { "name": "Rest", "duration": 300 }],
    "loops": "Infinite"
  },
  "state": "Running",
  "cycle": { "name": "Work", "duration": 1493 },
  "started_at": 1786711298,
  "elapsed": 0
}
```

The schedule is what the timer was given and never changes: its `cycles` in the order they run, each with its configured duration, and its `loops`, either `"Infinite"` or `{"Fixed": n}` full loops through those cycles. The `cycle` is where the timer is now, and its `duration` is the time remaining rather than the configured one, so it is the field a status bar reads. `state` is `Running`, `Paused` or `Stopped`, and `started_at` is null unless the state is `Running`.

The remaining duration is derived from `started_at` and `elapsed` rather than stored, which is why nothing in the object counts down loops or cycles: an elapsed time names both.

A method that cannot apply to the current state answers with an empty `events` array rather than with an error. Starting a running timer, pausing a paused one and stopping a stopped one all report nothing happened, since the caller asked for a state the timer is already in.

### Scenario: A method the server does not know is called

The server answers -32601, naming the method. It does not close the connection.

### Scenario: `timer.set` is called without a duration

The server answers -32602, with `["duration"]` as the error data.

### Scenario: A set duration meets the next tick

It survives. The current cycle is derived from the elapsed time rather than stored, so `timer.set` moves the elapsed time to the point that leaves the requested duration remaining. Every later tick recomputes the same value, and a client reading a second later sees what it set.

### Scenario: `timer.set` asks for more than the cycle holds

The duration is clamped to the cycle's configured length, and the `set` event carries the effective value rather than the requested one. A longer remaining duration is not representable: it would place the timeline inside the previous cycle, renaming the cycle under the caller.

Setting exactly the configured length restarts the current cycle. Setting zero ends it, and the next tick begins the following one.

### Scenario: `timer.set` is called on a stopped timer

Nothing happens, and no event comes back. `timer.start` resets the elapsed time, so a duration set beforehand could never survive the start that follows it.

### Scenario: `timer.stop` is called on a paused timer

It stops, emitting `timer.ended` then `timer.stopped` as it does from a running timer. Only an already stopped timer reports nothing.

## Requirement: Notifications are named after the past tense of what happened

`timer.started`, `timer.began`, `timer.running`, `timer.durationSet`, `timer.paused`, `timer.resumed`, `timer.ended` and `timer.stopped`. Naming the two directions differently is what keeps a request method and a notification method from ever colliding, which is why `TimerEvent::Set` maps to `timer.durationSet` rather than to `timer.set`.

A notification carries `{"cycle": {...}}` when it concerns a cycle, and no parameters otherwise. It identifies its kind by its method name, so repeating the kind in the parameters would be redundant. Events travelling inside a result have no method name to lean on, so those carry their own `event` discriminator.

### Scenario: A client receives a notification it does not know

It is ignored. A server pushing something newer than the client understands is not an error.

## Requirement: A tick reports the state it just computed

The per-second tick recomputes the current cycle from the elapsed time, then reports that value. It never reports the value it just replaced.

### Scenario: The tick stays inside its cycle

It sends `timer.running` carrying the remaining duration as of that tick.

### Scenario: The tick crosses into another cycle

It sends `timer.ended` for the cycle that finished, then `timer.began` for the one starting, and no `timer.running`. The started cycle's remaining duration travels with `timer.began`, so a `timer.running` beside it would repeat it.

A boundary is recognised by the remaining duration going back up as well as by the cycle name changing, since remaining time only ever decreases inside a cycle. A configuration looping a single cycle, or repeating a name twice in a row, therefore announces its boundaries like any other.

### Scenario: The tick changes nothing

It sends nothing. A tick lands less than a second after the last state change when a client has just called `timer.set`, and the recomputed duration is then the one already reported.

Two consecutive `timer.running` notifications therefore never carry the same duration.

### Scenario: The tick completes the last configured loop

A `cycles-count` bounds how many full loops the timer runs. The tick reaching that bound sends `timer.ended` for the cycle that ran out, then `timer.stopped`, and leaves the timer in the state a fresh one is in, exactly as `timer.stop` does.

## Requirement: Notifications reach only subscribed connections

A connection receives nothing until it calls `timer.subscribe`, and stops receiving on `timer.unsubscribe` or on disconnect. Subscribing twice on one connection is idempotent.

### Scenario: One client drives the timer while another watches

Both the events a client's own call emits and the events another client's call emits reach every subscriber, as does the per-second tick from the server's own loop.

### Scenario: A subscriber issues a request while notifications are in flight

Responses and notifications interleave on the connection. The client matches responses by `id` and buffers any notification met while waiting, so a subscribed connection can keep making calls without losing an event.

### Scenario: A subscriber disappears without unsubscribing

Its line channel is closed, so the next fan-out drops it. Nothing reaps subscribers on a timer.

## Requirement: Batches are supported

A top-level array of requests is executed in order, and answered with an array holding one response per non-notification request. A batch holding only notifications is answered with nothing. An empty batch is answered with -32600.

## Requirement: Error codes -32000 to -32099 are reserved and unused

The specification reserves that range for server-defined errors. Comodoro defines none: every failure it can report today is one of the standard codes.
