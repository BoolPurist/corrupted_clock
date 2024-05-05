use std::str::FromStr;

use chrono::{offset::LocalResult, TimeZone, Utc};
use once_cell::sync::Lazy;
use regex::{Captures, Regex};

use crate::timing::UtcDateTime;

static REGEX_DATE: Lazy<Regex> = Lazy::new(|| {
    const DATE: &str = r"(?<year>\d+)-(?<month>[0-1]?\d)-(?<day>[0-3]?\d)";
    const TIME: &str = r"(?<hours>\d{1,2}):(?<minutes>\d{1,2}):(?<seconds>\d{1,2})";

    Regex::new(&format!("{}{}{}", DATE, r"[\s,_]+", TIME)).unwrap()
});
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ParsedDate(UtcDateTime);

impl From<ParsedDate> for UtcDateTime {
    fn from(value: ParsedDate) -> Self {
        value.0
    }
}

use thiserror::Error;
#[derive(Debug, PartialEq, Eq, Error, Clone, Copy)]
pub enum InvalidDateFormat {
    #[error("Given input is an invalid format for a date")]
    InvalidDateFromat,
    #[error("No is no valid number for the year provided")]
    NoYear,
    #[error("There is no number for month between 1 and 12")]
    NoMonth,
    #[error("There is no number for day between 1 and 31")]
    NoDay,
    #[error("There is no number for hours between 0 and 23")]
    NoHours,
    #[error("There is no number for minutes between 0 and 59")]
    NoMinutes,
    #[error("There is no number for seconds between 0 and 59")]
    NoSeconds,
    #[error("there is no date mapping to given input")]
    NoValidDate,
    #[error("There is more date than one date for the given input")]
    Ambiguous,
}

impl FromStr for ParsedDate {
    type Err = InvalidDateFormat;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_comp<T>(
            cap: &Captures,
            key: &str,
            err: InvalidDateFormat,
        ) -> Result<T, InvalidDateFormat>
        where
            T: FromStr,
        {
            cap.name(key)
                .ok_or(err.clone())?
                .as_str()
                .parse()
                .map_err(|_| err)
        }

        let matched = REGEX_DATE
            .captures(s)
            .ok_or_else(|| InvalidDateFormat::InvalidDateFromat)?;

        let (year, month, day, hour, min, sec) = (
            parse_comp(&matched, "year", InvalidDateFormat::NoYear)?,
            parse_comp(&matched, "month", InvalidDateFormat::NoMonth)?,
            parse_comp(&matched, "day", InvalidDateFormat::NoDay)?,
            parse_comp(&matched, "hours", InvalidDateFormat::NoHours)?,
            parse_comp(&matched, "minutes", InvalidDateFormat::NoMonth)?,
            parse_comp(&matched, "seconds", InvalidDateFormat::NoSeconds)?,
        );

        if day > 31 {
            return Err(InvalidDateFormat::NoDay);
        }
        if day == 0 {
            return Err(InvalidDateFormat::NoDay);
        }
        if month == 0 {
            return Err(InvalidDateFormat::NoMonth);
        }
        if month > 12 {
            return Err(InvalidDateFormat::NoMonth);
        }

        if hour > 23 {
            return Err(InvalidDateFormat::NoHours);
        }
        if min > 59 {
            return Err(InvalidDateFormat::NoMinutes);
        }
        if sec > 59 {
            return Err(InvalidDateFormat::NoSeconds);
        }

        match Utc.with_ymd_and_hms(year, month, day, hour, min, sec) {
            LocalResult::Single(date) => Ok(ParsedDate(date)),
            LocalResult::Ambiguous(_, _) => Err(InvalidDateFormat::Ambiguous),
            LocalResult::None => Err(InvalidDateFormat::NoValidDate),
        }
    }
}

#[cfg(test)]
mod testing {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn regex_on_date() {
        fn assert_case(input: &str, expected: Result<ParsedDate, InvalidDateFormat>) {
            let actual = input.parse();
            assert_eq!(expected, actual, "Given input: {}", input);
        }
        assert_case(
            "2022-02-01 02:04:02",
            Ok(ParsedDate(
                Utc.with_ymd_and_hms(2022, 2, 1, 2, 4, 2).unwrap(),
            )),
        );
        assert_case(
            "2022-2-1 2:4:2",
            Ok(ParsedDate(
                Utc.with_ymd_and_hms(2022, 2, 1, 2, 4, 2).unwrap(),
            )),
        );
        assert_case(
            "2022-10-15 00:00:00",
            Ok(ParsedDate(
                Utc.with_ymd_and_hms(2022, 10, 15, 0, 0, 0).unwrap(),
            )),
        );
        assert_case(
            "2022-10-15__00:00:00",
            Ok(ParsedDate(
                Utc.with_ymd_and_hms(2022, 10, 15, 0, 0, 0).unwrap(),
            )),
        );
        assert_case(
            "2022-12-31__23:59:59",
            Ok(ParsedDate(
                Utc.with_ymd_and_hms(2022, 12, 31, 23, 59, 59).unwrap(),
            )),
        );
        assert_case(
            "2022-01-01 00:59:59",
            Ok(ParsedDate(
                Utc.with_ymd_and_hms(2022, 1, 1, 0, 59, 59).unwrap(),
            )),
        );
        assert_case("2022-12-32__23:59:59", Err(InvalidDateFormat::NoDay));
        assert_case("2022-12-00__23:59:59", Err(InvalidDateFormat::NoDay));
        assert_case("2022-00-01__23:59:59", Err(InvalidDateFormat::NoMonth));
        assert_case("2022-13-01__23:59:59", Err(InvalidDateFormat::NoMonth));
        assert_case("2022-01-01 23:60:59", Err(InvalidDateFormat::NoMinutes));
        assert_case("2022-01-01 23:59:60", Err(InvalidDateFormat::NoSeconds));
    }
}
