mod start;

use clap::Subcommand;
use color_eyre::Result;

use crate::config::TomlConfig;

use self::start::StartServerCommand;

/// Manage servers.
///
/// A server controls a timer, and receive requests from clients to
/// manipulate the timer.
#[derive(Debug, Subcommand)]
pub enum ServerSubcommand {
    #[command()]
    Start(StartServerCommand),
}

impl ServerSubcommand {
    pub async fn execute(self, config: &TomlConfig) -> Result<()> {
        match self {
            Self::Start(cmd) => cmd.execute(config).await,
        }
    }
}
