//! Command pausing the timer.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::{Message, Printer};

use crate::{
    cli::{client::ComodoroTransportArg, config::ComodoroAccountConfig},
    client::TimerClient,
};

/// Pause the timer.
///
/// This command sends a pause request to the server.
#[derive(Debug, Parser)]
pub struct TimerPauseCommand {
    #[command(flatten)]
    pub transport: ComodoroTransportArg,
}

impl TimerPauseCommand {
    /// Pauses the timer the server owns.
    pub fn execute(
        self,
        printer: &mut impl Printer,
        account: &ComodoroAccountConfig,
    ) -> Result<()> {
        let address = account.address(self.transport.transport)?;
        TimerClient::connect(&address)?.pause()?;
        printer.out(Message::new("Timer successfully paused"))
    }
}
