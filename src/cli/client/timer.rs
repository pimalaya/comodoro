//! # Timer output
//!
//! What every client command prints: the timer the server reported, read
//! after the command's change if it made one.
//!
//! [`TimerOutput`] carries the account precision beside the timer, since
//! how coarsely a duration reads is an account setting, and prints the
//! start as an instant rather than as the epoch seconds the wire carries.

use alloc::string::String;
use core::{fmt, time::Duration};

use std::time::UNIX_EPOCH;

use humantime::format_rfc3339_seconds;
use pimalaya_cli::sanitize;
use schemars::JsonSchema;
use serde::{Serialize, Serializer};

use crate::timer::{Timer, TimerCycle, TimerPrecision, TimerSchedule, TimerState};

/// The timer, as a client command prints it.
///
/// Its text is one line, `OFF`, `[Work] paused` or `[Work] 25min`, so a
/// status bar reads it as is.
#[derive(Clone, Debug, JsonSchema, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerOutput {
    /// What the timer runs: its cycles, and how many loops of them.
    pub schedule: TimerSchedule,
    /// The timer state.
    pub state: TimerState,
    /// The current cycle, its duration the time remaining in seconds.
    pub cycle: TimerCycle,
    /// When the timer was last started or resumed, an RFC 3339 instant
    /// in UTC, `null` unless the timer runs.
    #[serde(serialize_with = "serialize_rfc3339")]
    #[schemars(with = "Option<String>")]
    pub started_at: Option<u64>,
    /// Seconds accumulated before the last pause or stop.
    pub elapsed: usize,
    /// How precisely the text renders the remaining duration.
    #[serde(skip)]
    #[schemars(skip)]
    pub precision: TimerPrecision,
}

impl TimerOutput {
    /// Pairs the timer a server reported with the account precision.
    pub fn new(timer: Timer, precision: TimerPrecision) -> Self {
        Self {
            schedule: timer.schedule,
            state: timer.state,
            cycle: timer.cycle,
            started_at: timer.started_at,
            elapsed: timer.elapsed,
            precision,
        }
    }

    fn fmt_state(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: a TCP server may be remote, so its cycle name could drive
        // the terminal.
        let cycle = sanitize::text(&self.cycle.name);
        let duration = self.cycle.duration;

        match self.state {
            TimerState::Stopped => write!(f, "OFF"),
            TimerState::Paused => write!(f, "[{cycle}] paused"),
            TimerState::Running if duration < 60 => write!(f, "[{cycle}] {duration}s"),
            TimerState::Running if duration < 3600 => match self.precision {
                TimerPrecision::Second => {
                    write!(f, "[{cycle}] {}min {}s", duration / 60, duration % 60)
                }
                TimerPrecision::Minute | TimerPrecision::Hour => {
                    write!(f, "[{cycle}] {}min", duration / 60)
                }
            },
            TimerState::Running => match self.precision {
                TimerPrecision::Second => write!(
                    f,
                    "[{cycle}] {}h {}min {}s",
                    duration / 3600,
                    (duration % 3600) / 60,
                    (duration % 3600) % 60,
                ),
                TimerPrecision::Minute => write!(
                    f,
                    "[{cycle}] {}h {}min",
                    duration / 3600,
                    (duration % 3600) / 60,
                ),
                TimerPrecision::Hour => write!(f, "[{cycle}] {}h", duration / 3600),
            },
        }
    }
}

impl fmt::Display for TimerOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_state(f)?;

        // NOTE: the newline is what flushes the line-buffered stdout,
        // without which `watch` would print nothing until it exits.
        writeln!(f)
    }
}

/// Serializes Unix epoch seconds as an RFC 3339 instant in UTC.
fn serialize_rfc3339<S: Serializer>(secs: &Option<u64>, s: S) -> Result<S::Ok, S::Error> {
    match secs {
        Some(secs) => {
            let instant = UNIX_EPOCH + Duration::from_secs(*secs);
            s.collect_str(&format_rfc3339_seconds(instant))
        }
        None => s.serialize_none(),
    }
}
