use serde::{Deserialize, Serialize};

use super::{
    ClockDuration, InvalidDateInFuture, Stopwatch, TimeImpl, Timer, UtcDateTime, UtcTimeImpl,
};

#[derive(Serialize, Deserialize)]
pub struct CountDown<T = UtcTimeImpl>
where
    T: Default,
{
    stopwatch: Stopwatch<T>,
    time: ClockDuration,
}

impl<T> std::fmt::Debug for CountDown<T>
where
    T: Default,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CountDown")
            .field("stopwatch", &self.stopwatch)
            .field("time", &self.time)
            .finish()
    }
}

impl CountDown {
    pub fn new(time: ClockDuration) -> Self {
        let stopwatch: Stopwatch = Stopwatch::new();
        Self { stopwatch, time }
    }

    pub fn new_with_start(
        time: ClockDuration,
        start: UtcDateTime,
    ) -> Result<Self, InvalidDateInFuture> {
        let stopwatch: Stopwatch = Stopwatch::new_with_impl_and_start_date(UtcTimeImpl, start)?;
        Ok(Self { stopwatch, time })
    }
}

impl<T> CountDown<T>
where
    T: TimeImpl + Default,
{
    pub fn count_down_time(&self) -> ClockDuration {
        self.time
    }

    pub fn new_with_impl(time_impl: T, time: ClockDuration) -> Self {
        let stopwatch = Stopwatch::new_with_impl(time_impl);
        Self { stopwatch, time }
    }

    pub fn left_time(&self) -> ClockDuration {
        let left = self.time - self.passed();
        debug_assert!(left.total_secs() >= 0);
        debug_assert!(left.nanos_part() >= 0);
        left
    }

    pub fn stopwatch(&self) -> &Stopwatch<T> {
        &self.stopwatch
    }
}

impl<T> Timer for CountDown<T>
where
    T: TimeImpl + Default,
{
    fn passed(&self) -> ClockDuration {
        let actual_passed = self.stopwatch.passed();
        actual_passed.min(self.time)
    }

    fn pause(&mut self) {
        self.stopwatch.pause();
    }

    fn resume(&mut self) {
        self.stopwatch.resume();
    }

    fn reset(&mut self) {
        self.stopwatch.reset();
    }

    fn is_paused(&self) -> bool {
        self.stopwatch.is_paused()
    }

    fn created_at(&self) -> super::UtcDateTime {
        self.stopwatch().created_at()
    }

    fn paused_time(&self) -> ClockDuration {
        self.stopwatch().paused_time()
    }

    fn last_resumed_at(&self) -> Option<super::UtcDateTime> {
        self.stopwatch.last_resumed_at()
    }

    fn last_paused_at(&self) -> Option<super::UtcDateTime> {
        self.stopwatch.last_paused_at()
    }

    fn start_moment(&self) -> super::UtcDateTime {
        self.stopwatch.start_moment()
    }
}

#[cfg(test)]
mod testing {
    use chrono::TimeDelta;

    use crate::timing::{
        mocking_time::MockTimeImpl,
        test_utils::{add_to_now, new_utc_moment},
    };

    use super::*;

    fn set_up_counte_mock(
        input: &str,
        duration: ClockDuration,
    ) -> (CountDown<MockTimeImpl>, MockTimeImpl) {
        let time_impl = MockTimeImpl::new(new_utc_moment(input));
        (
            CountDown::new_with_impl(time_impl.clone(), duration),
            time_impl,
        )
    }

    fn assert_left_time(count_down: &CountDown<MockTimeImpl>, expected: ClockDuration) {
        let left_time = count_down.left_time();
        pretty_assertions::assert_eq!(expected, left_time);
    }

    #[test]
    fn count_down_to_left_time() {
        let intial_time = ClockDuration::new_secs_mins_hours(Some(2), None, None);
        let (count_down, mut setter) = set_up_counte_mock("2020-02-11 12:00:00", intial_time);
        assert_left_time(&count_down, intial_time);
        add_to_now(&mut setter, TimeDelta::hours(1));
        add_to_now(&mut setter, TimeDelta::hours(2));
        // Count should be finished now
        assert_left_time(&count_down, ClockDuration::default());
    }
}
