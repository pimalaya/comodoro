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
use io_timer::timer::TimerCycle;
use log::debug;
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
        let mut first_available_protocol = None;
        let mut default_protocol = None;

        #[cfg(unix)]
        if let Some(sock) = &self.unix_socket {
            if first_available_protocol.is_none() {
                first_available_protocol = Some(Protocol::UnixSocket);
            }

            if sock.default {
                default_protocol.replace(Protocol::UnixSocket);
            }
        }

        if let Some(tcp) = &self.tcp {
            if first_available_protocol.is_none() {
                first_available_protocol = Some(Protocol::Tcp);
            }

            if tcp.default {
                default_protocol.replace(Protocol::Tcp);
            }
        }

        if let Some(protocol) = default_protocol {
            return Ok(protocol);
        };

        if let Some(protocol) = first_available_protocol {
            debug!("cannot find default protocol, taking the first available one: {protocol:?}");
            return Ok(protocol);
        };

        bail!("Cannot find default protocol, please configure at least one");
    }
}

/// The account name argument parser.
#[derive(Debug, Parser)]
pub struct AccountNameArg {
    /// Name of the account configuration.
    ///
    /// Uses configuration matching the given account name from the
    /// configuration file.
    #[arg(long = "account", short = 'a')]
    #[arg(name = "account-name", value_name = "NAME")]
    pub name: Option<String>,
}
