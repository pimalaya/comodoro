//! # Account
//!
//! The merged runtime account, the view every command consumes.
//!
//! [`Account`] is what the configuration becomes once resolved: the
//! addresses the two sides meet on, the timer the server runs, the
//! precision the client renders at, and the hooks bound to the events.
//!
//! `default` is gone, which account a command picks being settled before
//! an account exists. The rest is built by folding the global [`Config`]
//! then the selected `[accounts.<name>]`, which today reduces to the
//! account alone, the document carrying no global section yet.
//!
//! Defaults are applied here rather than at consumption time, since each
//! resolves against the platform (the socket path) or against a single
//! documented value (the precision) rather than against another layer.
//!
//! [`Config`]: crate::cli::config::Config

use alloc::{string::String, vec, vec::Vec};

use std::collections::HashMap;

use crate::{
    cli::{config::AccountConfig, hook::TimerHook, transport::Transport},
    timer::{TimerLoop, TimerPrecision, TimerSchedule},
    transport::TimerAddress,
};

/// One account, resolved for the commands to run against.
#[derive(Debug)]
pub struct Account {
    /// What the server runs: the cycles, and how many loops.
    pub schedule: TimerSchedule,
    /// How precisely a client renders the remaining duration.
    pub precision: TimerPrecision,
    /// The hooks to run, by event name.
    pub hooks: HashMap<String, TimerHook>,
    /// Where the local socket is.
    pub socket: TimerAddress,
    /// Where the TCP endpoint is.
    pub tcp: TimerAddress,
    /// The transport a command talks over when it names none.
    pub default_transport: Transport,
}

impl Account {
    /// The address of the given transport, or of the default one when the
    /// command names none.
    ///
    /// Every transport has an address, every field describing one having
    /// a default, so naming a transport picks which address a command
    /// talks over, never whether that transport exists.
    pub fn address(&self, transport: Option<Transport>) -> TimerAddress {
        match transport.unwrap_or(self.default_transport) {
            Transport::UnixSocket => self.socket.clone(),
            Transport::Tcp => self.tcp.clone(),
        }
    }

    /// The addresses a server binds, one per transport it was given.
    ///
    /// Given none it binds the default transport alone, so no socket
    /// appears under an account meant for TCP and no port opens under one
    /// meant for the socket. Binding both is asking for both.
    pub fn addresses(&self, transports: &[Transport]) -> Vec<TimerAddress> {
        if transports.is_empty() {
            return vec![self.address(None)];
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

        // NOTE: the local socket wins the tie, since it is the transport
        // that opens no port: `tcp.default` is honoured only when the
        // socket does not claim the default itself.
        let default_transport = if tcp.default && !socket.default {
            Transport::Tcp
        } else {
            Transport::UnixSocket
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
            socket: socket.address(),
            tcp: tcp.address(),
            default_transport,
        }
    }
}
