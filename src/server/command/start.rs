use clap::Parser;
use color_eyre::Result;
use tracing::info;

use crate::{
    config::TomlConfig,
    preset::arg::PresetNameArg,
    protocol::{arg::ProtocolsArg, Protocol},
};

/// Start the server.
///
/// This command allows you to start the server using the given
/// configuration preset and protocols.
#[derive(Debug, Parser)]
pub struct StartServerCommand {
    #[command(flatten)]
    pub preset: PresetNameArg,

    #[command(flatten)]
    pub protocols: ProtocolsArg,
}

impl StartServerCommand {
    pub async fn execute(self, config: &TomlConfig) -> Result<()> {
        info!("executing start server command");

        let preset = config.get_preset(&self.preset.name)?;
        let server = Protocol::into_server(preset, self.protocols.into()).await?;

        server.bind().await?;

        Ok(())
    }
}
