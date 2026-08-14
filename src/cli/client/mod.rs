//! Commands driving the timer from a client.
//!
//! One module per command, each connecting to the server, sending its
//! request and printing the outcome. The rendering the two commands
//! showing a timer share sits in [`timer`], and the transport argument
//! they all share in [`crate::cli::transport`], since the server
//! commands select a transport too.

pub mod get;
pub mod pause;
pub mod resume;
pub mod set;
pub mod start;
pub mod stop;
pub mod timer;
pub mod watch;
