//! Deserialized config module.
//!
//! This module contains the raw deserialized representation of the
//! user configuration file.

use anyhow::{anyhow, Context, Result};
use dirs::{config_dir, home_dir};
use log::debug;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use toml;

use crate::PresetConfig;

/// Represents the user config file.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(flatten)]
    pub presets: HashMap<String, PresetConfig>,
}

impl Config {
    /// Tries to create a config from an optional path.
    pub fn from_opt_path(path: Option<&str>) -> Result<Self> {
        debug!("path: {:?}", path);

        let config: Self = match path.map(|s| s.into()).or_else(Self::path) {
            Some(path) => {
                let content = fs::read_to_string(path).context("cannot read config file")?;
                toml::from_str(&content).context("cannot parse config file")?
            }
            None => return Err(anyhow!("cannot find config file")),
        };

        debug!("comodoro config: {:#?}", config);
        Ok(config)
    }

    /// Finds the preset configuration associated to the given
    /// name. If no preset found, returns an error.
    pub fn get_preset(&self, name: &str) -> Result<PresetConfig> {
        self.presets
            .iter()
            .find_map(|(preset_name, preset)| {
                if preset_name == name {
                    Some(preset)
                } else {
                    None
                }
            })
            .cloned()
            .ok_or_else(|| anyhow!("cannot find preset {name}"))
    }

    /// Tries to return a config path from a few default settings.
    ///
    /// Tries paths in this order:
    ///
    /// - `"$XDG_CONFIG_DIR/comodoro/config.toml"` (or equivalent to `$XDG_CONFIG_DIR` in  other
    ///   OSes.)
    /// - `"$HOME/.config/comodoro/config.toml"`
    /// - `"$HOME/.comodororc"`
    ///
    /// Returns `Some(path)` if the path exists, otherwise `None`.
    pub fn path() -> Option<PathBuf> {
        config_dir()
            .map(|p| p.join("comodoro").join("config.toml"))
            .filter(|p| p.exists())
            .or_else(|| home_dir().map(|p| p.join(".config").join("comodoro").join("config.toml")))
            .filter(|p| p.exists())
            .or_else(|| home_dir().map(|p| p.join(".comodororc")))
            .filter(|p| p.exists())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::{collections::HashMap, io::prelude::*};
    use tempfile::NamedTempFile;
    use time::TimerCycle;

    use crate::{Config, PresetConfig, PresetKind, PresetKindOrCyclesConfig};

    fn make_config(config: &str) -> Result<Config> {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", config).unwrap();
        Config::from_opt_path(file.into_temp_path().to_str())
    }

    #[test]
    fn empty_config() {
        let config = make_config("");
        assert_eq!(config.unwrap(), Config::default());
    }

    #[test]
    fn empty_preset_config() {
        let config = make_config("[preset-1]");

        assert!(config
            .unwrap_err()
            .root_cause()
            .to_string()
            .contains("no variant of enum PresetKindOrCyclesConfig found in flattened data"));
    }

    #[test]
    fn pomodoro_preset_config() {
        let config = make_config(
            "[preset-1]
            preset = \"pomodoro\"",
        );

        assert_eq!(
            config.unwrap(),
            Config {
                presets: HashMap::from_iter([(
                    String::from("preset-1"),
                    PresetConfig {
                        preset_or_cycles: PresetKindOrCyclesConfig::Preset(
                            PresetKind::PresetPomodoro
                        ),
                        tcp: None,
                        hooks: HashMap::default(),
                        cycles_count: Default::default(),
                        timer_precision: Default::default(),
                    }
                )]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn _52_17_preset_config() {
        let config = make_config(
            "[preset-1]
            preset = \"52/17\"",
        );

        assert_eq!(
            config.unwrap(),
            Config {
                presets: HashMap::from_iter([(
                    String::from("preset-1"),
                    PresetConfig {
                        preset_or_cycles: PresetKindOrCyclesConfig::Preset(PresetKind::Preset52_17),
                        tcp: None,
                        hooks: HashMap::default(),
                        cycles_count: Default::default(),
                        timer_precision: Default::default(),
                    }
                )]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn cycles_preset_config() {
        let config = make_config(
            "[preset-1]

            [[preset-1.cycles]]
            name = \"work\"
            duration = 10

            [[preset-1.cycles]]
            name = \"rest\"
            duration = 5",
        );

        assert_eq!(
            config.unwrap(),
            Config {
                presets: HashMap::from_iter([(
                    String::from("preset-1"),
                    PresetConfig {
                        preset_or_cycles: PresetKindOrCyclesConfig::Cycles(vec![
                            TimerCycle::new("work", 10),
                            TimerCycle::new("rest", 5)
                        ]),
                        tcp: None,
                        // FIXME: preset is also captured by hooks, serde bug?
                        hooks: HashMap::default(),
                        cycles_count: Default::default(),
                        timer_precision: Default::default(),
                    }
                )]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn hooks_preset_config() {
        let config = make_config(
            "[preset-1]
            on-timer-start = \"hook-1\"
            on-server-stop = \"hook-2\"

            [[preset-1.cycles]]
            name = \"timer\"
            duration = 10",
        );

        assert_eq!(
            config.unwrap(),
            Config {
                presets: HashMap::from_iter([(
                    String::from("preset-1"),
                    PresetConfig {
                        preset_or_cycles: PresetKindOrCyclesConfig::Cycles(vec![TimerCycle::new(
                            "timer", 10
                        )]),
                        tcp: None,
                        hooks: HashMap::from_iter([
                            (String::from("on-timer-start"), String::from("hook-1")),
                            (String::from("on-server-stop"), String::from("hook-2"))
                        ]),
                        cycles_count: Default::default(),
                        timer_precision: Default::default(),
                    }
                )]),
                ..Default::default()
            }
        );
    }
}
