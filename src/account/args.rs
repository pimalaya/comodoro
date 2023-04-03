//! This module provides arguments related to the user account config.

use clap::{Arg, ArgMatches};

const ARG_ACCOUNT: &str = "account";

/// Represents the user account name argument. This argument allows
/// the user to select a different account than the default one.
pub fn arg() -> Arg {
    Arg::new(ARG_ACCOUNT)
        .help("Set the account")
        .long("account")
        .short('a')
        .global(true)
        .value_name("STRING")
}

/// Represents the user account name argument parser.
pub fn parse_arg(matches: &ArgMatches) -> Option<&str> {
    matches.get_one::<String>(ARG_ACCOUNT).map(String::as_str)
}
