use anyhow::{bail, Result};
use clap::Parser;
use io_stream::runtimes::std::handle;
use io_timer::client::coroutines::{send::SendRequestResult, stop::StopTimer};
use pimalaya_toolbox::terminal::printer::{Message, Printer};

use crate::{account::config::AccountConfig, protocol::arg::ProtocolArg};

/// Stop the timer.
///
/// This command allows you to send a request to the server in order
/// to stop the timer.
#[derive(Debug, Parser)]
pub struct StopTimerCommand {
    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl StopTimerCommand {
    pub fn execute(self, printer: &mut impl Printer, account: &AccountConfig) -> Result<()> {
        let protocol = match &*self.protocol {
            Some(protocol) => protocol.clone(),
            None => account.get_default_protocol()?,
        };

        let mut stream = protocol.connect(&account)?;

        let mut arg = None;
        let mut stop = StopTimer::new();

        loop {
            match stop.resume(arg.take()) {
                SendRequestResult::Ok(_) => break,
                SendRequestResult::Io(io) => arg = Some(handle(&mut stream, io)?),
                SendRequestResult::Err(err) => bail!(err),
            }
        }

        printer.out(Message::new("Timer successfully stopped"))
    }
}
