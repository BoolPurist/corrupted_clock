mod from_str;
use chrono::TimeDelta;
use serde::{Deserialize, Serialize};

use super::ChronoDuration;

const SECONDS_RATE: i64 = 60;
const MINUTES_RATE: i64 = SECONDS_RATE * SECONDS_RATE;
const HOURS_RATE: i64 = MINUTES_RATE * 24;

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClockDuration {
    secs: i64,
    nanos: i32,
}

impl ClockDuration {
    pub fn new_secs_mins_hours(
        hours: Option<i64>,
        minutes: Option<i64>,
        secs: Option<i64>,
    ) -> Self {
        let delta = TimeDelta::hours(hours.unwrap_or_default())
            + TimeDelta::minutes(minutes.unwrap_or_default())
            + TimeDelta::seconds(secs.unwrap_or_default());
        delta.into()
    }
}

impl std::ops::Sub for ClockDuration {
    type Output = ClockDuration;

    fn sub(self, rhs: Self) -> Self::Output {
        let (left, right): (ChronoDuration, ChronoDuration) = (self.into(), rhs.into());
        let delta = left - right;
        delta.into()
    }
}
impl std::ops::Add for ClockDuration {
    type Output = ClockDuration;

    fn add(self, rhs: Self) -> Self::Output {
        let (left, right): (ChronoDuration, ChronoDuration) = (self.into(), rhs.into());
        let delta = left + right;
        delta.into()
    }
}

impl ClockDuration {
    pub fn total_secs(&self) -> i64 {
        self.secs
    }
    pub fn nanos_part(&self) -> i32 {
        self.nanos
    }

    pub fn secs(&self) -> i32 {
        (self.secs % SECONDS_RATE) as i32
    }
    pub fn mins(&self) -> i32 {
        ((self.secs / SECONDS_RATE) % SECONDS_RATE) as i32
    }
    pub fn hours(&self) -> i32 {
        ((self.secs / MINUTES_RATE) % HOURS_RATE) as i32
    }
}

impl std::fmt::Display for ClockDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hours(),
            self.mins(),
            self.secs()
        )
    }
}

impl From<ClockDuration> for ChronoDuration {
    fn from(value: ClockDuration) -> Self {
        ChronoDuration::new(value.secs, value.nanos as u32).unwrap()
    }
}

impl From<ChronoDuration> for ClockDuration {
    fn from(value: ChronoDuration) -> Self {
        let (secs, nanos) = (value.num_seconds(), value.subsec_nanos());
        Self { secs, nanos }
    }
}
