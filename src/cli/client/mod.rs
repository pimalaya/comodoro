//! # Client commands
//!
//! Commands driving the timer from a client, one module each, connecting
//! to the server, sending a request and printing the outcome.
//!
//! The rendering the two commands showing a timer share sits in
//! [`timer`], and the transport argument they all share in
//! [`crate::cli::transport`], since the server commands select one too.

pub mod get;
pub mod pause;
pub mod resume;
pub mod set;
pub mod start;
pub mod stop;
pub mod timer;
pub mod watch;
