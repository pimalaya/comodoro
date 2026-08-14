//! Command resuming the timer.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::{Message, Printer};

use crate::{
    cli::{account::Account, transport::TransportArg},
    client::std::TimerClient,
};

/// Resume the timer.
///
/// This command sends a resume request to the server.
#[derive(Debug, Parser)]
pub struct TimerResumeCommand {
    /// The transport used to reach the server.
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: TransportArg,
}

impl TimerResumeCommand {
    /// Resumes the timer the server owns.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let address = account.address(self.transport.transport)?;
        TimerClient::connect(&address)?.resume()?;
        printer.out(Message::new("Timer successfully resumed"))
    }
}
