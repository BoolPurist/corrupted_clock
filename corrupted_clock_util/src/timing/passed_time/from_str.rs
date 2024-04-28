use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;

use super::ClockDuration;

#[derive(Debug, Error)]
pub enum InvalidClockDurationString {
    #[error("Text format for clock duration is invalid")]
    InvalidFormat,
    #[error("Value for hour is not valid")]
    InvalidHour,
    #[error("Value for minutes is not valid")]
    InvalidMinutes,
    #[error("Value for seconds is not valid")]
    InvalidSeconds,
    #[error("Value for hours must not be negative")]
    NegativeHours,
    #[error("Value for minutes must not be negative")]
    NegativeMinutes,
    #[error("Value for seconds must not be negative")]
    NegativeSeconds,
}

static REGEX_CLOCK_DURATION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?<hour>\d{1,2}):(?<minutes>\d{1,2}):(?<seconds>\d{1,2})").unwrap());

impl FromStr for ClockDuration {
    type Err = InvalidClockDurationString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let groups = REGEX_CLOCK_DURATION
            .captures(s)
            .ok_or(InvalidClockDurationString::InvalidFormat)?;
        let hours: i64 = groups
            .name("hour")
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| InvalidClockDurationString::InvalidHour)?;
        let minutes: i64 = groups
            .name("minutes")
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| InvalidClockDurationString::InvalidMinutes)?;
        let seconds: i64 = groups
            .name("seconds")
            .unwrap()
            .as_str()
            .parse()
            .map_err(|_| InvalidClockDurationString::InvalidSeconds)?;

        if hours.is_negative() {
            return Err(InvalidClockDurationString::NegativeHours);
        }
        if minutes.is_negative() {
            return Err(InvalidClockDurationString::NegativeMinutes);
        }
        if seconds.is_negative() {
            return Err(InvalidClockDurationString::NegativeSeconds);
        }

        let parsed = ClockDuration::new_secs_mins_hours(Some(hours), Some(minutes), Some(seconds));
        Ok(parsed)
    }
}
