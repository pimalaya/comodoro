use std::fmt;

use anyhow::{bail, Result};
use clap::Parser;
use io_stream::runtimes::std::handle;
use io_timer::{
    client::coroutines::{get::GetTimer, send::SendRequestResult},
    timer::{Timer, TimerState},
    Response,
};
use pimalaya_toolbox::terminal::printer::Printer;
use serde::{Serialize, Serializer};

use crate::{
    account::config::{AccountConfig, TimerPrecision},
    protocol::arg::ProtocolArg,
};

/// Get the timer.
///
/// This command allows you to send a request to the server in order
/// to get the actual timer and display its value.
#[derive(Debug, Parser)]
pub struct GetTimerCommand {
    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl GetTimerCommand {
    pub fn execute(self, printer: &mut impl Printer, account: &AccountConfig) -> Result<()> {
        let protocol = match &*self.protocol {
            Some(protocol) => protocol.clone(),
            None => account.get_default_protocol()?,
        };

        let mut stream = protocol.connect(&account)?;

        let mut arg = None;
        let mut get = GetTimer::new();

        let timer = loop {
            match get.resume(arg.take()) {
                SendRequestResult::Ok(Response::Timer(timer)) => {
                    break DisplayTimer { account, timer }
                }
                SendRequestResult::Ok(Response::Ok) => bail!("Invalid response Ok, expected Timer"),
                SendRequestResult::Err(err) => bail!(err),
                SendRequestResult::Io(io) => arg = Some(handle(&mut stream, io)?),
            }
        };

        printer.out(timer)
    }
}

struct DisplayTimer<'a> {
    account: &'a AccountConfig,
    timer: Timer,
}

impl fmt::Display for DisplayTimer<'_> {
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

impl Serialize for DisplayTimer<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.timer.serialize(serializer)
    }
}
