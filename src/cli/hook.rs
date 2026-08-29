//! # Hook
//!
//! The reactions run when the timer emits an event, bound to it by name
//! in the account configuration.
//!
//! A hook runs a shell command or a desktop notification, and performs
//! its I/O directly, neither the process nor the notification backend
//! having anything to resume.
//!
//! Both are read straight into the type performing them, through the
//! pimalaya-config serde adapters: a shell line or an argument list
//! becomes a [`Command`], a summary and a body a [`Notification`].
//!
//! A build without the `notify` feature has no notification backend, so
//! an account carrying one is refused as it loads, naming the cargo
//! feature to rebuild with.

use alloc::{format, string::String};

use std::process::Command;

use convert_case::{Case, Casing};
use log::{debug, error, warn};
use pimalaya_config::notify::Notification;
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
    /// Runs a command, a shell line or a program with its arguments.
    Command(#[serde(with = "pimalaya_config::command")] Command),
    /// Sends a desktop notification, given as a summary and a body.
    Notify(#[serde(with = "pimalaya_config::notify")] Notification),
}

impl TimerHook {
    /// Runs the reaction and waits for it, logging whatever goes wrong.
    ///
    /// A hook reacts to the timer and is never a step of it: a command
    /// that cannot be run, one exiting non-zero and a notification
    /// reaching no daemon are all logged and left there. Returning no
    /// error is what keeps a hook from stopping the timer.
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
            Self::Notify(notification) => {
                debug!("begin hook notification send");

                if let Err(err) = notification.show() {
                    error!("cannot send hook notification: {err}");
                }

                debug!("end of hook notification send");
            }
        }
    }
}
