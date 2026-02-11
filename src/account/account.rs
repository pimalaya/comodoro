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
use io_timer::timer::TimerCycle;
use serde::{Deserialize, Serialize};

#[cfg(unix)]
use crate::unix_socket::UnixSocket;
use crate::{hooks::Hooks, protocol::Protocol, tcp::Tcp, timer::TimerPrecision};

/// Represents the user config file.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Account {
    #[serde(default)]
    pub default: bool,

    #[cfg(unix)]
    pub unix_socket: Option<UnixSocket>,
    pub tcp: Option<Tcp>,

    pub cycles: Vec<TimerCycle>,
    pub cycles_count: Option<usize>,

    #[serde(default)]
    pub precision: TimerPrecision,

    #[serde(default)]
    pub hooks: Hooks,
}

impl Account {
    pub fn get_default_protocol(&self) -> Result<Protocol> {
        let mut protocol = None;

        #[cfg(unix)]
        if let Some(sock) = &self.unix_socket {
            if sock.default {
                protocol.replace(Protocol::UnixSocket);
            }
        }

        if let Some(tcp) = &self.tcp {
            if tcp.default {
                protocol.replace(Protocol::Tcp);
            }
        }

        let Some(protocol) = protocol else {
            bail!("Cannot find default protocol");
        };

        Ok(protocol)
    }
}
