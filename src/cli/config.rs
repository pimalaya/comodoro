//! TOML configuration of the Comodoro CLI.
//!
//! [`ComodoroConfig`] is the whole document, a table of named accounts.
//! Each [`ComodoroAccountConfig`] describes the transports the client
//! and the server meet on, the timer cycles, and the hooks bound to
//! timer events. Only the cycles are required: everything else has a
//! default. See config.sample.toml for the annotated field reference.

use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use std::{collections::HashMap, path::PathBuf};

use anyhow::{Result, bail};
use pimalaya_config::toml::TomlConfig;
use serde::{Deserialize, Serialize};

use crate::{
    cli::{hook::TimerHook, transport::ComodoroTransport},
    timer::{TimerCycle, TimerPrecision},
    transport::{TimerAddress, default_socket_path},
};

/// The annotated field reference, pointed at whenever a configuration
/// is missing or a field needs a human rather than a prompt.
pub const CONFIG_SAMPLE_URL: &str =
    "https://github.com/pimalaya/comodoro/blob/master/config.sample.toml";

/// The whole Comodoro configuration.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ComodoroConfig {
    /// The accounts, by name.
    pub accounts: HashMap<String, ComodoroAccountConfig>,
}

impl TomlConfig for ComodoroConfig {
    type Account = ComodoroAccountConfig;

    fn project_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn take_default_account(&mut self) -> Option<(String, Self::Account)> {
        let name = self
            .accounts
            .iter()
            .find_map(|(name, account)| account.default.then(|| name.clone()))?;
        self.accounts.remove_entry(&name)
    }

    fn take_named_account(&mut self, name: &str) -> Option<(String, Self::Account)> {
        self.accounts.remove_entry(name)
    }
}

/// The configuration of a single Comodoro account.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ComodoroAccountConfig {
    /// Whether this account is picked when none is given.
    #[serde(default)]
    pub default: bool,
    /// The local socket the client and the server meet on.
    ///
    /// Also spelled `unix-socket`, the name Comodoro 1.x used, so a 1.x
    /// account file loads unchanged.
    #[serde(default, alias = "unix-socket")]
    pub socket: ComodoroSocketConfig,
    /// The TCP endpoint the client and the server meet on, absent when
    /// the account opens no port.
    pub tcp: Option<ComodoroTcpConfig>,
    /// The ordered cycles the timer runs through.
    pub cycles: Vec<TimerCycle>,
    /// How many full loops the timer runs before stopping, unbounded
    /// when absent.
    pub cycles_count: Option<usize>,
    /// How precisely the remaining duration is displayed.
    #[serde(default)]
    pub precision: TimerPrecision,
    /// The hooks to run, by event name.
    #[serde(default)]
    pub hooks: HashMap<String, TimerHook>,
}

impl ComodoroAccountConfig {
    /// The address a client reaches the server at.
    ///
    /// Falls back to the transport the configuration marks as default
    /// when the command names none.
    pub fn address(&self, transport: Option<ComodoroTransport>) -> Result<TimerAddress> {
        match transport.unwrap_or_else(|| self.default_transport()) {
            ComodoroTransport::UnixSocket => Ok(TimerAddress::UnixSocket(self.socket.path())),
            ComodoroTransport::Tcp => {
                let Some(tcp) = &self.tcp else {
                    bail!("Missing TCP configuration");
                };

                Ok(tcp.address())
            }
        }
    }

    /// The addresses a server binds.
    ///
    /// Falls back to every transport the configuration describes when
    /// the command names none, which is the local socket alone unless a
    /// `tcp` table is present.
    pub fn addresses(&self, transports: &[ComodoroTransport]) -> Result<Vec<TimerAddress>> {
        if transports.is_empty() {
            let mut addresses = vec![TimerAddress::UnixSocket(self.socket.path())];

            if let Some(tcp) = &self.tcp {
                addresses.push(tcp.address());
            }

            return Ok(addresses);
        }

        transports
            .iter()
            .map(|transport| self.address(Some(*transport)))
            .collect()
    }

    /// The transport picked when a command names none.
    ///
    /// The local socket wins a tie, since it is the transport every
    /// account has: `tcp.default` is honoured only when the socket does
    /// not claim the default itself.
    fn default_transport(&self) -> ComodoroTransport {
        match &self.tcp {
            Some(tcp) if tcp.default && !self.socket.default => ComodoroTransport::Tcp,
            _ => ComodoroTransport::UnixSocket,
        }
    }
}

/// The local socket transport configuration.
#[derive(Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ComodoroSocketConfig {
    /// Whether a client picks this transport when the command names
    /// none.
    #[serde(default)]
    pub default: bool,
    /// The socket path to bind and connect to.
    ///
    /// Defaults to comodoro.sock inside `$XDG_RUNTIME_DIR`, or inside
    /// the platform temporary directory when that variable is unset.
    pub path: Option<PathBuf>,
}

impl ComodoroSocketConfig {
    /// The configured socket path, or the platform default.
    pub fn path(&self) -> PathBuf {
        self.path.clone().unwrap_or_else(default_socket_path)
    }
}

/// The TCP transport configuration.
///
/// The listener it describes is unauthenticated, so whoever reaches the
/// port drives the timer. Hence the loopback default, and hence the
/// absence of any default for the port: an account opens one only by
/// saying so.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ComodoroTcpConfig {
    /// Whether a client picks this transport when the command names
    /// none.
    #[serde(default)]
    pub default: bool,
    /// The host to bind and connect to.
    #[serde(default = "localhost")]
    pub host: String,
    /// The port to bind and connect to.
    pub port: u16,
}

impl ComodoroTcpConfig {
    /// The address this configuration points at.
    pub fn address(&self) -> TimerAddress {
        TimerAddress::Tcp {
            host: self.host.clone(),
            port: self.port,
        }
    }
}

fn localhost() -> String {
    "127.0.0.1".to_string()
}
