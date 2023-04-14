use pimalaya::time::pomodoro::ServerBuilder;
use serde::{Deserialize, Serialize};

use crate::Config;

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DurationsConfig {
    pub work_duration: Option<usize>,
    pub short_break_duration: Option<usize>,
    pub long_break_duration: Option<usize>,
}

impl DurationsConfig {
    pub fn apply(&self, config: &Config, mut server: ServerBuilder) -> ServerBuilder {
        if let Some(duration) = self.work_duration.or(config.durations.work_duration) {
            server = server.with_work_duration(duration);
        }

        if let Some(duration) = self
            .short_break_duration
            .or(config.durations.short_break_duration)
        {
            server = server.with_short_break_duration(duration);
        }

        if let Some(duration) = self
            .long_break_duration
            .or(config.durations.long_break_duration)
        {
            server = server.with_long_break_duration(duration);
        }

        server
    }
}
