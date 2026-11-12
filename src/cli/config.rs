//! # Configuration
//!
//! The TOML configuration of the Comodoro CLI. [`Config`] is the whole
//! document, a table of named accounts.
//!
//! Each [`AccountConfig`] describes the transports the two sides meet on,
//! the timer cycles, and the hooks bound to timer events. Only the cycles
//! are required, and config.sample.toml is the annotated reference.
//!
//! What lives here is the document alone: its shape and the way it is
//! read, Comodoro never writing it. What a command runs against is
//! [`Account`], the resolved view built from these types.
//!
//! [`Account`]: crate::cli::account::Account

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use std::{collections::HashMap, path::PathBuf};

use pimalaya_config::toml::TomlConfig;
use serde::Deserialize;

use crate::{
    cli::hook::TimerHook,
    timer::{TimerCycle, TimerPrecision},
    transport::{TimerAddress, default_socket_path},
};

/// The whole Comodoro configuration.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    /// The accounts, by name.
    pub accounts: HashMap<String, AccountConfig>,
}

impl TomlConfig for Config {
    type Account = AccountConfig;

    fn project_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn sample_url() -> &'static str {
        concat!(
            env!("CARGO_PKG_REPOSITORY"),
            "/blob/master/config.sample.toml"
        )
    }

    fn accounts(&self) -> impl Iterator<Item = (&str, bool)> {
        let accounts = self.accounts.iter();
        accounts.map(|(name, account)| (name.as_str(), account.default))
    }

    fn take_named_account(&mut self, name: &str) -> Option<(String, Self::Account)> {
        self.accounts.remove_entry(name)
    }
}

/// The configuration of a single Comodoro account.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct AccountConfig {
    /// Whether this account is picked when none is given.
    #[serde(default)]
    pub default: bool,
    /// Where the client and the server meet over a local socket.
    ///
    /// Every field has a default, so an account describes a socket
    /// without holding a `socket` table at all: the table adjusts the
    /// address, never whether the transport exists.
    ///
    /// Also spelled `unix-socket`, the name Comodoro 1.x used, so a 1.x
    /// account file loads unchanged.
    #[serde(default, alias = "unix-socket")]
    pub socket: SocketConfig,
    /// Where the client and the server meet over TCP.
    ///
    /// Defaulted like the socket, so `comodoro server start tcp` opens
    /// a port on an account that says nothing about TCP.
    #[serde(default)]
    pub tcp: TcpConfig,
    /// The ordered cycles the timer runs through.
    pub cycles: Vec<TimerCycle>,
    /// How many full loops the timer runs, unbounded when absent.
    pub cycles_count: Option<usize>,
    /// How precisely the remaining duration is displayed.
    #[serde(default)]
    pub precision: TimerPrecision,
    /// The hooks to run, by event name.
    #[serde(default)]
    pub hooks: HashMap<String, TimerHook>,
}

/// Where a client and a server meet over a local socket.
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SocketConfig {
    /// Whether a command talks over this transport when it names none.
    #[serde(default)]
    pub default: bool,
    /// The socket path to bind and connect to.
    ///
    /// Defaults to comodoro.sock inside `$XDG_RUNTIME_DIR`, or inside the
    /// platform temporary directory when that variable is unset.
    #[serde(default = "default_socket_path")]
    pub path: PathBuf,
}

impl SocketConfig {
    /// The address this configuration points at.
    pub fn address(&self) -> TimerAddress {
        TimerAddress::UnixSocket(self.path.clone())
    }
}

impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            default: false,
            path: default_socket_path(),
        }
    }
}

/// Where a client and a server meet over TCP.
///
/// The listener it describes is unauthenticated, so whoever reaches the
/// port drives the timer. Hence the loopback host, the one default an
/// account should have to overrule deliberately.
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TcpConfig {
    /// Whether a command talks over this transport when it names none.
    #[serde(default)]
    pub default: bool,
    /// The host to bind and connect to.
    #[serde(default = "localhost")]
    pub host: String,
    /// The port to bind and connect to.
    #[serde(default = "tcp_port")]
    pub port: u16,
}

impl TcpConfig {
    /// The address this configuration points at.
    pub fn address(&self) -> TimerAddress {
        TimerAddress::Tcp {
            host: self.host.clone(),
            port: self.port,
        }
    }
}

impl Default for TcpConfig {
    fn default() -> Self {
        Self {
            default: false,
            host: localhost(),
            port: TCP_PORT,
        }
    }
}

/// The host a `tcp` table falls back to.
///
/// The listener is unauthenticated, so an account naming no host stays
/// where only this machine can reach it.
pub const LOCALHOST: &str = "127.0.0.1";

/// The port a `tcp` table falls back to.
pub const TCP_PORT: u16 = 9999;

fn localhost() -> String {
    LOCALHOST.to_string()
}

fn tcp_port() -> u16 {
    TCP_PORT
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use core::slice;

    use std::{fs, path::PathBuf};

    use pimalaya_config::{sample::variants, toml::TomlConfig};
    use tempfile::TempDir;

    use crate::cli::{account::Account, config::Config};

    /// The headings of the cycle presets, one of which a user uncomments.
    const SECTIONS: [&str; 3] = ["# Pomodoro", "# 52/17", "# 112/26"];

    /// Checks config.sample.toml: one `[accounts.example]` table and no
    /// other, its preset sections each read through the loader `-c` uses
    /// with every variant on top.
    #[test]
    fn the_sample_configuration_parses_with_each_variant() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.sample.toml");
        let text = fs::read_to_string(&path).unwrap();
        let headers: Vec<_> = text.lines().filter(|l| l.starts_with('[')).collect();
        assert_eq!(headers, ["[accounts.example]"]);

        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");

        for variant in variants(&text, &SECTIONS) {
            let name = variant.name;

            // NOTE: a build without notifications refuses them, as it
            // should.
            if cfg!(not(feature = "notify")) && name.contains("notify") {
                continue;
            }

            fs::write(&path, &variant.text).unwrap();

            let (_, account) = Config::load(slice::from_ref(&path))
                .and_then(|mut config| Ok(config.take_account(Some("example"))?))
                .unwrap_or_else(|err| {
                    panic!("config.sample.toml with {name} uncommented: {err:?}")
                });

            let account = Account::try_from(account).unwrap();
            assert!(!account.schedule.cycles.is_empty(), "{name}");
        }
    }
}
