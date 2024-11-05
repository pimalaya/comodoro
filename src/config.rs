use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use color_eyre::{eyre::eyre, Result};
use serde::{Deserialize, Serialize};

use crate::preset::config::TomlPresetConfig;

/// Represents the user config file.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TomlConfig {
    pub presets: HashMap<String, TomlPresetConfig>,
}

#[async_trait]
impl pimalaya_tui::terminal::config::TomlConfig for TomlConfig {
    type TomlAccountConfig = TomlPresetConfig;

    fn project_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn get_default_account_config(&self) -> Option<(String, Self::TomlAccountConfig)> {
        None
    }

    fn get_account_config(&self, name: &str) -> Option<(String, Self::TomlAccountConfig)> {
        self.presets
            .get(name)
            .map(|preset| (name.to_owned(), preset.clone()))
    }
}

impl TomlConfig {
    /// Finds the preset configuration associated to the given
    /// name.
    ///
    /// If no preset is found, returns an error.
    pub fn get_preset(&self, name: &str) -> Result<Arc<TomlPresetConfig>> {
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
            .ok_or_else(|| eyre!("cannot find preset {name}"))?;

        Ok(Arc::new(preset))
    }
}
