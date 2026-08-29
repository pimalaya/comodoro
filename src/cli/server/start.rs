//! # Server start command
//!
//! Binds the account transports and runs the timer until interrupted.

use alloc::vec::Vec;

use anyhow::Result;
use clap::Parser;
use log::{debug, info};

use crate::{
    cli::{account::Account, transport::Transport},
    server::std::TimerServer,
};

/// Start the server.
///
/// Binds the given transports and runs the timer in the foreground until
/// interrupted, firing the configured hook on every event it emits.
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

impl TimerServerStartCommand {
    /// Binds the transports, then runs the hook bound to every event the
    /// timer emits, until the server is killed.
    pub fn execute(self, account: &mut Account) -> Result<()> {
        let addresses = account.addresses(&self.transports);
        let events = TimerServer {
            schedule: account.schedule.clone(),
            addresses: addresses.clone(),
        }
        .serve()?;

        for address in addresses {
            info!("timer server listening at {address}");
        }

        while let Ok(event) = events.recv() {
            debug!("received timer event {event:?}");

            let name = event.hook_name();

            let Some(hook) = account.hooks.get_mut(&name) else {
                continue;
            };

            info!("run hook {name}");

            hook.execute();
        }

        Ok(())
    }
}
