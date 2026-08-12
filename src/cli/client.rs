//! Commands driving the timer from a client.
//!
//! One module per command, each connecting to the server, sending its
//! request and printing the outcome. What lives here is the rendering
//! the two commands displaying a timer share. The transport argument
//! they also share sits in [`crate::cli::transport`], since the server
//! commands select a transport too.

pub mod get;
pub mod pause;
pub mod resume;
pub mod set;
pub mod start;
pub mod stop;
pub mod watch;

use core::fmt;

use serde::{Serialize, Serializer};

use crate::{
    cli::config::ComodoroAccountConfig,
    timer::{Timer, TimerPrecision, TimerState},
};

/// A timer as the terminal shows it, at the account precision.
struct DisplayTimer<'a> {
    account: &'a ComodoroAccountConfig,
    timer: Timer,
}

impl fmt::Display for DisplayTimer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_state(f)?;
        // NOTE: the newline is what makes the line-buffered stdout
        // flush, without which `watch` would print nothing until it
        // exits. It also separates consecutive outputs.
        writeln!(f)
    }
}

impl DisplayTimer<'_> {
    fn fmt_state(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let timer = &self.timer;
        let cycle = &timer.cycle.name;

        match timer.state {
            TimerState::Stopped => write!(f, "OFF"),
            TimerState::Paused => write!(f, "[{cycle}] paused"),
            TimerState::Running if timer.cycle.duration < 60 => {
                write!(f, "[{cycle}] {}s", timer.cycle.duration)
            }
            TimerState::Running if timer.cycle.duration < 3600 => match self.account.precision {
                TimerPrecision::Second => write!(
                    f,
                    "[{cycle}] {}min {}s",
                    timer.cycle.duration / 60,
                    timer.cycle.duration % 60
                ),
                TimerPrecision::Minute | TimerPrecision::Hour => {
                    write!(f, "[{cycle}] {}min", timer.cycle.duration / 60)
                }
            },
            TimerState::Running => match self.account.precision {
                TimerPrecision::Second => write!(
                    f,
                    "[{cycle}] {}h {}min {}s",
                    timer.cycle.duration / 3600,
                    (timer.cycle.duration % 3600) / 60,
                    (timer.cycle.duration % 3600) % 60,
                ),
                TimerPrecision::Minute => write!(
                    f,
                    "[{cycle}] {}h {}min",
                    timer.cycle.duration / 3600,
                    (timer.cycle.duration % 3600) / 60,
                ),
                TimerPrecision::Hour => write!(f, "[{cycle}] {}h", timer.cycle.duration / 3600),
            },
        }
    }
}

impl Serialize for DisplayTimer<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.timer.serialize(serializer)
    }
}
