//! # Timer output
//!
//! What every client command prints: the timer the server reported, read
//! after the command's change if it made one.
//!
//! [`TimerOutput`] carries the account precision beside the timer, since
//! how coarsely a duration reads is an account setting, and prints the
//! start as an instant rather than as the epoch seconds the wire carries.

use alloc::{format, string::String};
use core::{fmt, time::Duration};

use jiff::{SignedDuration, Span, SpanRound, Unit, civil, fmt::friendly::SpanPrinter};
use pimalaya_cli::sanitize;
use schemars::JsonSchema;
use serde::{Serialize, Serializer, ser::Error as _};

use crate::timer::{Timer, TimerCycle, TimerPrecision, TimerSchedule, TimerState};

/// The timer, as a client command prints it.
///
/// Its text is one line, `OFF`, `[Work] paused` or `[Work] 25m`, so a
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

        match self.state {
            TimerState::Stopped => write!(f, "OFF"),
            TimerState::Paused => write!(f, "[{cycle}] paused"),
            TimerState::Running => {
                let left = truncate(self.cycle.duration as u64, &self.precision);
                write!(f, "[{cycle}] {}", friendly(left))
            }
        }
    }
}

/// Truncates a remaining duration in seconds to the precision, down to
/// the unit under it when the precision's own would read zero.
fn truncate(secs: u64, precision: &TimerPrecision) -> u64 {
    let unit = match precision {
        TimerPrecision::Second => 1,
        TimerPrecision::Minute | TimerPrecision::Hour if secs < 60 => 1,
        TimerPrecision::Minute => 60,
        TimerPrecision::Hour if secs < 3600 => 60,
        TimerPrecision::Hour => 3600,
    };

    secs - secs % unit
}

/// Prints a duration in seconds in jiff's friendly form, balanced up to
/// days of 24 hours: `2d 4h 30m`, `0s` when empty.
///
/// A duration too long for a jiff span stops at hours.
fn friendly(secs: u64) -> String {
    let rounding = SpanRound::new().largest(Unit::Day).days_are_24_hours();

    let span = i64::try_from(secs)
        .ok()
        .and_then(|secs| Span::new().try_seconds(secs).ok())
        .and_then(|span| span.round(rounding).ok());

    match span {
        Some(span) => format!("{span:#}"),
        None => SpanPrinter::new().unsigned_duration_to_string(&Duration::from_secs(secs)),
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

/// The last second RFC 3339 can write, 9999-12-31T23:59:59Z, in Unix
/// epoch seconds.
const RFC3339_MAX_SECS: u64 = 253_402_300_799;

/// Serializes Unix epoch seconds as an RFC 3339 instant in UTC, clamped
/// to the last instant RFC 3339 can write.
fn serialize_rfc3339<S: Serializer>(secs: &Option<u64>, s: S) -> Result<S::Ok, S::Error> {
    match secs {
        Some(secs) => {
            let secs = (*secs).min(RFC3339_MAX_SECS) as i64;
            let epoch = civil::date(1970, 1, 1).at(0, 0, 0, 0);
            let instant = epoch
                .checked_add(SignedDuration::from_secs(secs))
                .map_err(S::Error::custom)?;

            s.collect_str(&format_args!("{instant}Z"))
        }
        None => s.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cli::client::timer::{friendly, truncate},
        timer::TimerPrecision,
    };

    #[test]
    fn a_remaining_duration_prints_at_its_precision() {
        let cases = [
            (0, TimerPrecision::Minute, "0s"),
            (59, TimerPrecision::Hour, "59s"),
            (90, TimerPrecision::Second, "1m 30s"),
            (90, TimerPrecision::Minute, "1m"),
            (1500, TimerPrecision::Minute, "25m"),
            (3599, TimerPrecision::Hour, "59m"),
            (3600, TimerPrecision::Second, "1h"),
            (5430, TimerPrecision::Second, "1h 30m 30s"),
            (5430, TimerPrecision::Minute, "1h 30m"),
            (5430, TimerPrecision::Hour, "1h"),
            (187_200 + 61, TimerPrecision::Second, "2d 4h 1m 1s"),
            (187_200 + 61, TimerPrecision::Hour, "2d 4h"),
        ];

        for (secs, precision, text) in cases {
            assert_eq!(friendly(truncate(secs, &precision)), text, "{secs}");
        }
    }

    #[test]
    fn a_duration_past_a_span_prints_in_hours() {
        assert_eq!(friendly(u64::MAX), "5124095576030431h 15s");
    }
}
