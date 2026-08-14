//! Command starting the timer.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::{Message, Printer};

use crate::{
    cli::{account::Account, transport::TransportArg},
    client::std::TimerClient,
};

/// Start the timer.
///
/// This command sends a start request to the server.
#[derive(Debug, Parser)]
pub struct TimerStartCommand {
    /// The transport used to reach the server.
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
