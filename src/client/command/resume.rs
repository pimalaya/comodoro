use anyhow::Result;
use clap::Parser;
use log::info;

use crate::{config::TomlConfig, preset::arg::PresetNameArg, protocol::arg::ProtocolArg};

/// Resume the timer.
///
/// This command allows you to send a request to the server in order
/// to resume the timer.
#[derive(Debug, Parser)]
pub struct ResumeTimerCommand {
    #[command(flatten)]
    pub preset: PresetNameArg,

    #[command(flatten)]
    pub protocol: ProtocolArg,
}

impl ResumeTimerCommand {
    pub async fn execute(self, config: &TomlConfig) -> Result<()> {
        info!("executing resume timer command");

        let preset = config.get_preset(&self.preset.name)?;
        let client = self.protocol.to_client(&preset)?;

        client.resume().await?;

        Ok(())
    }
}
