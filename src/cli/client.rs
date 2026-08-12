//! Commands driving the timer from a client.
//!
//! One module per command, each connecting to the server, sending its
//! request and printing the outcome. What they share lives here: the
//! transport argument selecting the connection, and the rendering the
//! two commands that display a timer print.

pub mod get;
pub mod pause;
pub mod resume;
pub mod set;
pub mod start;
pub mod stop;
pub mod watch;

use core::fmt;

use clap::Parser;
use serde::{Serialize, Serializer};

use crate::{
    cli::config::{ComodoroAccountConfig, ComodoroTransport},
    timer::{Timer, TimerPrecision, TimerState},
};

/// The transport a client command reaches the server over.
#[derive(Debug, Parser)]
pub struct ComodoroTransportArg {
    /// The transport used to send the request.
    ///
    /// Defaults to the transport the account configuration marks as
    /// default, or to the local socket when neither does.
    #[arg(name = "transport", value_name = "TRANSPORT")]
    pub transport: Option<ComodoroTransport>,
}

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
