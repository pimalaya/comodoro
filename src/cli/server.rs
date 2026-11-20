//! # Server commands
//!
//! Commands managing the timer servers, one module each, acting on the
//! server that owns the timer rather than on the timer itself.

pub mod start;

use alloc::format;

use anyhow::Result;
use clap::Subcommand;
use pimalaya_cli::printer::Printer;

use crate::cli::{account::Account, server::start::TimerServerStartCommand};

/// Manage servers.
///
/// A server owns a timer and answers the client requests driving it.
#[derive(Debug, Subcommand)]
pub enum TimerServerCommand {
    /// Start the server.
    Start(TimerServerStartCommand),
}

impl TimerServerCommand {
    /// Dispatches to the matching server subcommand.
    pub fn execute(self, printer: &mut impl Printer, account: &mut Account) -> Result<()> {
        match self {
            Self::Start(cmd) => cmd.execute(printer, account),
        }
    }
}
