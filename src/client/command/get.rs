use std::fmt;

use anyhow::{bail, Result};
use clap::Parser;
use io_stream::runtimes::std::handle;
use io_timer::{
    client::coroutines::GetTimer,
    timer::{Timer, TimerState},
    Response,
};
use pimalaya_tui::terminal::{cli::printer::Printer, config::TomlConfig as _};
use serde::{Serialize, Serializer};

use crate::{
    account::{
        arg::AccountNameArg,
        config::{TimerPrecision, TomlAccountConfig},
    },
    config::TomlConfig,
    protocol::arg::ProtocolArg,
};

/// Get the timer.
///
/// This command allows you to send a request to the server in order
/// to get the actual timer and display its value.
#[derive(Debug, Parser)]
pub struct GetTimerCommand {
    #[command(flatten)]
    pub account: AccountNameArg,

    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl GetTimerCommand {
    pub fn execute(self, printer: &mut impl Printer, config: &TomlConfig) -> Result<()> {
        let (_, account) = config.to_toml_account_config(self.account.name.as_deref())?;

        let protocol = match &*self.protocol {
            Some(protocol) => protocol.clone(),
            None => account.get_default_protocol()?,
        };

        let mut stream = protocol.connect(&account)?;

        let mut arg = None;
        let mut get = GetTimer::new();

        let timer = loop {
            match get.resume(arg.take()) {
                Ok(Response::Timer(timer)) => break DisplayTimer { account, timer },
                Ok(Response::Ok) => bail!("invalid response Ok, expected Timer"),
                Err(io) => arg = Some(handle(&mut stream, io)?),
            }
        };

        printer.out(timer)
    }
}

struct DisplayTimer {
    account: TomlAccountConfig,
    timer: Timer,
}

impl fmt::Display for DisplayTimer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timer = &self.timer;
        let cycle = &timer.cycle.name;

        match timer.state {
            TimerState::Stopped => write!(f, "OFF"),
            TimerState::Paused => write!(f, "[{cycle}] paused"),
            TimerState::Running if timer.cycle.duration < 60 => {
                write!(f, "[{cycle}] {}s", timer.cycle.duration)
            }
            TimerState::Running if timer.cycle.duration < 3600 => match self.account.precision {
                TimerPrecision::Second => write!(
                    f,
                    "[{cycle}] {}min {}s",
                    timer.cycle.duration / 60,
                    timer.cycle.duration % 60
                ),
                TimerPrecision::Minute | TimerPrecision::Hour => {
                    write!(f, "[{cycle}] {}min", timer.cycle.duration / 60,)
                }
            },
            TimerState::Running => match self.account.precision {
                TimerPrecision::Second => write!(
                    f,
                    "[{cycle}] {}h {}min {}s",
                    timer.cycle.duration / 3600,
                    (timer.cycle.duration % 3600) / 60,
                    (timer.cycle.duration % 3600) % 60,
                ),
                TimerPrecision::Minute => write!(
                    f,
                    "[{cycle}] {}h {}min",
                    timer.cycle.duration / 3600,
                    (timer.cycle.duration % 3600) / 60,
                ),
                TimerPrecision::Hour => write!(f, "[{cycle}] {}h", timer.cycle.duration / 3600),
            },
        }
    }
}

impl Serialize for DisplayTimer {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.timer.serialize(serializer)
    }
}
