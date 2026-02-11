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

use std::collections::HashMap;

use anyhow::Result;
#[cfg(feature = "command")]
use io_process::{
    command::Command,
    coroutines::spawn_then_wait_with_output::{
        SpawnThenWaitWithOutput, SpawnThenWaitWithOutputResult,
    },
    runtimes::std::handle,
};
use log::debug;
#[cfg(feature = "notify")]
use log::trace;
#[cfg(feature = "notify")]
use notify_rust::Notification;
use serde::{Deserialize, Serialize};

pub type Hooks = HashMap<String, Hook>;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Hook {
    /// The hook based on shell commands.
    #[cfg(feature = "command")]
    #[serde(alias = "cmd")]
    pub command: Option<Command>,

    /// The hook based on system notifications.
    #[cfg(feature = "notify")]
    pub notify: Option<Notify>,
}

impl Hook {
    pub fn exec(&self) -> Result<()> {
        #[cfg(feature = "command")]
        if let Some(cmd) = self.command.clone() {
            debug!("execute shell command hook: {cmd:?}");

            let mut arg = None;
            let mut spawn = SpawnThenWaitWithOutput::new(cmd);

            loop {
                match spawn.resume(arg.take()) {
                    SpawnThenWaitWithOutputResult::Ok(_) => break,
                    SpawnThenWaitWithOutputResult::Io(io) => arg = Some(handle(io).unwrap()),
                    SpawnThenWaitWithOutputResult::Err(err) => {
                        debug!("error while executing shell command: {err}");
                        trace!("{err:?}");
                    }
                }
            }
        }

        #[cfg(feature = "notify")]
        if let Some(notify) = self.notify.as_ref() {
            debug!("execute system notification hook: {notify:?}");

            let notif = Notification::new()
                .summary(&notify.summary)
                .body(&notify.body)
                .show();

            if let Err(err) = notif {
                debug!("error while sending system notification: {err}");
                trace!("{err:?}");
            }
        }

        Ok(())
    }
}

/// The configuration of the notify hook variant.
///
/// The structure tries to match the [`notify_rust::Notification`] API
/// and may evolve in the future.
#[cfg(feature = "notify")]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Notify {
    /// The summary (or the title) of the notification.
    pub summary: String,

    /// The body of the notification.
    pub body: String,
}
