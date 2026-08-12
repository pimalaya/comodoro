#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Comodoro
//!
//! A shared timer: one server owns it, any number of clients drive it
//! and watch it. The whole timer logic is a pure state machine that
//! performs no I/O and reads no clock, so the interesting behaviour can
//! be tested without a socket and without waiting for time to pass.
//!
//! ## Layers
//!
//! The crate stacks three layers, gated by cargo features so a consumer
//! pays only for what it uses.
//!
//! The core is always compiled and is the only no_std layer. [`timer`]
//! holds the state machine, [`jsonrpc20`] the JSON-RPC 2.0 envelope,
//! and [`protocol`] the Comodoro method surface expressed in it. These
//! three modules are the contract, and they contain no I/O at all.
//!
//! The blocking layer arrives with the `client` and `server` features.
//! [`transport`] resolves, opens and accepts connections,
//! [`client::std`] drives a server over one of them, and [`server::std`]
//! owns the timer, answers requests and pushes notifications.
//!
//! Both sit in a module named after the runtime they are written
//! against, so an asynchronous port lands beside them as `client::tokio`
//! and `server::tokio` rather than replacing them. Inside such a module
//! the standard library is reached through `::std`, since the module
//! name shadows it.
//!
//! The CLI arrives with the `cli` feature, and lives entirely under
//! [`cli`]: the command grammar, [`cli::config`] for the TOML
//! configuration, [`cli::hook`] for the reactions run when the timer
//! emits an event, and one module per command under [`cli::client`] and
//! [`cli::server`]. One feature therefore gates one subtree.
//!
//! ## Protocol
//!
//! Client and server speak [JSON-RPC 2.0] framed as NDJSON, one compact
//! JSON value per line, over a Unix domain socket or over TCP. The
//! specification defines the payload and says nothing about transport,
//! which is exactly the boundary this crate wanted: [`protocol`] is
//! implementable by anyone in any language, without reading a line of
//! Rust.
//!
//! Requests flow client to server and are named after the imperative
//! that performs them. Notifications flow server to client, are named
//! after the past tense of what just happened, and reach only the
//! connections that asked for them with `timer.subscribe`.
//!
//! ## Where to look next
//!
//! Runnable programs live in the examples folder. The tests cover the
//! state machine, the envelope against the specification's own
//! examples, and one full client-server exchange over a real socket.
//! The development history and living design notes live in the cairn/
//! folder.
//!
//! [JSON-RPC 2.0]: https://www.jsonrpc.org/specification

extern crate alloc;
#[cfg(any(feature = "client", feature = "server"))]
extern crate std;

pub mod jsonrpc20;
pub mod protocol;
pub mod timer;

#[cfg(any(feature = "client", feature = "server"))]
pub mod transport;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "cli")]
pub mod cli;
