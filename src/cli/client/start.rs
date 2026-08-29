//! # Start command
//!
//! Starts the timer the server owns, from its first cycle.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::{Message, Printer};

use crate::{
    cli::{account::Account, transport::TransportArg},
    client::std::TimerClient,
};

/// Start the timer.
///
/// Sends a start request to the server, which runs the account cycles
/// from the first one.
#[derive(Debug, Parser)]
pub struct TimerStartCommand {
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: TransportArg,
}

impl TimerStartCommand {
    /// Starts the timer the server owns.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let address = account.address(self.transport.transport);
        TimerClient::connect(&address)?.start()?;
        printer.out(Message::new("Timer successfully started"))
    }
}
