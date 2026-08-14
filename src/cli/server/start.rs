//! Command starting the timer server.

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
/// This command binds the account transports and runs the timer until
/// interrupted, firing the configured hook on every event.
#[derive(Debug, Parser)]
pub struct TimerServerStartCommand {
    /// The transports the server accepts requests on.
    ///
    /// Defaults to every transport the account configures, which is the
    /// local socket alone unless a `tcp` table is present.
    #[arg(name = "transports", value_name = "TRANSPORTS")]
    pub transports: Vec<Transport>,
}

impl TimerServerStartCommand {
    /// Binds the transports, then runs the hook bound to every event
    /// the timer emits, until the server is killed.
    pub fn execute(self, account: &mut Account) -> Result<()> {
        let addresses = account.addresses(&self.transports)?;
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
