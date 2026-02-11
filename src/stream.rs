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

#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use anyhow::{bail, Result};

use crate::{account::Account, protocol::Protocol};

pub trait Stream: Read + Write {}

impl<T: Read + Write> Stream for T {}

pub fn connect(account: &Account, protocol: &Protocol) -> Result<Box<dyn Stream>> {
    let stream: Box<dyn Stream> = match protocol {
        #[cfg(unix)]
        Protocol::UnixSocket => {
            let Some(sock) = &account.unix_socket else {
                bail!("Missing unix socket configuration");
            };

            let stream = UnixStream::connect(&sock.path)?;
            Box::new(stream)
        }
        Protocol::Tcp => {
            let Some(tcp) = &account.tcp else {
                bail!("Missing TCP configuration");
            };

            let stream = TcpStream::connect((tcp.host.as_str(), tcp.port))?;
            Box::new(stream)
        }
    };

    Ok(stream)
}
