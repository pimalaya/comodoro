use std::collections::HashMap;

#[cfg(feature = "notify")]
use notify_rust::Notification;
#[cfg(feature = "command")]
use process::Command;
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "tcp-client", feature = "tcp-binder"))]
use time::tcp::TcpConfig;
use time::timer::TimerCycle;
use tracing::debug;

/// Represents the user config file.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TomlPresetConfig {
    pub preset: Option<PresetKind>,

    #[serde(default)]
    pub cycles: Vec<TimerCycle>,

    #[cfg(any(feature = "tcp-client", feature = "tcp-binder"))]
    pub tcp: Option<TcpConfig>,

    #[serde(default)]
    pub hooks: HashMap<String, HookConfig>,

    #[serde(default)]
    pub cycles_count: usize,

    #[serde(default)]
    pub timer_precision: TimerPrecision,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HookConfig {
    /// The hook based on shell commands.
    #[cfg(feature = "command")]
    pub cmd: Option<Command>,

    /// The hook based on system notifications.
    #[cfg(feature = "notify")]
    pub notify: Option<NotifyConfig>,
}

impl HookConfig {
    pub async fn exec(&self, name: &str) {
        if let Some(cmd) = self.cmd.as_ref() {
            let res = cmd.run().await;

            if let Err(err) = res {
                debug!("error while executing command for hook {name}");
                debug!("{err:?}");
            }
        }

        #[cfg(target_os = "linux")]
        if let Some(notify) = self.notify.as_ref() {
            let notif = Notification::new()
                .summary(&notify.summary)
                .body(&notify.body)
                .show_async()
                .await;

            if let Err(err) = notif {
                debug!("error while sending system notification for hook {name}");
                debug!("{err:?}");
            }
        }

        #[cfg(not(target_os = "linux"))]
        if let Some(notify) = self.notify.as_ref() {
            let summary = notify.summary.clone();
            let body = notify.body.clone();

            let res = tokio::task::spawn_blocking(move || {
                Notification::new().summary(&summary).body(&body).show()
            })
            .await;

            if let Err(err) = res {
                debug!("cannot send notification for hook {name}");
                debug!("{err:?}");
            } else {
                let notif = res.unwrap();

                if let Err(err) = notif {
                    debug!("error while sending system notification for hook {name}");
                    debug!("{err:?}");
                }
            }
        }
    }
}

/// The configuration of the notify hook variant.
///
/// The structure tries to match the [`notify_rust::Notification`] API
/// and may evolve in the future.
#[cfg(feature = "notify")]
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NotifyConfig {
    /// The summary (or the title) of the notification.
    pub summary: String,

    /// The body of the notification.
    pub body: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TimerPrecision {
    #[serde(alias = "seconds", alias = "secs", alias = "sec", alias = "s")]
    Second,

    #[default]
    #[serde(alias = "minutes", alias = "mins", alias = "min", alias = "m")]
    Minute,

    #[serde(alias = "hours", alias = "h")]
    Hour,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PresetKind {
    #[serde(rename = "pomodoro")]
    PresetPomodoro,
    #[serde(rename = "52/17")]
    Preset52_17,
}
