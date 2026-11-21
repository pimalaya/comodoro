//! # RFC 3339
//!
//! The instants the timer carries, Unix epoch seconds in memory, written
//! on the wire and under `--json` as an RFC 3339 instant in UTC,
//! `2026-11-20T09:00:00Z`.
//!
//! The calendar arithmetic is Howard Hinnant's `civil_from_days` and
//! `days_from_civil`, so the timer needs no clock nor calendar crate.
//!
//! Refs: <https://datatracker.ietf.org/doc/html/rfc3339#section-5.6>

use std::fmt;

use serde::{Deserialize, Deserializer, Serializer, de::Error};

/// The last second RFC 3339 can write, 9999-12-31T23:59:59Z, in Unix
/// epoch seconds.
const MAX_SECS: u64 = 253_402_300_799;

/// The seconds of one day.
const DAY_SECS: u64 = 86_400;

/// Unix epoch seconds, displayed as an RFC 3339 instant in UTC, clamped
/// to the last instant RFC 3339 can write.
pub(crate) struct Rfc3339(pub(crate) u64);

impl fmt::Display for Rfc3339 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.min(MAX_SECS);
        let time = secs % DAY_SECS;

        let days = secs / DAY_SECS + 719_468;
        let era = days / 146_097;
        let doe = days - era * 146_097;
        let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;

        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = if mp < 10 { mp + 3 } else { mp - 9 };
        let year = yoe + era * 400 + u64::from(month <= 2);

        let (hour, minute, second) = (time / 3_600, time / 60 % 60, time % 60);
        write!(
            f,
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z"
        )
    }
}

/// Serializes epoch seconds as an RFC 3339 instant, `None` as `null`.
pub(crate) fn serialize<S: Serializer>(secs: &Option<u64>, s: S) -> Result<S::Ok, S::Error> {
    match secs {
        Some(secs) => s.collect_str(&Rfc3339(*secs)),
        None => s.serialize_none(),
    }
}

/// Deserializes the instant [`Rfc3339`] writes back into epoch seconds,
/// `null` as `None`, refusing any other form.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u64>, D::Error> {
    let Some(text) = Option::<String>::deserialize(d)? else {
        return Ok(None);
    };

    match parse(&text) {
        Some(secs) => Ok(Some(secs)),
        None => Err(D::Error::custom(format!(
            "invalid instant `{text}`, expected RFC 3339 in UTC as YYYY-MM-DDTHH:MM:SSZ"
        ))),
    }
}

/// Parses `YYYY-MM-DDTHH:MM:SSZ` from 1970 on into epoch seconds.
///
/// It is the one form [`Rfc3339`] writes, and a date the calendar lacks
/// is refused by writing the result back.
fn parse(text: &str) -> Option<u64> {
    let bytes = text.as_bytes();

    let shape = bytes.len() == 20
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes[10] == b'T'
        && bytes[13] == b':'
        && bytes[16] == b':'
        && bytes[19] == b'Z';

    if !shape {
        return None;
    }

    let number = |from: usize, to: usize| {
        bytes[from..to].iter().try_fold(0u64, |n, digit| {
            digit
                .is_ascii_digit()
                .then(|| n * 10 + u64::from(digit - b'0'))
        })
    };

    let (year, month, day) = (number(0, 4)?, number(5, 7)?, number(8, 10)?);
    let (hour, minute, second) = (number(11, 13)?, number(14, 16)?, number(17, 19)?);

    if year < 1970 || !(1..=12).contains(&month) || day == 0 {
        return None;
    }

    if hour > 23 || minute > 59 || second > 59 {
        return None;
    }

    let year = if month <= 2 { year - 1 } else { year };
    let era = year / 400;
    let yoe = year - era * 400;
    let mp = if month > 2 { month - 3 } else { month + 9 };
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;

    let secs = days * DAY_SECS + hour * 3_600 + minute * 60 + second;

    (format!("{}", Rfc3339(secs)) == text).then_some(secs)
}

#[cfg(test)]
mod tests {

    use crate::timer::rfc3339::*;

    const INSTANTS: [(u64, &str); 5] = [
        (0, "1970-01-01T00:00:00Z"),
        (951_782_400, "2000-02-29T00:00:00Z"),
        (1_709_164_800, "2024-02-29T00:00:00Z"),
        (1_786_711_298, "2026-08-14T12:41:38Z"),
        (MAX_SECS, "9999-12-31T23:59:59Z"),
    ];

    #[test]
    fn epoch_seconds_write_and_read_back_as_rfc3339() {
        for (secs, text) in INSTANTS {
            assert_eq!(Rfc3339(secs).to_string(), text);
            assert_eq!(parse(text), Some(secs), "{text}");
        }
    }

    #[test]
    fn an_instant_past_rfc3339_writes_its_last_one() {
        assert_eq!(Rfc3339(u64::MAX).to_string(), "9999-12-31T23:59:59Z");
    }

    #[test]
    fn any_other_form_is_refused() {
        let refused = [
            "2026-08-14T12:41:38+00:00",
            "2026-08-14T12:41:38.5Z",
            "2026-08-14t12:41:38z",
            "2026-08-14 12:41:38Z",
            "2026-02-30T00:00:00Z",
            "2025-02-29T00:00:00Z",
            "2026-13-01T00:00:00Z",
            "2026-08-14T24:00:00Z",
            "2026-08-14T12:60:00Z",
            "2026-08-14T12:41:60Z",
            "1969-12-31T23:59:59Z",
            "2026-08-1éT12:41:38Z",
            "1786711298",
        ];

        for text in refused {
            assert_eq!(parse(text), None, "{text}");
        }
    }
}
