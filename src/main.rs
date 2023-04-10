use anyhow::Result;
use clap::Command;
use comodoro::{
    client, compl,
    config::{self, Config},
    man, server, Protocol,
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

    // init config
    let config = Config::from_opt_path(config::args::parse_arg(&m))?;

    // check server commands
    match server::args::matches(&m)? {
        Some(server::args::Cmd::Start(protocols)) => {
            let binders = Protocol::to_binders(&config, protocols);
            return server::handlers::start(binders);
        }
        _ => (),
    }

    // checks client commands
    match client::args::matches(&m)? {
        Some(client::args::Cmd::Start(protocol)) => {
            let client = protocol.to_client(&config)?;
            return client::handlers::start(client.as_ref());
        }
        Some(client::args::Cmd::Get(protocol)) => {
            let client = protocol.to_client(&config)?;
            return client::handlers::get(client.as_ref());
        }
        Some(client::args::Cmd::Pause(protocol)) => {
            let client = protocol.to_client(&config)?;
            return client::handlers::pause(client.as_ref());
        }
        Some(client::args::Cmd::Resume(protocol)) => {
            let client = protocol.to_client(&config)?;
            return client::handlers::resume(client.as_ref());
        }
        Some(client::args::Cmd::Stop(protocol)) => {
            let client = protocol.to_client(&config)?;
            return client::handlers::stop(client.as_ref());
        }
        _ => (),
    }

    Ok(())
}
