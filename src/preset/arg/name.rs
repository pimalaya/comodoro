use clap::Parser;

/// The preset name argument parser.
#[derive(Debug, Parser)]
pub struct PresetNameArg {
    /// Name of the configuration preset.
    ///
    /// Use configuration matching the given preset name from the
    /// configuration file.
    #[arg(name = "preset_name", value_name = "PRESET")]
    pub name: String,
}
