use clap::Parser;
use color_eyre::eyre::Result;
use comodoro::cli::Cli;
use pimalaya_tui::terminal::cli::{printer::StdoutPrinter, tracing};

#[tokio::main]
async fn main() -> Result<()> {
    let tracing = tracing::install()?;

    let cli = Cli::parse();
    let mut printer = StdoutPrinter::new(cli.output);
    let res = cli
        .command
        .execute(&mut printer, cli.config_paths.as_ref())
        .await;

    tracing.with_debug_and_trace_notes(res)
}
