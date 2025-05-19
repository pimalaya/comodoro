use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use pimalaya_tui::{
    long_version,
    terminal::{
        cli::{arg::path_parser, printer::Printer},
        config::TomlConfig as _,
    },
};

#[cfg(feature = "client")]
use crate::client::command::{
    get::GetTimerCommand, pause::PauseTimerCommand, resume::ResumeTimerCommand,
    start::StartTimerCommand, stop::StopTimerCommand,
};
#[cfg(feature = "server")]
use crate::server::command::ServerSubcommand;
use crate::{
    completion::command::CompletionGenerateCommand, config::TomlConfig,
    manual::command::ManualGenerateCommand,
};

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about)]
#[command(long_version = long_version!())]
#[command(propagate_version = true, infer_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: ComodoroCommand,

    /// Override the default configuration file path.
    ///
    /// The given paths are shell-expanded then canonicalized (if
    /// applicable). If the first path does not point to a valid file,
    /// the wizard will propose to assist you in the creation of the
    /// configuration file. Other paths are merged with the first one,
    /// which allows you to separate your public config from your
    /// private(s) one(s).
    #[arg(short, long = "config", global = true, env = "COMODORO_CONFIG")]
    #[arg(value_name = "PATH", value_parser = path_parser)]
    pub config_paths: Vec<PathBuf>,

    /// Enable JSON output.
    ///
    /// When set, command output (data and errors) is displayed as
    /// JSON string.
    #[arg(long, global = true)]
    pub json: bool,

    /// Enable debug logs.
    ///
    /// Same as running command with `RUST_LOG=debug` environment
    /// variable.
    #[arg(long, global = true, conflicts_with = "trace")]
    pub debug: bool,

    /// Enable verbose trace logs with backtrace.
    ///
    /// Same as running command with `RUST_LOG=trace` and
    /// `RUST_BACKTRACE=1` environment variables.
    #[arg(long, global = true, conflicts_with = "debug")]
    pub trace: bool,
}

#[derive(Subcommand, Debug)]
pub enum ComodoroCommand {
    #[cfg(feature = "client")]
    Start(StartTimerCommand),
    #[cfg(feature = "client")]
    Get(GetTimerCommand),
    #[cfg(feature = "client")]
    Pause(PauseTimerCommand),
    #[cfg(feature = "client")]
    Resume(ResumeTimerCommand),
    #[cfg(feature = "client")]
    Stop(StopTimerCommand),

    #[cfg(feature = "server")]
    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    #[command(alias = "servers", alias = "srvs", alias = "srv")]
    Server(ServerSubcommand),

    #[command(arg_required_else_help = true)]
    #[command(alias = "manuals", alias = "mans")]
    Manual(ManualGenerateCommand),

    #[command(arg_required_else_help = true)]
    #[command(alias = "completions")]
    Completion(CompletionGenerateCommand),
}

impl ComodoroCommand {
    pub fn execute(self, printer: &mut impl Printer, config_paths: &[PathBuf]) -> Result<()> {
        match self {
            #[cfg(feature = "client")]
            Self::Start(cmd) => {
                let config = TomlConfig::from_paths_or_default(config_paths)?;
                cmd.execute(printer, &config)
            }
            #[cfg(feature = "client")]
            Self::Get(cmd) => {
                let config = TomlConfig::from_paths_or_default(config_paths)?;
                cmd.execute(printer, &config)
            }
            #[cfg(feature = "client")]
            Self::Pause(cmd) => {
                let config = TomlConfig::from_paths_or_default(config_paths)?;
                cmd.execute(printer, &config)
            }
            #[cfg(feature = "client")]
            Self::Resume(cmd) => {
                let config = TomlConfig::from_paths_or_default(config_paths)?;
                cmd.execute(printer, &config)
            }
            #[cfg(feature = "client")]
            Self::Stop(cmd) => {
                let config = TomlConfig::from_paths_or_default(config_paths)?;
                cmd.execute(printer, &config)
            }
            #[cfg(feature = "server")]
            Self::Server(cmd) => {
                let config = TomlConfig::from_paths_or_default(config_paths)?;
                cmd.execute(&config)
            }
            Self::Manual(cmd) => cmd.execute(printer),
            Self::Completion(cmd) => cmd.execute(printer),
        }
    }
}
