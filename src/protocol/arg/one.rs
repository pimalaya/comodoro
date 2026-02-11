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

use clap::Parser;

use crate::protocol::Protocol;

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
