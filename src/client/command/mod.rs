mod get;
mod pause;
mod resume;
mod start;
mod stop;

use clap::Subcommand;
use color_eyre::Result;
use pimalaya_tui::terminal::cli::printer::Printer;

use crate::config::TomlConfig;

use self::{
    get::GetTimerCommand, pause::PauseTimerCommand, resume::ResumeTimerCommand,
    start::StartTimerCommand, stop::StopTimerCommand,
};

/// Manage timers.
///
/// A client connects to a server and sends requests to control a
/// timer.
#[derive(Debug, Subcommand)]
pub enum TimerSubcommand {
    #[command()]
    Start(StartTimerCommand),

    #[command()]
    Get(GetTimerCommand),

    #[command()]
    Pause(PauseTimerCommand),

    #[command()]
    Resume(ResumeTimerCommand),

    #[command()]
    Stop(StopTimerCommand),
}

impl TimerSubcommand {
    pub async fn execute(self, printer: &mut impl Printer, config: &TomlConfig) -> Result<()> {
        match self {
            Self::Start(cmd) => cmd.execute(config).await,
            Self::Get(cmd) => cmd.execute(printer, config).await,
            Self::Pause(cmd) => cmd.execute(config).await,
            Self::Resume(cmd) => cmd.execute(config).await,
            Self::Stop(cmd) => cmd.execute(config).await,
        }
    }
}
