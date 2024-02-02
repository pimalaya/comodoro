use anyhow::Result;
use clap::Parser;
use log::info;

use crate::{config::TomlConfig, preset::arg::PresetNameArg, protocol::arg::ProtocolArg};

/// Start the timer.
///
/// This command allows you to send a request to the server in order
/// to start the timer.
#[derive(Debug, Parser)]
pub struct StartTimerCommand {
    #[command(flatten)]
    pub preset: PresetNameArg,

    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl StartTimerCommand {
    pub async fn execute(self, config: &TomlConfig) -> Result<()> {
        info!("executing start timer command");

        let preset = config.get_preset(&self.preset.name)?;
        let client = self.protocol.to_client(&preset)?;

        client.start().await?;

        Ok(())
    }
}
