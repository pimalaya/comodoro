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

use std::{collections::HashMap, path::PathBuf};

use io_hook::hook::Hook;
use io_time::timer::TimerCycle;
use pimalaya_toolbox::config::TomlConfig;
use serde::{Deserialize, Serialize};

use crate::timer::TimerPrecision;

/// Represents the user config file.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    pub accounts: HashMap<String, AccountConfig>,
}

impl TomlConfig for Config {
    type Account = AccountConfig;

    fn project_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn find_default_account(&self) -> Option<(String, Self::Account)> {
        self.accounts
            .iter()
            .find(|(_, account)| account.default)
            .map(|(name, account)| (name.clone(), account.clone()))
    }

    fn find_account(&self, name: &str) -> Option<(String, Self::Account)> {
        self.accounts
            .get(name)
            .map(|account| (name.to_owned(), account.clone()))
    }
}

/// Represents the user config file.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct AccountConfig {
    #[serde(default)]
    pub default: bool,

    pub unix_socket: Option<UnixSocketConfig>,
    pub tcp: Option<TcpConfig>,

    pub cycles: Vec<TimerCycle>,
    pub cycles_count: Option<usize>,

    #[serde(default)]
    pub precision: TimerPrecision,

    #[serde(default)]
    pub hooks: HashMap<String, Hook>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct UnixSocketConfig {
    #[serde(default)]
    pub default: bool,
    pub path: PathBuf,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TcpConfig {
    #[serde(default)]
    pub default: bool,
    #[serde(default = "localhost")]
    pub host: String,
    pub port: u16,
}

fn localhost() -> String {
    String::from("127.0.0.1")
}
