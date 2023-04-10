//! Deserialized config module.
//!
//! This module contains the raw deserialized representation of the
//! user configuration file.

use anyhow::{anyhow, Context, Result};
use dirs::{config_dir, home_dir};
use log::{debug, trace};
use pimalaya::time::pomodoro::ServerBind;
#[cfg(feature = "tcp-binder")]
use pimalaya::time::pomodoro::TcpBind;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use toml;

use super::DurationsConfig;
#[cfg(any(feature = "tcp-client", feature = "tcp-binder"))]
use super::TcpConfig;

/// Represents the user config file.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(flatten)]
    pub durations: DurationsConfig,
    #[cfg(any(feature = "tcp-client", feature = "tcp-binder"))]
    pub tcp: Option<TcpConfig>,
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

    pub fn to_binders(&self) -> Vec<Box<dyn ServerBind>> {
        let mut binders: Vec<Box<dyn ServerBind>> = Vec::new();

        #[cfg(feature = "tcp-binder")]
        if let Some(ref config) = self.tcp {
            binders.push(TcpBind::new(&config.host, config.port))
        }

        binders
    }
}
