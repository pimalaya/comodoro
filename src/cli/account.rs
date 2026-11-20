//! # Account
//!
//! The `account` subcommand tree, listing the accounts a configuration
//! declares and checking one, and the merged runtime account, the view
//! every command consumes.
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

pub mod check;
pub mod list;

use alloc::{format, string::String, vec, vec::Vec};

use std::collections::HashMap;

use anyhow::{Result, bail};
use clap::Subcommand;
use pimalaya_cli::printer::Printer;

use crate::{
    cli::{
        Session,
        account::{check::AccountCheckCommand, list::AccountListCommand},
        config::AccountConfig,
        hook::TimerHook,
        transport::Transport,
    },
    timer::{TimerCycle, TimerLoop, TimerPrecision, TimerSchedule},
    transport::TimerAddress,
};

/// Manage the accounts of the configuration.
///
/// An account is a timer: its cycles, the transports its server and
/// clients meet on, and its hooks. These subcommands list the accounts
/// and check one; creating one means adding an `[accounts.<name>]`
/// table, as config.sample.toml shows.
#[derive(Debug, Subcommand)]
pub enum AccountCommand {
    /// List the accounts of the configuration.
    List(AccountListCommand),
    /// Check that a timer server answers on every transport the account
    /// declares.
    Check(AccountCheckCommand),
}

impl AccountCommand {
    /// Dispatches the account leaf: the list reads the configuration, the
    /// check runs over the session's account.
    pub fn execute(self, printer: &mut impl Printer, session: &mut Session) -> Result<()> {
        match self {
            Self::List(cmd) => cmd.execute(printer, session.config_paths),
            Self::Check(cmd) => cmd.execute(printer, session.account()?),
        }
    }
}

/// One account, resolved for the commands to run against.
#[derive(Debug)]
pub struct Account {
    /// The name of its `[accounts.<name>]` table.
    pub name: String,
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
    /// The transports the account declares, the socket first, which
    /// `account check` reaches.
    pub transports: Vec<Transport>,
}

impl Account {
    /// Resolves the account `name` from its configuration, refusing one
    /// with no cycle and one whose hooks no event fires or name a
    /// variable their event does not fill.
    pub fn new(name: String, config: AccountConfig) -> Result<Self> {
        let default_transport = config.default_transport();
        let transports = config.transports();

        let AccountConfig {
            default: _,
            socket,
            tcp,
            cycles,
            cycles_count,
            precision,
            hooks,
        } = config;

        let cycles: Vec<TimerCycle> = cycles.into_iter().map(TimerCycle::from).collect();

        if cycles.is_empty() {
            bail!("The account has no cycle, give `cycles` at least one");
        }

        for (hook_name, hook) in &hooks {
            hook.validate(hook_name, &cycles)?;
        }

        Ok(Self {
            name,
            schedule: TimerSchedule {
                cycles,
                loops: match cycles_count {
                    Some(count) => TimerLoop::Fixed(count),
                    None => TimerLoop::Infinite,
                },
            },
            precision,
            hooks,
            socket: socket.unwrap_or_default().address(),
            tcp: tcp.unwrap_or_default().address(),
            default_transport,
            transports,
        })
    }

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
