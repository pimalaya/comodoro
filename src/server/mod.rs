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

mod start;

use anyhow::Result;
use clap::Subcommand;

use crate::{config::AccountConfig, server::start::StartServerCommand};

/// Manage servers.
///
/// A server controls a timer, and receive requests from clients to
/// manipulate the timer.
#[derive(Debug, Subcommand)]
pub enum ServerSubcommand {
    Start(StartServerCommand),
}

impl ServerSubcommand {
    pub fn execute(self, account: &AccountConfig) -> Result<()> {
        match self {
            Self::Start(cmd) => cmd.execute(account),
        }
    }
}
