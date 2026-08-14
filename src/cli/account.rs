//! Merged runtime account, the view every command consumes.
//!
//! [`Account`] is what the configuration becomes once it is resolved:
//! the addresses the client and the server meet on, the timer the
//! server runs, the precision the client renders at, and the hooks
//! bound to the events. Everything a command needs, and nothing that
//! only the document cares about, so `default` is gone: which account a
//! command picks is settled before an account exists.
//!
//! It is built by folding the global [`Config`] then the selected
//! `[accounts.<name>]`, which today reduces to the account alone, since
//! the document carries no global section yet. Defaults are applied
//! here rather than at consumption time, since every one of them
//! resolves against the platform (the socket path) or against a single
//! documented value (the precision) rather than against another layer.
//!
//! [`Config`]: crate::cli::config::Config

use alloc::{string::String, vec, vec::Vec};

use std::collections::HashMap;

use anyhow::{Result, bail};

use crate::{
    cli::{config::AccountConfig, hook::TimerHook, transport::Transport},
    timer::{TimerLoop, TimerPrecision, TimerSchedule},
    transport::TimerAddress,
};

/// One account, resolved for the commands to run against.
#[derive(Debug)]
pub struct Account {
    /// What the server runs: the cycles, and how many loops of them.
    pub schedule: TimerSchedule,
    /// How precisely a client renders the remaining duration.
    pub precision: TimerPrecision,
    /// The hooks to run, by event name.
    pub hooks: HashMap<String, TimerHook>,
    /// The local socket, which every account has.
    pub socket: TimerAddress,
    /// The TCP endpoint, absent when the account opens no port.
    pub tcp: Option<TimerAddress>,
    /// The transport a command talks over when it names none.
    pub default_transport: Transport,
}

impl Account {
    /// The address a client reaches the server at.
    ///
    /// Falls back to [`Account::default_transport`] when the command
    /// names no transport.
    pub fn address(&self, transport: Option<Transport>) -> Result<TimerAddress> {
        match transport.unwrap_or(self.default_transport) {
            Transport::UnixSocket => Ok(self.socket.clone()),
            Transport::Tcp => match &self.tcp {
                Some(tcp) => Ok(tcp.clone()),
                None => bail!("Missing TCP configuration"),
            },
        }
    }

    /// The addresses a server binds.
    ///
    /// Falls back to every transport the account describes when the
    /// command names none, which is the local socket alone unless the
    /// account opens a port.
    pub fn addresses(&self, transports: &[Transport]) -> Result<Vec<TimerAddress>> {
        if transports.is_empty() {
            let mut addresses = vec![self.socket.clone()];

            if let Some(tcp) = &self.tcp {
                addresses.push(tcp.clone());
            }

            return Ok(addresses);
        }

        transports
            .iter()
            .map(|transport| self.address(Some(*transport)))
            .collect()
    }
}

impl From<AccountConfig> for Account {
    fn from(config: AccountConfig) -> Self {
        let AccountConfig {
            default: _,
            socket,
            tcp,
            cycles,
            cycles_count,
            precision,
            hooks,
        } = config;

        // NOTE: the local socket wins a tie, since it is the transport
        // every account has: `tcp.default` is honoured only when the
        // socket does not claim the default itself.
        let default_transport = match &tcp {
            Some(tcp) if tcp.default && !socket.default => Transport::Tcp,
            _ => Transport::UnixSocket,
        };

        Self {
            schedule: TimerSchedule {
                cycles,
                loops: match cycles_count {
                    Some(count) => TimerLoop::Fixed(count),
                    None => TimerLoop::Infinite,
                },
            },
            precision,
            hooks,
            socket: TimerAddress::UnixSocket(socket.path()),
            tcp: tcp.map(|tcp| tcp.address()),
            default_transport,
        }
    }
}
