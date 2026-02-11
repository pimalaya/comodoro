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

use std::ops::Deref;

use anyhow::Result;
use clap::{builder::PossibleValue, Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Protocol {
    #[cfg(unix)]
    UnixSocket,
    Tcp,
}

impl Protocol {
    pub const ALL: &[Protocol] = &[
        #[cfg(unix)]
        Protocol::UnixSocket,
        Protocol::Tcp,
    ];
}

impl ValueEnum for Protocol {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        match input {
            #[cfg(unix)]
            p if "unix-socket" == p || ignore_case && p.eq_ignore_ascii_case("unix-socket") => {
                Ok(Self::UnixSocket)
            }
            p if "tcp" == p || ignore_case && p.eq_ignore_ascii_case("tcp") => Ok(Self::Tcp),
            p => Err(format!("Invalid protocol {p}")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[
            #[cfg(unix)]
            Self::UnixSocket,
            Self::Tcp,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            #[cfg(unix)]
            Self::UnixSocket => Some(PossibleValue::new("unix-socket")),
            Self::Tcp => Some(PossibleValue::new("tcp")),
        }
    }
}

impl ToString for Protocol {
    fn to_string(&self) -> String {
        match self {
            #[cfg(unix)]
            Self::UnixSocket => "unix-socket".into(),
            Self::Tcp => "tcp".into(),
        }
    }
}

/// The protocol name argument parser.
#[derive(Debug, Parser)]
pub struct ProtocolArg {
    /// Protocol used to send requests.
    #[arg(name = "protocol", value_name = "PROTOCOL")]
    pub value: Option<Protocol>,
}

impl Deref for ProtocolArg {
    type Target = Option<Protocol>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// The protocols name argument parser.
#[derive(Debug, Parser)]
pub struct ProtocolsArg {
    /// Protocols used to accept requests from clients.
    #[arg(name = "protocols", value_name = "PROTOCOLS")]
    pub value: Option<Vec<Protocol>>,
}

impl Deref for ProtocolsArg {
    type Target = Option<Vec<Protocol>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<ProtocolsArg> for Option<Vec<Protocol>> {
    fn from(arg: ProtocolsArg) -> Self {
        arg.value
    }
}
