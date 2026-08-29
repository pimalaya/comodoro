#![no_std]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Comodoro
//!
//! A shared timer: one server owns it, any number of clients drive it
//! and watch it. The timer logic is a pure state machine performing no
//! I/O and reading no clock, so the interesting behaviour is testable
//! without a socket and without waiting for time to pass.
//!
//! ## Layers
//!
//! The crate stacks three layers, gated by cargo features so a consumer
//! pays only for what it uses.
//!
//! The core is always compiled and is the only no_std layer. [`timer`]
//! holds the state machine, [`jsonrpc20`] the JSON-RPC 2.0 envelope and
//! [`protocol`] the Comodoro method surface expressed in it. These three
//! modules are the contract, and contain no I/O at all.
//!
//! The blocking layer arrives with the `client` and `server` features.
//! [`transport`] resolves, opens and accepts connections,
//! [`client::std`] drives a server over one of them, and [`server::std`]
//! owns the timer, answers requests and pushes notifications.
//!
//! Both sit in a module named after the runtime they target, so an
//! asynchronous port lands beside them as `client::tokio` and
//! `server::tokio` rather than replacing them. Inside them the standard
//! library is reached through `::std`, the module name shadowing it.
//!
//! The CLI arrives with the `cli` feature and lives entirely under
//! [`cli`], so one feature gates one subtree rather than a scattering of
//! items.
//!
//! [`cli`] holds the command grammar, [`cli::config`] the TOML document,
//! [`cli::configure`] the wizard writing one, [`cli::account`] the view a
//! command runs against, [`cli::hook`] the event reactions, and
//! [`cli::json_schema`] the schemas of the printed output.
//!
//! One module per command sits under [`cli::client`] and
//! [`cli::server`].
//!
//! ## Protocol
//!
//! Client and server speak [JSON-RPC 2.0] framed as NDJSON, one compact
//! JSON value per line, over a Unix domain socket or over TCP. The
//! specification defines the payload alone, so [`protocol`] is
//! implementable in any language, without reading a line of Rust.
//!
//! Requests flow client to server and are named after the imperative
//! that performs them.
//!
//! | Method | Parameters | Result |
//! |---|---|---|
//! | `timer.get` | none | the timer |
//! | `timer.start` | none | the events it emitted |
//! | `timer.pause` | none | the events it emitted |
//! | `timer.resume` | none | the events it emitted |
//! | `timer.stop` | none | the events it emitted |
//! | `timer.set` | `duration` in seconds | the events it emitted |
//! | `timer.subscribe` | none | whether the connection is subscribed |
//! | `timer.unsubscribe` | none | whether the connection is subscribed |
//!
//! Notifications flow the other way, are named after the past tense of
//! what just happened, and reach only the connections that asked for
//! them with `timer.subscribe`. Naming the two directions differently is
//! what keeps a request and a notification method from ever colliding.
//!
//! A subscriber receives `timer.started`, `timer.began`,
//! `timer.running`, `timer.durationSet`, `timer.paused`,
//! `timer.resumed`, `timer.ended` and `timer.stopped`, each carrying the
//! cycle it concerns except the two timer-wide ones.
//!
//! Failures come back as the standard codes, and only those.
//!
//! | Code | Meaning |
//! |---|---|
//! | -32700 | the payload is not valid JSON |
//! | -32600 | the request is malformed |
//! | -32601 | the method is unknown |
//! | -32602 | the parameters are missing or wrong |
//! | -32603 | the server failed internally |
//! | -32000 to -32099 | reserved for Comodoro, unused today |
//!
//! ## Where to look next
//!
//! Runnable programs live in the examples folder. The tests cover the
//! state machine, the envelope against the specification's own examples,
//! the account configuration and its hooks, the account the wizard
//! generates, and full exchanges over both transports.
//!
//! The development history and living design notes live in the cairn/
//! folder.
//!
//! [JSON-RPC 2.0]: https://www.jsonrpc.org/specification

extern crate alloc;
#[cfg(any(feature = "client", feature = "server"))]
extern crate std;

#[cfg(feature = "cli")]
pub mod cli;
#[cfg(feature = "client")]
pub mod client;
pub mod jsonrpc20;
pub mod protocol;
#[cfg(feature = "server")]
pub mod server;
pub mod timer;
#[cfg(any(feature = "client", feature = "server"))]
pub mod transport;
