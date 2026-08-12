//! Command stopping the timer.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::{Message, Printer};

use crate::{
    cli::{client::ComodoroTransportArg, config::ComodoroAccountConfig},
    client::TimerClient,
};

/// Stop the timer.
///
/// This command sends a stop request to the server.
#[derive(Debug, Parser)]
pub struct TimerStopCommand {
    #[command(flatten)]
    pub transport: ComodoroTransportArg,
}

impl TimerStopCommand {
    /// Stops the timer the server owns.
    pub fn execute(
        self,
        printer: &mut impl Printer,
        account: &ComodoroAccountConfig,
    ) -> Result<()> {
        let address = account.address(self.transport.transport)?;
        TimerClient::connect(&address)?.stop()?;
        printer.out(Message::new("Timer successfully stopped"))
    }
}
