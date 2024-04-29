use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

use super::ClockDuration;

#[derive(Debug, Error, PartialEq, Eq, Clone, Copy)]
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

impl FromStr for ClockDuration {
    type Err = InvalidClockDurationString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn group_to_number<'h>(matched: Option<&str>) -> Option<Result<i64, ParseIntError>> {
            matched.map(|element| element.parse())
        }

        fn check_if_positive(
            input: Result<i64, ParseIntError>,
            invalid_error: InvalidClockDurationString,
            negative_error: InvalidClockDurationString,
        ) -> Result<i64, InvalidClockDurationString> {
            match input {
                Ok(number) => {
                    if number.is_negative() {
                        Err(negative_error)
                    } else {
                        Ok(number)
                    }
                }
                Err(_) => Err(invalid_error),
            }
        }

        fn check_seconds(
            input: Result<i64, ParseIntError>,
        ) -> Result<i64, InvalidClockDurationString> {
            check_if_positive(
                input,
                InvalidClockDurationString::InvalidSeconds,
                InvalidClockDurationString::NegativeSeconds,
            )
        }

        fn check_minutes(
            input: Result<i64, ParseIntError>,
        ) -> Result<i64, InvalidClockDurationString> {
            check_if_positive(
                input,
                InvalidClockDurationString::InvalidMinutes,
                InvalidClockDurationString::NegativeMinutes,
            )
        }

        fn check_hours(
            input: Result<i64, ParseIntError>,
        ) -> Result<i64, InvalidClockDurationString> {
            check_if_positive(
                input,
                InvalidClockDurationString::InvalidHour,
                InvalidClockDurationString::NegativeHours,
            )
        }

        let mut splitted = s.splitn(3, ":");
        let (hours, minutes, secs) = match (
            group_to_number(splitted.next()),
            group_to_number(splitted.next()),
            group_to_number(splitted.next()),
        ) {
            (None, None, None) => return Err(InvalidClockDurationString::InvalidFormat),
            (Some(seconds), None, None) => (None, None, Some(check_seconds(seconds)?)),
            (Some(minutes), Some(seconds), None) => (
                None,
                Some(check_minutes(minutes)?),
                Some(check_seconds(seconds)?),
            ),
            (Some(hours), Some(minutes), Some(seconds)) => (
                Some(check_hours(hours)?),
                Some(check_minutes(minutes)?),
                Some(check_seconds(seconds)?),
            ),
            _not_expected => {
                unreachable!("Should not land here with this value {:#?}", _not_expected)
            }
        };
        Ok(ClockDuration::new_secs_mins_hours(hours, minutes, secs))
    }
}
#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn regex_on_duration_input() {
        fn assert_case(input: &str, expected: Result<ClockDuration, InvalidClockDurationString>) {
            let actual = input.parse();
            assert_eq!(expected, actual, "Actul input: '{}'", input);
        }
        assert_case(
            "20:12:13",
            Ok(ClockDuration::new_secs_mins_hours(
                Some(20),
                Some(12),
                Some(13),
            )),
        );
        assert_case(
            "01:100",
            Ok(ClockDuration::new_secs_mins_hours(None, Some(1), Some(100))),
        );
        assert_case(
            "234",
            Ok(ClockDuration::new_secs_mins_hours(None, None, Some(234))),
        );
        assert_case("aa", Err(InvalidClockDurationString::InvalidSeconds));
        assert_case("", Err(InvalidClockDurationString::InvalidSeconds));
        assert_case("232:aa", Err(InvalidClockDurationString::InvalidSeconds));
        assert_case("aa:22", Err(InvalidClockDurationString::InvalidMinutes));
        assert_case("aa:22:02", Err(InvalidClockDurationString::InvalidHour));
        assert_case(
            "01:22:-02",
            Err(InvalidClockDurationString::NegativeSeconds),
        );
        assert_case(
            "02:-22:02",
            Err(InvalidClockDurationString::NegativeMinutes),
        );
        assert_case("-1:00:00", Err(InvalidClockDurationString::NegativeHours));
    }
}
