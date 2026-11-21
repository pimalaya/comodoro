//! # Resume command
//!
//! Resumes the paused timer the server owns.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::Printer;

use crate::{
    cli::{account::Account, client::timer::TimerOutput, transport::TransportArg},
    client::TimerClient,
};

/// Resume the timer.
///
/// Sends a resume request to the server, which picks the paused cycle up
/// where it was left.
#[derive(Debug, Parser)]
pub struct TimerResumeCommand {
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: TransportArg,
}

impl TimerResumeCommand {
    /// Resumes the timer the server owns.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let address = account.address(self.transport.transport);
        let mut client = TimerClient::connect(&address)?;
        client.resume()?;

        let timer = client.get()?;
        printer.out(TimerOutput::new(timer, account.precision.clone()))
    }
}
