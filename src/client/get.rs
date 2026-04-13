// This file is part of Comodoro, a CLI to manage timers.
//
// Copyright (C) 2025-2026 Clément DOUIN <pimalaya.org@posteo.net>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Affero General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

use std::fmt;

use anyhow::{bail, Result};
use clap::Parser;
use io_socket::runtimes::std_stream::handle;
use io_time::{
    coroutines::client::{TimerRequestSend, TimerRequestSendResult},
    timer::{Timer, TimerResponse, TimerState},
};
use pimalaya_toolbox::terminal::printer::Printer;
use serde::{Serialize, Serializer};

use crate::{config::AccountConfig, protocol::ProtocolArg, stream, timer::TimerPrecision};

/// Get the timer.
///
/// This command allows you to send a request to the server in order
/// to get the actual timer and display its value.
#[derive(Debug, Parser)]
pub struct TimerGetCommand {
    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl TimerGetCommand {
    pub fn execute(self, printer: &mut impl Printer, account: &AccountConfig) -> Result<()> {
        let protocol = match &*self.protocol {
            Some(protocol) => protocol.clone(),
            None => account.try_into()?,
        };

        let mut stream = stream::connect(&account, &protocol)?;

        let mut arg = None;
        let mut client = TimerRequestSend::get();

        let timer = loop {
            match client.resume(arg.take()) {
                TimerRequestSendResult::Ok {
                    response: TimerResponse::Timer(timer),
                } => break DisplayTimer { account, timer },
                TimerRequestSendResult::Ok {
                    response: TimerResponse::Events(_),
                } => {
                    bail!("invalid response Events, expected Timer")
                }
                TimerRequestSendResult::Io { input } => arg = Some(handle(&mut stream, input)?),
                TimerRequestSendResult::Err { err } => bail!("{err}"),
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
