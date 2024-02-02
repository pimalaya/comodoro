use anyhow::Result;
use clap::Parser;
use log::info;

use crate::{config::TomlConfig, preset::arg::PresetNameArg, protocol::arg::ProtocolArg};

/// Stop the timer.
///
/// This command allows you to send a request to the server in order
/// to stop the timer.
#[derive(Debug, Parser)]
pub struct StopTimerCommand {
    #[command(flatten)]
    pub preset: PresetNameArg,

    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl StopTimerCommand {
    pub async fn execute(self, config: &TomlConfig) -> Result<()> {
        info!("executing stop timer command");

        let preset = config.get_preset(&self.preset.name)?;
        let client = self.protocol.to_client(&preset)?;

        client.stop().await?;

        Ok(())
    }
}
