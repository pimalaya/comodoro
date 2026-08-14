//! Command overriding the remaining duration of the current cycle.

// NOTE: the clap derive expands to `format!` when parsing the duration
// value, which no_std does not put in scope.
use alloc::format;

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::{Message, Printer};

use crate::{
    cli::{account::Account, transport::TransportArg},
    client::std::TimerClient,
};

/// Set the remaining duration of the current cycle.
///
/// This command overrides how long is left in the cycle the timer is
/// currently running, without changing the configured cycles.
#[derive(Debug, Parser)]
pub struct TimerSetCommand {
    /// The new remaining duration, in seconds.
    #[arg(name = "duration", value_name = "SECONDS")]
    pub duration: usize,
    // NOTE: the transport comes last here, unlike in the other client
    // commands: clap refuses an optional positional standing before a
    // required one.
    /// The transport used to reach the server.
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: TransportArg,
}

impl TimerSetCommand {
    /// Overrides the remaining duration of the current cycle.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let address = account.address(self.transport.transport);
        TimerClient::connect(&address)?.set(self.duration)?;
        printer.out(Message::new("Timer duration successfully set"))
    }
}
