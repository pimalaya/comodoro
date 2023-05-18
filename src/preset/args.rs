use clap::{Arg, ArgMatches};

const ARG_PRESET: &str = "preset";

pub fn arg() -> Arg {
    Arg::new(ARG_PRESET)
        .help("Use configuration from the given preset name")
        .required(true)
}

pub fn parse_arg(m: &ArgMatches) -> &str {
    m.get_one::<String>(ARG_PRESET).map(String::as_str).unwrap()
}
