use std::collections::HashMap;

use anyhow::{bail, Result};
#[cfg(feature = "command")]
use io_process::Command;
use io_process::{coroutines::SpawnThenWaitWithOutput, runtimes::std::handle};
use io_timer::timer::TimerCycle;
use log::{debug, trace};
#[cfg(feature = "notify")]
use notify_rust::Notification;
use serde::{Deserialize, Serialize};

#[cfg(unix)]
use crate::unix_socket::UnixSocketConfig;
use crate::{protocol::Protocol, tcp::TcpConfig};

/// Represents the user config file.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TomlAccountConfig {
    #[serde(default)]
    pub default: bool,

    #[cfg(unix)]
    pub unix_socket: Option<UnixSocketConfig>,
    pub tcp: Option<TcpConfig>,

    pub cycles: Vec<TimerCycle>,
    pub cycles_count: Option<usize>,

    #[serde(default)]
    pub precision: TimerPrecision,

    #[serde(default)]
    pub hooks: HashMap<String, HookConfig>,
}

impl TomlAccountConfig {
    pub fn get_default_protocol(&self) -> Result<Protocol> {
        let mut protocol = None;

        #[cfg(unix)]
        if let Some(sock) = &self.unix_socket {
            if sock.default {
                protocol.replace(Protocol::UnixSocket);
            }
        }

        if let Some(tcp) = &self.tcp {
            if tcp.default {
                protocol.replace(Protocol::Tcp);
            }
        }

        let Some(protocol) = protocol else {
            bail!("no default protocol");
        };

        Ok(protocol)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct HookConfig {
    /// The hook based on shell commands.
    #[cfg(feature = "command")]
    #[serde(alias = "cmd")]
    pub command: Option<Command>,

    /// The hook based on system notifications.
    #[cfg(feature = "notify")]
    pub notify: Option<NotifyConfig>,
}

impl HookConfig {
    pub fn exec(&self) {
        if let Some(cmd) = self.command.as_ref() {
            debug!("execute shell command hook: {cmd:?}");

            let mut arg = None;
            let mut spawn = SpawnThenWaitWithOutput::new(cmd.clone());

            loop {
                match spawn.resume(arg.take()) {
                    Ok(_) => break,
                    Err(io) => arg = Some(handle(io).unwrap()),
                }
            }
        }

        if let Some(notify) = self.notify.as_ref() {
            debug!("execute system notification hook: {notify:?}");

            let notif = Notification::new()
                .summary(&notify.summary)
                .body(&notify.body)
                .show();

            if let Err(err) = notif {
                debug!("error while sending system notification: {err}");
                trace!("{err:?}");
            }
        }
    }
}

/// The configuration of the notify hook variant.
///
/// The structure tries to match the [`notify_rust::Notification`] API
/// and may evolve in the future.
#[cfg(feature = "notify")]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct NotifyConfig {
    /// The summary (or the title) of the notification.
    pub summary: String,

    /// The body of the notification.
    pub body: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum TimerPrecision {
    #[serde(alias = "seconds", alias = "secs", alias = "sec", alias = "s")]
    Second,
    #[default]
    #[serde(alias = "minutes", alias = "mins", alias = "min", alias = "m")]
    Minute,
    #[serde(alias = "hours", alias = "h")]
    Hour,
}
