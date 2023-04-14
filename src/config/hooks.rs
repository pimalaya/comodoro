use log::{debug, warn};
use pimalaya::time::pomodoro::{ServerBuilder, ServerEvent, TimerCycle, TimerEvent};
use serde::{Deserialize, Serialize};
use std::{
    env, io,
    process::{Command, Stdio},
};

use crate::Config;

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct HooksConfig {
    pub server_started_hook: Option<String>,
    pub server_stopping_hook: Option<String>,
    pub server_stopped_hook: Option<String>,

    pub timer_started_hook: Option<String>,
    pub timer_stopped_hook: Option<String>,

    pub work_began_hook: Option<String>,
    pub work_running_hook: Option<String>,
    pub work_paused_hook: Option<String>,
    pub work_resumed_hook: Option<String>,
    pub work_ended_hook: Option<String>,

    pub first_work_began_hook: Option<String>,
    pub first_work_running_hook: Option<String>,
    pub first_work_paused_hook: Option<String>,
    pub first_work_resumed_hook: Option<String>,
    pub first_work_ended_hook: Option<String>,

    pub second_work_began_hook: Option<String>,
    pub second_work_running_hook: Option<String>,
    pub second_work_paused_hook: Option<String>,
    pub second_work_resumed_hook: Option<String>,
    pub second_work_ended_hook: Option<String>,

    pub short_break_began_hook: Option<String>,
    pub short_break_running_hook: Option<String>,
    pub short_break_paused_hook: Option<String>,
    pub short_break_resumed_hook: Option<String>,
    pub short_break_ended_hook: Option<String>,

    pub first_short_break_began_hook: Option<String>,
    pub first_short_break_running_hook: Option<String>,
    pub first_short_break_paused_hook: Option<String>,
    pub first_short_break_resumed_hook: Option<String>,
    pub first_short_break_ended_hook: Option<String>,

    pub second_short_break_began_hook: Option<String>,
    pub second_short_break_running_hook: Option<String>,
    pub second_short_break_paused_hook: Option<String>,
    pub second_short_break_resumed_hook: Option<String>,
    pub second_short_break_ended_hook: Option<String>,

    pub long_break_began_hook: Option<String>,
    pub long_break_running_hook: Option<String>,
    pub long_break_paused_hook: Option<String>,
    pub long_break_resumed_hook: Option<String>,
    pub long_break_ended_hook: Option<String>,
}

impl HooksConfig {
    pub fn run(cmd: Option<String>) -> io::Result<()> {
        if let Some(cmd) = cmd {
            debug!("running hook: {cmd}");

            let windows = cfg!(target_os = "windows")
                && !(env::var("MSYSTEM")
                    .map(|env| env.starts_with("MINGW"))
                    .unwrap_or_default());

            let mut pipeline = if windows {
                Command::new("cmd")
                    .args(&["/C", &cmd])
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::piped())
                    .spawn()
            } else {
                Command::new("sh")
                    .arg("-c")
                    .arg(&cmd)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::piped())
                    .spawn()
            }?;

            match pipeline.wait()?.code() {
                Some(0) => (),
                Some(code) => warn!("command {cmd:?} returned non-zero status exit code {code}"),
                _ => (),
            }
        };

        Ok(())
    }

    pub fn merge_with(mut self, config: &Config) -> Self {
        self.server_started_hook = self
            .server_started_hook
            .or_else(|| config.hooks.server_started_hook.clone());
        self.server_stopping_hook = self
            .server_stopping_hook
            .or_else(|| config.hooks.server_stopping_hook.clone());
        self.server_stopped_hook = self
            .server_stopped_hook
            .or_else(|| config.hooks.server_stopped_hook.clone());

        self.timer_started_hook = self
            .timer_started_hook
            .or_else(|| config.hooks.timer_started_hook.clone());
        self.timer_stopped_hook = self
            .timer_stopped_hook
            .or_else(|| config.hooks.timer_stopped_hook.clone());

        self.work_began_hook = self
            .work_began_hook
            .or_else(|| config.hooks.work_began_hook.clone());
        self.work_running_hook = self
            .work_running_hook
            .or_else(|| config.hooks.work_running_hook.clone());
        self.work_paused_hook = self
            .work_paused_hook
            .or_else(|| config.hooks.work_paused_hook.clone());
        self.work_resumed_hook = self
            .work_resumed_hook
            .or_else(|| config.hooks.work_resumed_hook.clone());
        self.work_ended_hook = self
            .work_ended_hook
            .or_else(|| config.hooks.work_ended_hook.clone());

        self.first_work_began_hook = self
            .first_work_began_hook
            .or_else(|| config.hooks.first_work_began_hook.clone());
        self.first_work_running_hook = self
            .first_work_running_hook
            .or_else(|| config.hooks.first_work_running_hook.clone());
        self.first_work_paused_hook = self
            .first_work_paused_hook
            .or_else(|| config.hooks.first_work_paused_hook.clone());
        self.first_work_resumed_hook = self
            .first_work_resumed_hook
            .or_else(|| config.hooks.first_work_resumed_hook.clone());
        self.first_work_ended_hook = self
            .first_work_ended_hook
            .or_else(|| config.hooks.first_work_ended_hook.clone());

        self.second_work_began_hook = self
            .second_work_began_hook
            .or_else(|| config.hooks.second_work_began_hook.clone());
        self.second_work_running_hook = self
            .second_work_running_hook
            .or_else(|| config.hooks.second_work_running_hook.clone());
        self.second_work_paused_hook = self
            .second_work_paused_hook
            .or_else(|| config.hooks.second_work_paused_hook.clone());
        self.second_work_resumed_hook = self
            .second_work_resumed_hook
            .or_else(|| config.hooks.second_work_resumed_hook.clone());
        self.second_work_ended_hook = self
            .second_work_ended_hook
            .or_else(|| config.hooks.second_work_ended_hook.clone());

        self.short_break_began_hook = self
            .short_break_began_hook
            .or_else(|| config.hooks.short_break_began_hook.clone());
        self.short_break_running_hook = self
            .short_break_running_hook
            .or_else(|| config.hooks.short_break_running_hook.clone());
        self.short_break_paused_hook = self
            .short_break_paused_hook
            .or_else(|| config.hooks.short_break_paused_hook.clone());
        self.short_break_resumed_hook = self
            .short_break_resumed_hook
            .or_else(|| config.hooks.short_break_resumed_hook.clone());
        self.short_break_ended_hook = self
            .short_break_ended_hook
            .or_else(|| config.hooks.short_break_ended_hook.clone());

        self.first_short_break_began_hook = self
            .first_short_break_began_hook
            .or_else(|| config.hooks.first_short_break_began_hook.clone());
        self.first_short_break_running_hook = self
            .first_short_break_running_hook
            .or_else(|| config.hooks.first_short_break_running_hook.clone());
        self.first_short_break_paused_hook = self
            .first_short_break_paused_hook
            .or_else(|| config.hooks.first_short_break_paused_hook.clone());
        self.first_short_break_resumed_hook = self
            .first_short_break_resumed_hook
            .or_else(|| config.hooks.first_short_break_resumed_hook.clone());
        self.first_short_break_ended_hook = self
            .first_short_break_ended_hook
            .or_else(|| config.hooks.first_short_break_ended_hook.clone());

        self.second_short_break_began_hook = self
            .second_short_break_began_hook
            .or_else(|| config.hooks.second_short_break_began_hook.clone());
        self.second_short_break_running_hook = self
            .second_short_break_running_hook
            .or_else(|| config.hooks.second_short_break_running_hook.clone());
        self.second_short_break_paused_hook = self
            .second_short_break_paused_hook
            .or_else(|| config.hooks.second_short_break_paused_hook.clone());
        self.second_short_break_resumed_hook = self
            .second_short_break_resumed_hook
            .or_else(|| config.hooks.second_short_break_resumed_hook.clone());
        self.second_short_break_ended_hook = self
            .second_short_break_ended_hook
            .or_else(|| config.hooks.second_short_break_ended_hook.clone());

        self.long_break_began_hook = self
            .long_break_began_hook
            .or_else(|| config.hooks.long_break_began_hook.clone());
        self.long_break_running_hook = self
            .long_break_running_hook
            .or_else(|| config.hooks.long_break_running_hook.clone());
        self.long_break_paused_hook = self
            .long_break_paused_hook
            .or_else(|| config.hooks.long_break_paused_hook.clone());
        self.long_break_resumed_hook = self
            .long_break_resumed_hook
            .or_else(|| config.hooks.long_break_resumed_hook.clone());
        self.long_break_ended_hook = self
            .long_break_ended_hook
            .or_else(|| config.hooks.long_break_ended_hook.clone());

        self
    }

    pub fn apply(&self, config: &Config, server: ServerBuilder) -> ServerBuilder {
        let config = self.clone().merge_with(config);

        server
            .with_server_handler(move |event: ServerEvent| match event {
                ServerEvent::Started => Self::run(config.server_started_hook.clone()),
                ServerEvent::Stopping => Self::run(config.server_stopping_hook.clone()),
                ServerEvent::Stopped => Self::run(config.server_stopped_hook.clone()),
            })
            .with_timer_handler(move |event: TimerEvent| match event {
                TimerEvent::Started => Self::run(config.timer_started_hook.clone()),
                TimerEvent::Stopped => Self::run(config.timer_stopped_hook.clone()),

                TimerEvent::Began(TimerCycle::FirstWork) => {
                    Self::run(config.work_began_hook.clone())?;
                    Self::run(config.first_work_began_hook.clone())
                }
                TimerEvent::Began(TimerCycle::FirstShortBreak) => {
                    Self::run(config.short_break_began_hook.clone())?;
                    Self::run(config.first_short_break_began_hook.clone())
                }
                TimerEvent::Began(TimerCycle::SecondWork) => {
                    Self::run(config.work_began_hook.clone())?;
                    Self::run(config.second_work_began_hook.clone())
                }
                TimerEvent::Began(TimerCycle::SecondShortBreak) => {
                    Self::run(config.short_break_began_hook.clone())?;
                    Self::run(config.second_short_break_began_hook.clone())
                }
                TimerEvent::Began(TimerCycle::LongBreak) => {
                    Self::run(config.long_break_began_hook.clone())
                }

                TimerEvent::Running(TimerCycle::FirstWork) => {
                    Self::run(config.work_running_hook.clone())?;
                    Self::run(config.first_work_running_hook.clone())
                }
                TimerEvent::Running(TimerCycle::FirstShortBreak) => {
                    Self::run(config.short_break_running_hook.clone())?;
                    Self::run(config.first_short_break_running_hook.clone())
                }
                TimerEvent::Running(TimerCycle::SecondWork) => {
                    Self::run(config.work_running_hook.clone())?;
                    Self::run(config.second_work_running_hook.clone())
                }
                TimerEvent::Running(TimerCycle::SecondShortBreak) => {
                    Self::run(config.short_break_running_hook.clone())?;
                    Self::run(config.second_short_break_running_hook.clone())
                }
                TimerEvent::Running(TimerCycle::LongBreak) => {
                    Self::run(config.long_break_running_hook.clone())
                }

                TimerEvent::Paused(TimerCycle::FirstWork) => {
                    Self::run(config.work_paused_hook.clone())?;
                    Self::run(config.first_work_paused_hook.clone())
                }
                TimerEvent::Paused(TimerCycle::FirstShortBreak) => {
                    Self::run(config.short_break_paused_hook.clone())?;
                    Self::run(config.first_short_break_paused_hook.clone())
                }
                TimerEvent::Paused(TimerCycle::SecondWork) => {
                    Self::run(config.work_paused_hook.clone())?;
                    Self::run(config.second_work_paused_hook.clone())
                }
                TimerEvent::Paused(TimerCycle::SecondShortBreak) => {
                    Self::run(config.short_break_paused_hook.clone())?;
                    Self::run(config.second_short_break_paused_hook.clone())
                }
                TimerEvent::Paused(TimerCycle::LongBreak) => {
                    Self::run(config.long_break_paused_hook.clone())
                }

                TimerEvent::Resumed(TimerCycle::FirstWork) => {
                    Self::run(config.work_resumed_hook.clone())?;
                    Self::run(config.first_work_resumed_hook.clone())
                }
                TimerEvent::Resumed(TimerCycle::FirstShortBreak) => {
                    Self::run(config.short_break_resumed_hook.clone())?;
                    Self::run(config.first_short_break_resumed_hook.clone())
                }
                TimerEvent::Resumed(TimerCycle::SecondWork) => {
                    Self::run(config.work_resumed_hook.clone())?;
                    Self::run(config.second_work_resumed_hook.clone())
                }
                TimerEvent::Resumed(TimerCycle::SecondShortBreak) => {
                    Self::run(config.short_break_resumed_hook.clone())?;
                    Self::run(config.second_short_break_resumed_hook.clone())
                }
                TimerEvent::Resumed(TimerCycle::LongBreak) => {
                    Self::run(config.long_break_resumed_hook.clone())
                }

                TimerEvent::Ended(TimerCycle::FirstWork) => {
                    Self::run(config.work_ended_hook.clone())?;
                    Self::run(config.first_work_ended_hook.clone())
                }
                TimerEvent::Ended(TimerCycle::FirstShortBreak) => {
                    Self::run(config.short_break_ended_hook.clone())?;
                    Self::run(config.first_short_break_ended_hook.clone())
                }
                TimerEvent::Ended(TimerCycle::SecondWork) => {
                    Self::run(config.work_ended_hook.clone())?;
                    Self::run(config.second_work_ended_hook.clone())
                }
                TimerEvent::Ended(TimerCycle::SecondShortBreak) => {
                    Self::run(config.short_break_ended_hook.clone())?;
                    Self::run(config.second_short_break_ended_hook.clone())
                }
                TimerEvent::Ended(TimerCycle::LongBreak) => {
                    Self::run(config.long_break_ended_hook.clone())
                }
            })
    }
}
