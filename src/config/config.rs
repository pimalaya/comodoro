//! Deserialized config module.
//!
//! This module contains the raw deserialized representation of the
//! user configuration file.

use anyhow::{anyhow, Context, Result};
use dirs::{config_dir, home_dir};
use log::{debug, trace};
use pimalaya::time::pomodoro::AccountConfig;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use toml;

use crate::account::DeserializedAccountConfig;

/// Represents the user config file.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeserializedConfig {
    pub work_duration: Option<usize>,
    pub short_break_duration: Option<usize>,
    pub long_break_duration: Option<usize>,
    #[serde(flatten)]
    pub accounts: HashMap<String, DeserializedAccountConfig>,
}

impl DeserializedConfig {
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

        if config.accounts.is_empty() {
            return Err(anyhow!("config file must contain at least one account"));
        }

        trace!("config: {:#?}", config);
        Ok(config)
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

    pub fn to_account_config(&self, account_name: Option<&str>) -> Result<AccountConfig> {
        let default_config = AccountConfig::default();

        let (name, deserialized_account_config) = match account_name {
            Some("default") | Some("") | None => self
                .accounts
                .iter()
                .find_map(|(name, account)| {
                    if let Some(true) = account.default {
                        Some((name.clone(), account))
                    } else {
                        None
                    }
                })
                .ok_or_else(|| anyhow!("cannot find default account")),
            Some(name) => self
                .accounts
                .get(name)
                .map(|account| (name.to_string(), account))
                .ok_or_else(|| anyhow!(format!("cannot find account {}", name))),
        }?;

        Ok(AccountConfig {
            name,
            work_duration: deserialized_account_config
                .work_duration
                .as_ref()
                .map(ToOwned::to_owned)
                .or_else(|| self.work_duration.as_ref().map(ToOwned::to_owned))
                .unwrap_or_else(|| default_config.work_duration),
            short_break_duration: deserialized_account_config
                .short_break_duration
                .as_ref()
                .map(ToOwned::to_owned)
                .or_else(|| self.short_break_duration.as_ref().map(ToOwned::to_owned))
                .unwrap_or_else(|| default_config.short_break_duration),
            long_break_duration: deserialized_account_config
                .long_break_duration
                .as_ref()
                .map(ToOwned::to_owned)
                .or_else(|| self.long_break_duration.as_ref().map(ToOwned::to_owned))
                .unwrap_or_else(|| default_config.long_break_duration),
        })
    }
}
