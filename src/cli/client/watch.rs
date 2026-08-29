//! # Watch command
//!
//! Prints the timer on every change the server pushes.

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::printer::Printer;

use crate::{
    cli::{account::Account, client::timer::DisplayTimer, transport::TransportArg},
    client::std::TimerClient,
};

/// Watch the timer.
///
/// Subscribes to the server and prints the timer every time it changes,
/// until interrupted. Suited to a status bar, which no longer has to
/// poll.
#[derive(Debug, Parser)]
pub struct TimerWatchCommand {
    /// The transport used to reach the server.
    #[command(flatten)]
    pub transport: TransportArg,
}

impl TimerWatchCommand {
    /// Prints the timer state on every event the server pushes.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let address = account.address(self.transport.transport);
        let mut client = TimerClient::connect(&address)?;
        client.subscribe()?;

        let mut last = client.get()?;
        printer.out(DisplayTimer {
            account,
            timer: last.clone(),
        })?;

        while client.next_event()?.is_some() {
            let timer = client.get()?;

            // NOTE: a cycle transition emits several events in the same
            // tick, all rendering the same line, so comparing the timer
            // keeps the output at one line per visible change.
            if timer == last {
                continue;
            }

            last = timer.clone();
            printer.out(DisplayTimer { account, timer })?;
        }

        Ok(())
    }
}
