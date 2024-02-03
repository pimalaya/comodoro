use anyhow::Result;
use clap::Parser;
use comodoro::cli::Cli;
use env_logger::{Builder as LoggerBuilder, Env, DEFAULT_FILTER_ENV};
use log::{debug, warn};

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(not(target_os = "windows"))]
    if let Err((_, err)) = coredump::register_panic_handler() {
        warn!("cannot register coredump panic handler");
        debug!("{err:?}");
    }

    LoggerBuilder::new()
        .parse_env(Env::new().filter_or(DEFAULT_FILTER_ENV, "warn"))
        .format_timestamp(None)
        .init();

    let cli = Cli::parse();

    cli.command.execute(cli.config_path.as_ref()).await
}
