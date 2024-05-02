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

#[derive(Default, Serialize, Deserialize)]
pub struct UtcTimeImpl;

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

impl TimeImpl for UtcTimeImpl {
    fn now(&self) -> UtcDateTime {
        Utc::now()
    }
}
