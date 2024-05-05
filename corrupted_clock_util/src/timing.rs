pub use clock_duration::ClockDuration;
pub use clock_table::ClockTable;
pub use count_down::CountDown;
use serde::{Deserialize, Serialize};
pub use stopwatch::Stopwatch;

pub mod mocking_time;
#[cfg(test)]
pub mod test_utils;

mod clock_duration;
mod clock_table;
mod count_down;
mod stopwatch;

pub type UtcDateTime = DateTime<Utc>;
pub type ChronoDuration = chrono::Duration;

use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Default, Serialize, Deserialize)]
pub struct UtcTimeImpl;

#[derive(Debug, Error, Clone, Copy)]
#[error("'{0}' as a start date must not be in the future")]
pub struct InvalidDateInFuture(UtcDateTime);

pub fn validate_if_date_is_not_in_future(
    time: &impl TimeImpl,
    to_check: UtcDateTime,
) -> Result<(), InvalidDateInFuture> {
    let now = time.now();
    if now.timestamp() < to_check.timestamp() {
        Err(InvalidDateInFuture(to_check))
    } else {
        Ok(())
    }
}

pub trait Timer {
    fn created_at(&self) -> UtcDateTime;
    fn start_moment(&self) -> UtcDateTime;
    fn last_resumed_at(&self) -> Option<UtcDateTime>;
    fn last_paused_at(&self) -> Option<UtcDateTime>;
    fn passed(&self) -> ClockDuration;
    fn paused_time(&self) -> ClockDuration;
    fn is_paused(&self) -> bool;
    fn pause(&mut self);
    fn resume(&mut self);
    fn reset(&mut self);
}

pub trait TimeImpl {
    fn now(&self) -> UtcDateTime;
}

impl<'a, T> TimeImpl for &'a T
where
    T: TimeImpl,
{
    fn now(&self) -> UtcDateTime {
        (*self).now()
    }
}

impl TimeImpl for UtcTimeImpl {
    fn now(&self) -> UtcDateTime {
        Utc::now()
    }
}
