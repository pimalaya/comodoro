use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[cfg(feature = "client")]
use crate::client::command::TimerSubcommand;
#[cfg(feature = "server")]
use crate::server::command::ServerSubcommand;
#[allow(unused)]
use crate::{
    completion::command::CompletionGenerateCommand,
    config::{self, TomlConfig},
    manual::command::ManualGenerateCommand,
};

#[derive(Parser, Debug)]
#[command(name = "comodoro", author, version, about)]
#[command(propagate_version = true, infer_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: ComodoroCommand,

    /// Override the default configuration file path.
    ///
    /// The given path is shell-expanded then canonicalized (if
    /// applicable).
    #[arg(short, long = "config", global = true)]
    #[arg(value_name = "PATH", value_parser = config::path_parser)]
    pub config_path: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum ComodoroCommand {
    #[cfg(feature = "client")]
    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    #[command(alias = "client")]
    Timer(TimerSubcommand),

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
    pub async fn execute(self, config_path: Option<&PathBuf>) -> Result<()> {
        match self {
            #[cfg(feature = "client")]
            Self::Timer(cmd) => {
                let config = TomlConfig::from_some_path_or_default(config_path).await?;
                cmd.execute(&config).await
            }
            #[cfg(feature = "server")]
            Self::Server(cmd) => {
                let config = TomlConfig::from_some_path_or_default(config_path).await?;
                cmd.execute(&config).await
            }
            Self::Manual(cmd) => cmd.execute().await,
            Self::Completion(cmd) => cmd.execute().await,
        }
    }
}
