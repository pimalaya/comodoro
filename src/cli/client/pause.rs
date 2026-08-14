//! Command pausing the timer.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::{Message, Printer};

use crate::{
    cli::{account::Account, transport::TransportArg},
    client::std::TimerClient,
};

/// Pause the timer.
///
/// This command sends a pause request to the server.
#[derive(Debug, Parser)]
pub struct TimerPauseCommand {
    /// The transport used to reach the server.
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
