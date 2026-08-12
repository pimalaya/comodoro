//! Commands managing the timer servers.
//!
//! One module per command, each acting on the server that owns the
//! timer rather than on the timer itself.

pub mod start;

use alloc::format;

use anyhow::Result;
use clap::Subcommand;

use crate::cli::{config::ComodoroAccountConfig, server::start::TimerServerStartCommand};

/// Manage servers.
///
/// A server controls a timer, and receives requests from clients to
/// manipulate it.
#[derive(Debug, Subcommand)]
pub enum TimerServerCommand {
    /// Start the server.
    Start(TimerServerStartCommand),
}

impl TimerServerCommand {
    /// Dispatches to the matching server subcommand.
    pub fn execute(self, account: &mut ComodoroAccountConfig) -> Result<()> {
        match self {
            Self::Start(cmd) => cmd.execute(account),
        }
    }
}
