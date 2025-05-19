use clap::Parser;

/// The account name argument parser.
#[derive(Debug, Parser)]
pub struct AccountNameArg {
    /// Name of the account configuration.
    ///
    /// Uses configuration matching the given account name from the
    /// configuration file.
    #[arg(long = "account", short = 'a')]
    #[arg(name = "account-name", value_name = "NAME")]
    pub name: Option<String>,
}
