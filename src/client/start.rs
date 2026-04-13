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

use anyhow::{bail, Result};
use clap::Parser;
use io_socket::runtimes::std_stream::handle;
use io_time::coroutines::client::{TimerRequestSend, TimerRequestSendResult};
use pimalaya_toolbox::terminal::printer::{Message, Printer};

use crate::{config::AccountConfig, protocol::ProtocolArg, stream};

/// Start the timer.
///
/// This command allows you to send a request to the server in order
/// to start the timer.
#[derive(Debug, Parser)]
pub struct TimerStartCommand {
    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl TimerStartCommand {
    pub fn execute(self, printer: &mut impl Printer, account: &AccountConfig) -> Result<()> {
        let protocol = match &*self.protocol {
            Some(protocol) => protocol.clone(),
            None => account.try_into()?,
        };

        let mut stream = stream::connect(&account, &protocol)?;

        let mut arg = None;
        let mut client = TimerRequestSend::start();

        loop {
            match client.resume(arg.take()) {
                TimerRequestSendResult::Ok { .. } => break,
                TimerRequestSendResult::Io { input } => arg = Some(handle(&mut stream, input)?),
                TimerRequestSendResult::Err { err } => bail!("{err}"),
            }
        }

        printer.out(Message::new("Timer successfully started"))
    }
}
