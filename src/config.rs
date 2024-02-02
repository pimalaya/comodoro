use anyhow::{anyhow, bail, Context, Result};
use dirs::{config_dir, home_dir};
use serde::{Deserialize, Serialize};
use shellexpand_utils::{canonicalize, expand};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use toml;

use crate::preset::config::PresetConfig;

/// Represents the user config file.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TomlConfig {
    pub presets: HashMap<String, PresetConfig>,
}

impl TomlConfig {
    /// Read and parse the TOML configuration at the given path.
    ///
    /// Returns an error if the configuration file cannot be read or
    /// if its content cannot be parsed.
    fn from_path(path: &Path) -> Result<Self> {
        let content =
            fs::read_to_string(path).context(format!("cannot read config file at {path:?}"))?;
        toml::from_str(&content).context(format!("cannot parse config file at {path:?}"))
    }

    /// Read and parse the TOML configuration from default paths.
    pub async fn from_default_paths() -> Result<Self> {
        match Self::first_valid_default_path() {
            Some(path) => Self::from_path(&path),
            None => bail!("cannot find configuration file from default locations"),
        }
    }

    /// Read and parse the TOML configuration at the optional given
    /// path.
    ///
    /// If the given path exists, then read and parse the TOML
    /// configuration from it.
    ///
    /// If the given path does not exist, then create it using the
    /// wizard.
    ///
    /// If no path is given, then either read and parse the TOML
    /// configuration at the first valid default path, otherwise
    /// create it using the wizard.  wizard.
    pub async fn from_some_path_or_default(path: Option<impl Into<PathBuf>>) -> Result<Self> {
        match path.map(Into::into) {
            Some(ref path) if path.exists() => Self::from_path(path),
            _ => Self::from_default_paths().await,
        }
    }

    /// Get the default configuration path.
    ///
    /// Returns an error if the XDG configuration directory cannot be
    /// found.
    pub fn default_path() -> Result<PathBuf> {
        Ok(config_dir()
            .ok_or(anyhow!("cannot get XDG config directory"))?
            .join("comodoro")
            .join("config.toml"))
    }

    /// Get the first default configuration path that points to a
    /// valid file.
    ///
    /// Tries paths in this order:
    ///
    /// - `$XDG_CONFIG_DIR/comodoro/config.toml` (or equivalent to
    ///   `$XDG_CONFIG_DIR` in other OSes.)
    /// - `$HOME/.config/comodoro/config.toml`
    /// - `$HOME/.comodororc`
    pub fn first_valid_default_path() -> Option<PathBuf> {
        Self::default_path()
            .ok()
            .filter(|p| p.exists())
            .or_else(|| home_dir().map(|p| p.join(".config").join("comodoro").join("config.toml")))
            .filter(|p| p.exists())
            .or_else(|| home_dir().map(|p| p.join(".comodororc")))
            .filter(|p| p.exists())
    }

    /// Finds the preset configuration associated to the given
    /// name.
    ///
    /// If no preset is found, returns an error.
    pub fn get_preset(&self, name: &str) -> Result<Arc<PresetConfig>> {
        let preset = self
            .presets
            .iter()
            .find_map(|(preset_name, preset)| {
                if preset_name == name {
                    Some(preset)
                } else {
                    None
                }
            })
            .cloned()
            .ok_or_else(|| anyhow!("cannot find preset {name}"))?;

        Ok(Arc::new(preset))
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::{collections::HashMap, io::prelude::*};
    use tempfile::NamedTempFile;
    use time::timer::TimerCycle;

    use crate::{
        config::TomlConfig,
        preset::config::{PresetConfig, PresetKind, PresetKindOrCyclesConfig},
    };

    fn make_config(config: &str) -> Result<TomlConfig> {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", config).unwrap();
        TomlConfig::from_opt_path(file.into_temp_path().to_str())
    }

    #[test]
    fn empty_config() {
        let config = make_config("");
        assert_eq!(config.unwrap(), TomlConfig::default());
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
            TomlConfig {
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
            TomlConfig {
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
            TomlConfig {
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
            TomlConfig {
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

/// Parse a configuration file path as [`PathBuf`].
///
/// The path is shell-expanded then canonicalized (if applicable).
pub fn path_parser(path: &str) -> Result<PathBuf, String> {
    expand::try_path(path)
        .map(canonicalize::path)
        .map_err(|err| err.to_string())
}
