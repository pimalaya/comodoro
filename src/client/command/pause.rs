use anyhow::Result;
use clap::Parser;
use io_stream::runtimes::std::handle;
use io_timer::client::coroutines::PauseTimer;
use pimalaya_tui::terminal::{
    cli::printer::{Message, Printer},
    config::TomlConfig as _,
};

use crate::{account::arg::AccountNameArg, config::TomlConfig, protocol::arg::ProtocolArg};

/// Pause the timer.
///
/// This command allows you to send a request to the server in order
/// to pause the timer.
#[derive(Debug, Parser)]
pub struct PauseTimerCommand {
    #[command(flatten)]
    pub account: AccountNameArg,

    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl PauseTimerCommand {
    pub fn execute(self, printer: &mut impl Printer, config: &TomlConfig) -> Result<()> {
        let (_, account) = config.to_toml_account_config(self.account.name.as_deref())?;

        let protocol = match &*self.protocol {
            Some(protocol) => protocol.clone(),
            None => account.get_default_protocol()?,
        };

        let mut stream = protocol.connect(&account)?;

        let mut arg = None;
        let mut stop = PauseTimer::new();

        while let Err(io) = stop.resume(arg.take()) {
            arg = Some(handle(&mut stream, io)?)
        }

        printer.out(Message::new("Timer successfully paused"))
    }
}
