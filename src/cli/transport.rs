//! Selection of the transport a command talks over.
//!
//! [`ComodoroTransport`] names one of the transports an account
//! configures, and [`ComodoroTransportArg`] is the positional argument
//! the client commands carry to pick one. Both name a configuration
//! table rather than an address: resolving one into an address is the
//! account's job, in [`crate::cli::config`].

use clap::{Parser, ValueEnum};

/// The transport a command talks to the server over.
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
pub enum ComodoroTransport {
    /// The local socket described by the `socket` table.
    #[value(name = "socket", alias = "unix-socket")]
    UnixSocket,
    /// The TCP endpoint described by the `tcp` table.
    Tcp,
}

/// The transport argument the client commands share.
#[derive(Debug, Parser)]
pub struct ComodoroTransportArg {
    /// The transport used to send the request.
    ///
    /// Defaults to the transport the account configuration marks as
    /// default, or to the local socket when neither does.
    #[arg(name = "transport", value_name = "TRANSPORT")]
    pub transport: Option<ComodoroTransport>,
}
