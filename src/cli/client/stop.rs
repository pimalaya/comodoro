//! # Stop command
//!
//! Stops the timer the server owns, and resets it.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::Printer;

use crate::{
    cli::{account::Account, client::timer::TimerOutput, transport::TransportArg},
    client::std::TimerClient,
};

/// Stop the timer.
///
/// Sends a stop request to the server, which resets the timer to its
/// first cycle. Stops a paused timer as well as a running one.
#[derive(Debug, Parser)]
pub struct TimerStopCommand {
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: TransportArg,
}

impl TimerStopCommand {
    /// Stops the timer the server owns.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let address = account.address(self.transport.transport);
        let mut client = TimerClient::connect(&address)?;
        client.stop()?;

        let timer = client.get()?;
        printer.out(TimerOutput::new(timer, account.precision.clone()))
    }
}
