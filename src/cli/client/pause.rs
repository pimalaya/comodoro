//! # Pause command
//!
//! Pauses the timer the server owns, keeping the elapsed time.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::{Message, Printer};

use crate::{
    cli::{account::Account, transport::TransportArg},
    client::std::TimerClient,
};

/// Pause the timer.
///
/// Sends a pause request to the server, which keeps the elapsed time.
#[derive(Debug, Parser)]
pub struct TimerPauseCommand {
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: TransportArg,
}

impl TimerPauseCommand {
    /// Pauses the timer the server owns.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let address = account.address(self.transport.transport);
        TimerClient::connect(&address)?.pause()?;
        printer.out(Message::new("Timer successfully paused"))
    }
}
