use anyhow::Result;
use clap::Command;
use comodoro::{
    account, client, compl,
    config::{self, DeserializedConfig},
    man, server,
};
use std::env;

fn create_app() -> Command {
    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .propagate_version(true)
        .infer_subcommands(true)
        .arg(config::args::arg())
        .arg(account::args::arg())
        .subcommands(client::args::subcmds())
        .subcommand(compl::args::subcmd())
        .subcommand(man::args::subcmd())
        .subcommand(server::args::subcmd())
}

#[allow(clippy::single_match)]
fn main() -> Result<()> {
    let default_env_filter = env_logger::DEFAULT_FILTER_ENV;
    env_logger::init_from_env(env_logger::Env::default().filter_or(default_env_filter, "off"));

    let app = create_app();
    let m = app.get_matches();

    // checks completion command before configs
    match compl::args::matches(&m)? {
        Some(compl::args::Cmd::Generate(shell)) => {
            return compl::handlers::generate(create_app(), shell);
        }
        _ => (),
    }

    // also checks man command before configs
    match man::args::matches(&m)? {
        Some(man::args::Cmd::GenerateAll(dir)) => {
            return man::handlers::generate(dir, create_app());
        }
        _ => (),
    }

    // inits config
    let config = DeserializedConfig::from_opt_path(config::args::parse_arg(&m))?;
    let _account_config = config.to_account_config(account::args::parse_arg(&m))?;

    // checks server commands
    match server::args::matches(&m)? {
        Some(server::args::Cmd::Start) => {
            return server::handlers::start();
        }
        Some(server::args::Cmd::Stop) => {
            return server::handlers::stop();
        }
        _ => (),
    }

    // checks client commands
    match client::args::matches(&m)? {
        Some(client::args::Cmd::Start) => {
            return client::handlers::start();
        }
        Some(client::args::Cmd::Get) => {
            return client::handlers::get();
        }
        Some(client::args::Cmd::Pause) => {
            return client::handlers::pause();
        }
        Some(client::args::Cmd::Resume) => {
            return client::handlers::resume();
        }
        Some(client::args::Cmd::Stop) => {
            return client::handlers::stop();
        }
        _ => (),
    }

    Ok(())
}
