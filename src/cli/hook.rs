//! Reactions run when the timer emits an event.
//!
//! A hook is bound to an event by its name in the account
//! configuration, and runs either a shell command or a desktop
//! notification. Both reactions perform their I/O directly, since
//! neither the process nor the notification backend has anything to
//! resume.

use alloc::{format, string::String};

use std::process::Command;

use convert_case::{Case, Casing};
use log::{debug, error, warn};
#[cfg(feature = "notify")]
use notify_rust::Notification;
use serde::{Deserialize, Serialize};

use crate::timer::TimerEvent;

/// Configuration mapping of the events a timer emits.
///
/// Lives here rather than next to [`TimerEvent`] because hook naming is
/// a configuration concern: the timer itself knows nothing about hooks.
impl TimerEvent {
    /// The configuration key holding the hook bound to this event.
    ///
    /// A cycle event is named after the kebab-case cycle name, so the
    /// cycle `Long rest` ending fires `on-long-rest-end`. The two
    /// timer-wide events carry a fixed name.
    pub fn hook_name(&self) -> String {
        let suffix = match self {
            Self::Started => return String::from("on-timer-start"),
            Self::Stopped => return String::from("on-timer-stop"),
            Self::Began(_) => "begin",
            Self::Running(_) => "running",
            Self::Set(_) => "set",
            Self::Paused(_) => "pause",
            Self::Resumed(_) => "resume",
            Self::Ended(_) => "end",
        };

        let name = match self.cycle() {
            Some(cycle) => cycle.name.to_case(Case::Kebab),
            None => String::new(),
        };

        format!("on-{name}-{suffix}")
    }
}

/// A reaction bound to a timer event.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimerHook {
    /// Runs a command, given either as a shell line or as a program
    /// followed by its arguments.
    Command(#[serde(with = "pimalaya_config::command")] Command),
    /// Sends a desktop notification, which requires the `notify`
    /// feature.
    Notify(TimerHookNotification),
}

impl TimerHook {
    /// Runs the reaction and waits for it to complete, logging whatever
    /// goes wrong.
    ///
    /// A hook reacts to the timer, it is never a step of it, so it
    /// reports nothing back: a command that cannot be run, one that
    /// exits non-zero, and a notification that reaches no daemon are
    /// all logged and left there. Returning no error is what makes it
    /// impossible for a hook to stop the timer.
    pub fn execute(&mut self) {
        match self {
            Self::Command(cmd) => {
                debug!("begin hook command execution");

                match cmd.status() {
                    Ok(status) if status.success() => {}
                    Ok(status) => warn!("hook command exited with {status}"),
                    Err(err) => error!("cannot run hook command: {err}"),
                }

                debug!("end of hook command execution");
            }
            Self::Notify(notification) => notification.send(),
        }
    }
}

/// The summary and body of a desktop notification.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TimerHookNotification {
    /// The notification title.
    pub summary: String,
    /// The notification content.
    pub body: String,
}

impl TimerHookNotification {
    /// Sends the notification to the system notification daemon,
    /// logging a daemon that cannot be reached.
    #[cfg(feature = "notify")]
    pub fn send(&self) {
        debug!("begin hook notification send");

        if let Err(err) = Notification::new()
            .summary(&self.summary)
            .body(&self.body)
            .show()
        {
            error!("cannot send hook notification: {err}");
        }

        debug!("end of hook notification send");
    }

    /// Logs that this build cannot send notifications.
    #[cfg(not(feature = "notify"))]
    pub fn send(&self) {
        error!(
            "cannot send hook notification `{}`: this build carries no notification backend, rebuild with the `notify` feature",
            self.summary,
        );
    }
}
