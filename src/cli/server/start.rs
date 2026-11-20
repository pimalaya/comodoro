//! # Server start command
//!
//! Binds the account transports and runs the timer until a termination
//! signal stops it with one result, its sockets removed.

use alloc::vec::Vec;
use core::{fmt, time::Duration};

use std::{fs, io::ErrorKind, sync::mpsc::RecvTimeoutError};

use anyhow::{Result, bail};
use clap::Parser;
use log::{debug, info, warn};
use pimalaya_cli::{printer::Printer, signal::Shutdown};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    cli::{account::Account, transport::Transport},
    server::std::TimerServer,
    transport::TimerAddress,
};

/// How often the server looks whether a signal asked it to stop.
const SHUTDOWN_POLL: Duration = Duration::from_millis(100);

/// Start the server.
///
/// Binds the given transports and runs the timer in the foreground,
/// firing the configured hook on every event it emits, until a
/// termination signal stops it: it then removes its socket, prints what
/// stopped it and exits 0.
#[derive(Debug, Parser)]
pub struct TimerServerStartCommand {
    /// The transports the server accepts requests on.
    ///
    /// Defaults to the one the account marks as default, the local socket
    /// unless `tcp.default` says otherwise. Name both to serve the same
    /// timer over both at once.
    #[arg(name = "transports", value_name = "TRANSPORTS")]
    pub transports: Vec<Transport>,
}

/// What stopped a server that ended cleanly.
#[derive(Clone, Copy, Debug, Eq, PartialEq, JsonSchema, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimerServerStop {
    /// A termination signal: SIGINT, SIGTERM or SIGHUP, or Ctrl-C.
    Signal,
}

/// The result `server start` prints once it stops.
#[derive(Clone, Debug, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerServerStartOutput {
    /// What stopped the server.
    pub stopped_by: TimerServerStop,
}

impl fmt::Display for TimerServerStartOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.stopped_by {
            TimerServerStop::Signal => writeln!(f, "Server stopped"),
        }
    }
}

impl TimerServerStartCommand {
    /// Binds the transports, then runs the hook bound to every event the
    /// timer emits, until a signal stops the server, whose sockets it
    /// removes before printing the result.
    pub fn execute(self, printer: &mut impl Printer, account: &mut Account) -> Result<()> {
        let shutdown = Shutdown::install()?;

        let addresses = account.addresses(&self.transports);
        let events = TimerServer {
            schedule: account.schedule.clone(),
            addresses: addresses.clone(),
        }
        .serve()?;

        for address in &addresses {
            info!("timer server listening at {address}");
        }

        while !shutdown.is_requested() {
            let event = match events.recv_timeout(SHUTDOWN_POLL) {
                Ok(event) => event,
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) => {
                    bail!("Cannot run the timer: it stopped emitting events")
                }
            };

            debug!("received timer event {event:?}");

            let name = event.hook_name();

            let Some(hook) = account.hooks.get(&name) else {
                continue;
            };

            info!("run hook {name}");

            hook.execute(&event.hook_variables());
        }

        for address in &addresses {
            let TimerAddress::UnixSocket(path) = address else {
                continue;
            };

            info!("remove socket {address}");

            match fs::remove_file(path) {
                Ok(()) => (),
                Err(err) if err.kind() == ErrorKind::NotFound => (),
                Err(err) => warn!("cannot remove socket {address}: {err}"),
            }
        }

        let output = TimerServerStartOutput {
            stopped_by: TimerServerStop::Signal,
        };

        printer.out(output)
    }
}
