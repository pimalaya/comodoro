//! # Get command
//!
//! Reads the timer the server owns, without touching it.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::Printer;

use crate::{
    cli::{account::Account, client::timer::DisplayTimer, transport::TransportArg},
    client::std::TimerClient,
};

/// Get the timer.
///
/// Sends a get request to the server and prints the timer it reports,
/// at the precision the account configures.
#[derive(Debug, Parser)]
pub struct TimerGetCommand {
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: TransportArg,
}

impl TimerGetCommand {
    /// Prints the timer state the server reports.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let address = account.address(self.transport.transport);
        let timer = TimerClient::connect(&address)?.get()?;
        printer.out(DisplayTimer { account, timer })
    }
}
