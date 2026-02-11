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

use clap::Parser;
use std::ops::Deref;

use crate::protocol::Protocol;

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
