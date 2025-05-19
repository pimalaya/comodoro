pub mod arg;

#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

use anyhow::{bail, Result};
use clap::{builder::PossibleValue, ValueEnum};
use serde::{Deserialize, Serialize};

use crate::account::config::TomlAccountConfig;

pub trait StreamExt: Read + Write {}
impl<T: Read + Write> StreamExt for T {}

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

    pub fn connect(&self, account: &TomlAccountConfig) -> Result<Box<dyn StreamExt>> {
        let stream: Box<dyn StreamExt> = match self {
            #[cfg(unix)]
            Protocol::UnixSocket => {
                let Some(sock) = &account.unix_socket else {
                    bail!("missing unix socket configuration");
                };

                let stream = UnixStream::connect(&sock.path)?;
                Box::new(stream)
            }
            Protocol::Tcp => {
                let Some(tcp) = &account.tcp else {
                    bail!("missing TCP configuration");
                };

                let stream = TcpStream::connect((tcp.host.as_str(), tcp.port))?;
                Box::new(stream)
            }
        };

        Ok(stream)
    }
}

impl ValueEnum for Protocol {
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String> {
        match input {
            #[cfg(unix)]
            p if "unix-socket" == p || ignore_case && p.eq_ignore_ascii_case("unix-socket") => {
                Ok(Self::UnixSocket)
            }
            p if "tcp" == p || ignore_case && p.eq_ignore_ascii_case("tcp") => Ok(Self::Tcp),
            p => Err(format!("invalid protocol {p}")),
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
