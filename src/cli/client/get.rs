//! Command reading the timer.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::Printer;

use crate::{
    cli::{client::DisplayTimer, config::ComodoroAccountConfig, transport::ComodoroTransportArg},
    client::std::TimerClient,
};

/// Get the timer.
///
/// This command sends a get request to the server and displays the
/// current timer state.
#[derive(Debug, Parser)]
pub struct TimerGetCommand {
    /// The transport used to reach the server.
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: ComodoroTransportArg,
}

impl TimerGetCommand {
    /// Prints the timer state the server reports.
    pub fn execute(
        self,
        printer: &mut impl Printer,
        account: &ComodoroAccountConfig,
    ) -> Result<()> {
        let address = account.address(self.transport.transport)?;
        let timer = TimerClient::connect(&address)?.get()?;
        printer.out(DisplayTimer { account, timer })
    }
}
