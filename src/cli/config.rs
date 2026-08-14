//! TOML configuration of the Comodoro CLI.
//!
//! [`Config`] is the whole document, a table of named accounts. Each
//! [`AccountConfig`] describes the transports the client and the server
//! meet on, the timer cycles, and the hooks bound to timer events. Only
//! the cycles are required: everything else has a default. See
//! config.sample.toml for the annotated field reference.
//!
//! What lives here is the document and nothing else: its shape, the way
//! it is read, and the way an account is written back to it. What a
//! command runs against is [`Account`], the resolved view built from
//! these types.
//!
//! [`Account`]: crate::cli::account::Account

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use std::{collections::HashMap, path::PathBuf};

use pimalaya_config::toml::TomlConfig;
use serde::{Deserialize, Serialize};

use crate::{
    cli::hook::TimerHook,
    timer::{TimerCycle, TimerPrecision},
    transport::{TimerAddress, default_socket_path},
};

/// The annotated field reference, pointed at whenever a configuration
/// is missing or a field needs a human rather than a prompt.
pub const CONFIG_SAMPLE_URL: &str = concat!(
    env!("CARGO_PKG_REPOSITORY"),
    "/blob/master/config.sample.toml"
);

/// The whole Comodoro configuration.
#[derive(Debug, Default, Deserialize, Serialize)]
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
pub struct AccountConfig {
    /// Whether this account is picked when none is given.
    #[serde(default)]
    pub default: bool,
    /// Where the client and the server meet over a local socket.
    ///
    /// Every field has a default, so an account describes a socket
    /// without holding a `socket` table at all. What the table adjusts
    /// is the address, never whether the transport exists: which one a
    /// server binds is what `server start` decides.
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

impl AccountConfig {
    /// Renders the account as the annotated sample writes one: one
    /// cycle per line, `name` before `duration`, and only the fields
    /// that are set.
    ///
    /// Hand-rendered rather than serialized, because the shared
    /// serializer renders a whole array on one line. That suits the
    /// short arrays the other Pimalaya wizards emit, and turns a
    /// six-cycle pomodoro into a two-hundred-column line, in a document
    /// whose whole point is to be edited by hand afterwards.
    ///
    /// This is the direction [`Account`] cannot take: a resolved
    /// account has lost the difference between a field left out and a
    /// field written to today's default, so rendering one would freeze
    /// a runtime socket path into the file.
    ///
    /// Nothing here needs escaping, since the caller is the wizard: the
    /// cycles come from its presets and the name from theirs, which are
    /// bare TOML keys by construction.
    ///
    /// [`Account`]: crate::cli::account::Account
    pub fn render(&self, name: &str) -> String {
        let mut document = format!("[accounts.{name}]\n");

        if self.default {
            document.push_str("default = true\n");
        }

        document.push_str("cycles = [\n");

        for TimerCycle { name, duration } in &self.cycles {
            document.push_str(&format!(
                "  {{ name = \"{name}\", duration = {duration} }},\n"
            ));
        }

        document.push_str("]\n");

        if let Some(count) = self.cycles_count {
            document.push_str(&format!("cycles-count = {count}\n"));
        }

        // NOTE: every transport field has a default, so what is written
        // is what departs from one. An account taking them all holds
        // its cycles and nothing else.
        if self.socket.path != default_socket_path() {
            document.push_str(&format!(
                "socket.path = \"{}\"\n",
                self.socket.path.display()
            ));
        }

        if self.socket.default {
            document.push_str("socket.default = true\n");
        }

        if self.tcp.host != LOCALHOST {
            document.push_str(&format!("tcp.host = \"{}\"\n", self.tcp.host));
        }

        if self.tcp.port != TCP_PORT {
            document.push_str(&format!("tcp.port = {}\n", self.tcp.port));
        }

        if self.tcp.default {
            document.push_str("tcp.default = true\n");
        }

        document
    }
}

/// Where a client and a server meet over a local socket.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SocketConfig {
    /// Whether a command talks over this transport when it names none.
    #[serde(default)]
    pub default: bool,
    /// The socket path to bind and connect to.
    ///
    /// Defaults to comodoro.sock inside `$XDG_RUNTIME_DIR`, or inside
    /// the platform temporary directory when that variable is unset.
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
/// port drives the timer. Hence the loopback host, which is the one
/// default an account should have to overrule deliberately.
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
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
/// The listener is unauthenticated, so an account that names no host
/// stays where only this machine can reach it.
pub const LOCALHOST: &str = "127.0.0.1";

/// The port a `tcp` table falls back to.
pub const TCP_PORT: u16 = 9999;

fn localhost() -> String {
    LOCALHOST.to_string()
}

fn tcp_port() -> u16 {
    TCP_PORT
}
