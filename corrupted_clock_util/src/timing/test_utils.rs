use chrono::{DateTime, NaiveDateTime, TimeDelta, Utc};

use crate::timing::mocking_time::MockTimeImpl;

use super::*;

pub fn new_utc_moment(input: &str) -> UtcDateTime {
    const FMT: &str = "%Y-%m-%d %H:%M:%S";
    let time = NaiveDateTime::parse_from_str(input, FMT).unwrap();
    DateTime::from_naive_utc_and_offset(time, Utc)
}

pub fn add_to_now(setter: &mut MockTimeImpl, delta: TimeDelta) {
    let later = setter.now().checked_add_signed(delta).unwrap();
    setter.set_now(later);
}

pub fn new_mocked_stopwatch(input: &str) -> (Stopwatch<MockTimeImpl>, MockTimeImpl) {
    let time_impl = MockTimeImpl::new(new_utc_moment(input));
    (Stopwatch::new_with_impl(time_impl.clone()), time_impl)
}
