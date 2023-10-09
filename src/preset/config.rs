use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::TimerCycle;

#[cfg(any(feature = "tcp-client", feature = "tcp-binder"))]
use crate::TcpConfig;

/// Represents the user config file.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PresetConfig {
    #[serde(flatten)]
    pub preset_or_cycles: PresetKindOrCyclesConfig,
    #[cfg(any(feature = "tcp-client", feature = "tcp-binder"))]
    #[serde(flatten)]
    pub tcp: Option<TcpConfig>,
    #[serde(flatten)]
    pub hooks: HashMap<String, String>,
    #[serde(default)]
    pub cycles_count: usize,
    #[serde(default)]
    pub timer_precision: TimerPrecision,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TimerPrecision {
    #[serde(alias = "seconds")]
    #[serde(alias = "sec")]
    #[serde(alias = "s")]
    Second,
    #[default]
    #[serde(alias = "minutes")]
    #[serde(alias = "mins")]
    #[serde(alias = "min")]
    #[serde(alias = "m")]
    Minute,
    #[serde(alias = "hours")]
    #[serde(alias = "h")]
    Hour,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PresetKindOrCyclesConfig {
    #[serde(rename = "preset")]
    Preset(PresetKind),
    #[serde(rename = "cycles")]
    Cycles(Vec<TimerCycle>),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PresetKind {
    #[serde(rename = "pomodoro")]
    PresetPomodoro,
    #[serde(rename = "52/17")]
    Preset52_17,
}
