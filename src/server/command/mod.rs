mod start;

use anyhow::Result;
use clap::Subcommand;

use crate::config::TomlConfig;

use self::start::StartServerCommand;

/// Manage servers.
///
/// A server controls a timer, and receive requests from clients to
/// manipulate the timer.
#[derive(Debug, Subcommand)]
pub enum ServerSubcommand {
    Start(StartServerCommand),
}

impl ServerSubcommand {
    pub fn execute(self, config: &TomlConfig) -> Result<()> {
        match self {
            Self::Start(cmd) => cmd.execute(config),
        }
    }
}
