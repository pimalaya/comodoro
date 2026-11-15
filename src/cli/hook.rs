//! # Hook
//!
//! The reactions run when the timer emits an event, bound to it by name
//! in the account configuration.
//!
//! A hook runs a command, shows a desktop notification, or both, and
//! performs its I/O directly, neither the process nor the notification
//! backend having anything to resume. Both are read through
//! pimalaya-config: a shell line or an argument list becomes a
//! [`CommandConfig`], a summary and a body a [`NotificationTemplate`].
//!
//! A cycle hook fills the variables `cycleName` and `cycleDuration`,
//! expanded in its notification and handed to its command as environment
//! variables of the same names, so one spelling serves both. The timer
//! hooks fill none. [`TimerHook::validate`] refuses a notification naming
//! a variable its hook does not fill, as the account is resolved.
//!
//! The 2.x `on-<cycle>-running` name is refused naming `on-<cycle>-tick`
//! and MIGRATION.md. `cmd` reads as the alias of `command`, as wherever
//! the family configures a command.
//!
//! A build without the `notify` feature has no notification backend, so
//! an account carrying one is refused as it loads, naming the cargo
//! feature to rebuild with.

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};

use anyhow::{Result, bail};
use convert_case::{Case, Casing};
use log::{debug, error, warn};
use pimalaya_config::{command::CommandConfig, notify::NotificationTemplate};
use serde::Deserialize;

use crate::timer::{TimerCycle, TimerEvent};

/// The events a cycle fires, each the suffix of its hook name.
pub const CYCLE_EVENTS: [&str; 6] = ["begin", "tick", "set", "pause", "resume", "end"];

/// The hook names the timer itself fires, whatever its cycles.
pub const TIMER_HOOKS: [&str; 2] = ["on-timer-start", "on-timer-stop"];

/// The variables a cycle hook fills.
pub const CYCLE_VARIABLES: [&str; 2] = ["cycleName", "cycleDuration"];

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
            Self::Started => return String::from(TIMER_HOOKS[0]),
            Self::Stopped => return String::from(TIMER_HOOKS[1]),
            Self::Began(_) => "begin",
            Self::Ticked(_) => "tick",
            Self::Set(_) => "set",
            Self::Paused(_) => "pause",
            Self::Resumed(_) => "resume",
            Self::Ended(_) => "end",
        };

        let name = match self.cycle() {
            Some(cycle) => cycle_hook_prefix(cycle),
            None => String::new(),
        };

        format!("on-{name}-{suffix}")
    }

    /// The variables the hook bound to this event fills, by name: the
    /// cycle's name and remaining seconds for a cycle event, none for the
    /// timer-wide ones.
    pub fn hook_variables(&self) -> Vec<(&'static str, String)> {
        match self.cycle() {
            Some(cycle) => Vec::from([
                (CYCLE_VARIABLES[0], cycle.name.clone()),
                (CYCLE_VARIABLES[1], cycle.duration.to_string()),
            ]),
            None => Vec::new(),
        }
    }
}

/// The kebab-case cycle name opening the hook names of a cycle.
fn cycle_hook_prefix(cycle: &TimerCycle) -> String {
    cycle.name.to_case(Case::Kebab)
}

/// A reaction bound to a timer event: a command, a notification, or
/// both, the command running first.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TimerHook {
    /// Runs a command, a shell line or a program with its arguments.
    ///
    /// `cmd` is read as its alias, as wherever a command is configured.
    #[serde(alias = "cmd")]
    pub command: Option<CommandConfig>,
    /// Shows a desktop notification, given as a summary and a body.
    pub notify: Option<NotificationTemplate>,
}

impl TimerHook {
    /// Checks the hook bound under `name` against the account's
    /// `cycles`: the name must be one an event fires, the hook must do
    /// something, and its notification may name only the variables its
    /// event fills.
    pub fn validate(&self, name: &str, cycles: &[TimerCycle]) -> Result<()> {
        let prefixes: Vec<String> = cycles.iter().map(cycle_hook_prefix).collect();

        let cycle_hook = prefixes.iter().any(|prefix| {
            CYCLE_EVENTS
                .iter()
                .any(|event| name == format!("on-{prefix}-{event}"))
        });

        if !cycle_hook && !TIMER_HOOKS.contains(&name) {
            let tick = prefixes.iter().find(|p| name == format!("on-{p}-running"));

            if let Some(prefix) = tick {
                bail!("Hook `{name}` is now `on-{prefix}-tick`, see MIGRATION.md");
            }

            let mut prefixes = prefixes;
            prefixes.dedup();

            bail!(
                "Hook `{name}` matches no event of this account: name it `on-<cycle>-<event>`, the cycle one of {} and the event one of {}, or use {}",
                prefixes.join(", "),
                CYCLE_EVENTS.join(", "),
                TIMER_HOOKS.join(" or "),
            );
        }

        if self.command.is_none() && self.notify.is_none() {
            bail!("Hook `{name}` does nothing, give it a `command`, a `notify` or both");
        }

        if let Some(notify) = &self.notify {
            let known: &[&str] = if cycle_hook { &CYCLE_VARIABLES } else { &[] };

            if let Err(err) = notify.check(|var| known.contains(&var)) {
                let fills = match known {
                    [] => String::from("no variable"),
                    known => {
                        let vars: Vec<String> = known.iter().map(|var| format!("${var}")).collect();
                        vars.join(", ")
                    }
                };

                bail!("Hook `{name}`: {err}, it fills {fills}");
            }
        }

        Ok(())
    }

    /// Runs the reaction with the variables its event fills, logging
    /// whatever goes wrong.
    ///
    /// A hook reacts to the timer and is never a step of it: a command
    /// that cannot be run, one exiting non-zero and a notification
    /// reaching no daemon are all logged and left there. Returning no
    /// error is what keeps a hook from stopping the timer.
    pub fn execute(&self, variables: &[(&'static str, String)]) {
        if let Some(command) = &self.command {
            debug!("begin hook command execution");

            let mut command = command.to_command();
            command.envs(variables.iter().map(|(name, value)| (*name, value)));

            match command.status() {
                Ok(status) if status.success() => {}
                Ok(status) => warn!("hook command exited with {status}"),
                Err(err) => error!("cannot run hook command: {err}"),
            }

            debug!("end of hook command execution");
        }

        if let Some(notify) = &self.notify {
            debug!("begin hook notification send");

            let lookup = |name: &str| {
                let variable = variables.iter().find(|(var, _)| *var == name);
                Some(variable.map(|(_, value)| value.clone()).unwrap_or_default())
            };

            match notify.render(lookup) {
                Ok(notification) => {
                    if let Err(err) = notification.show() {
                        error!("cannot send hook notification: {err}");
                    }
                }
                Err(err) => error!("cannot render hook notification: {err}"),
            }

            debug!("end of hook notification send");
        }
    }
}
