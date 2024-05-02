use serde::{Deserialize, Serialize};

use super::{ChronoDuration, ClockDuration, TimeImpl, Timer, UtcDateTime, UtcTimeImpl};

#[derive(Serialize, Deserialize)]
pub struct Stopwatch<T = UtcTimeImpl>
where
    T: Default,
{
    start_moment: UtcDateTime,
    last_resume_moment: Option<UtcDateTime>,
    last_paused_at: Option<UtcDateTime>,
    is_paused: bool,
    paused_time: ClockDuration,
    passed_time_between_pauses: ClockDuration,
    #[serde(skip)]
    time_impl: T,
}

impl Stopwatch<UtcTimeImpl> {
    pub fn new() -> Self {
        Self::new_with_impl(UtcTimeImpl::default())
    }
}
impl<T> Stopwatch<T>
where
    T: Default,
{
    pub fn start_moment(&self) -> UtcDateTime {
        self.start_moment
    }

    pub fn last_paused_at(&self) -> Option<UtcDateTime> {
        self.last_paused_at
    }
    pub fn last_resumed_at(&self) -> Option<UtcDateTime> {
        self.last_resume_moment
    }

    fn resume_moment(&self) -> UtcDateTime {
        self.last_resumed_at().unwrap_or(self.start_moment)
    }

    fn pause_moment(&self) -> UtcDateTime {
        self.last_paused_at().unwrap_or(self.start_moment)
    }
}

impl<T> Timer for Stopwatch<T>
where
    T: TimeImpl + Default,
{
    fn passed(&self) -> ClockDuration {
        if self.is_paused() {
            self.passed_time_between_pauses
        } else {
            let now = self.time_impl.now();
            let delta_and_last_resumed = now - self.resume_moment();
            let delta = delta_and_last_resumed + self.passed_time_between_pauses.into();
            delta.into()
        }
    }

    fn pause(&mut self) {
        if self.is_paused() {
            return;
        }
        self.is_paused = true;
        let now = self.time_impl.now();
        self.last_paused_at = Some(now);
        let to_add = now - self.resume_moment();
        self.passed_time_between_pauses =
            (ChronoDuration::from(self.passed_time_between_pauses) + to_add).into();
    }

    fn resume(&mut self) {
        if !self.is_paused() {
            return;
        }
        self.is_paused = false;
        let now = self.time_impl.now();
        self.paused_time = self.paused_time + (now - self.resume_moment()).into();
        self.last_resume_moment = Some(now);
    }

    fn reset(&mut self) {
        self.start_moment = self.time_impl.now();
        self.is_paused = false;
        self.last_resume_moment = None;
        self.last_paused_at = None;
        self.passed_time_between_pauses = Default::default();
        self.paused_time = Default::default();
    }

    fn is_paused(&self) -> bool {
        self.is_paused
    }

    fn created_at(&self) -> UtcDateTime {
        self.start_moment
    }

    fn paused_time(&self) -> ClockDuration {
        let current_pause_delta = if self.is_paused() {
            (self.time_impl.now() - self.pause_moment()).into()
        } else {
            Default::default()
        };
        self.paused_time + current_pause_delta
    }
}

impl<T> Stopwatch<T>
where
    T: TimeImpl + Default,
{
    pub fn new_with_impl(time_impl: T) -> Self {
        let now = time_impl.now();
        Self::new_with_impl_and_start_date(time_impl, now)
    }

    pub fn new_with_impl_and_start_date(time_impl: T, date: UtcDateTime) -> Self {
        let start_moment = date;
        let paused_time = Default::default();
        Self {
            start_moment,
            paused_time,
            passed_time_between_pauses: paused_time,
            time_impl,
            is_paused: false,
            last_paused_at: None,
            last_resume_moment: None,
        }
    }
}

impl<T> std::fmt::Debug for Stopwatch<T>
where
    T: Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Stopwatch")
            .field("Started", &self.start_moment)
            .field("Paused", &self.passed_time_between_pauses)
            .finish()
    }
}

#[cfg(test)]
mod testing {

    use chrono::TimeDelta;

    use crate::timing::mocking_time::MockTimeImpl;
    use crate::timing::test_utils::{self, add_to_now};

    use super::*;

    #[test]
    fn start_watch_and_progress() {
        let (watcher, setter) = test_utils::new_mocked_stopwatch("2000-11-11 11:11:11");
        let start = watcher.start_moment();
        let later = start
            .checked_add_signed(TimeDelta::minutes(10) + TimeDelta::seconds(51))
            .unwrap();
        setter.set_now(later);
        let passed = watcher.passed().to_string();
        pretty_assertions::assert_eq!("00:10:51", passed);
    }

    fn assert_passed(watcher: &Stopwatch<MockTimeImpl>, expected: &str) {
        let passed = watcher.passed().to_string();
        pretty_assertions::assert_eq!(expected, passed);
    }

    #[test]
    fn reset_watch() {
        let (mut watcher, mut setter) = test_utils::new_mocked_stopwatch("2000-01-10 10:00:00");

        // Now: 2000-01-10 10:10:51
        add_to_now(&mut setter, TimeDelta::minutes(10));
        assert_passed(&watcher, "00:10:00");
        watcher.reset();
        assert_passed(&watcher, "00:00:00");
        add_to_now(&mut setter, TimeDelta::minutes(20));
        assert_passed(&watcher, "00:20:00");
    }

    #[test]
    fn pause_and_resume_watch() {
        let (mut watcher, mut setter) = test_utils::new_mocked_stopwatch("2000-01-10 10:00:00");

        // Now: 2000-01-10 10:10:51
        add_to_now(&mut setter, TimeDelta::minutes(10) + TimeDelta::seconds(51));
        // Checking if times passed after no pausing or resuming
        assert_passed(&watcher, "00:10:51");

        watcher.pause();
        // Now: 2000-01-10 11:10:51
        add_to_now(&mut setter, TimeDelta::hours(1));
        // Checking if one hour is ignored becauses of pause
        assert_passed(&watcher, "00:10:51");
        watcher.resume();
        // Now: 2000-01-10 12:10:51
        add_to_now(&mut setter, TimeDelta::hours(1));
        // Checking if new one hour is considered after resume
        assert_passed(&watcher, "01:10:51");

        watcher.pause();
        // Now: 2000-01-10 15:10:51
        add_to_now(&mut setter, TimeDelta::hours(3));
        watcher.resume();
        // Now: 2000-01-10 18:10:51
        add_to_now(&mut setter, TimeDelta::hours(3));
        assert_passed(&watcher, "04:10:51");
    }
}
