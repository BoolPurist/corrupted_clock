use chrono::{Local, TimeZone, Utc};
use clap::Args;
use corrupted_clock_util::{
    parsed_date::{InvalidDateFormat, ParsedDate},
    timing::{
        validate_if_date_is_not_in_future, ClockDuration, InvalidDateInFuture, UtcDateTime,
        UtcTimeImpl,
    },
};

#[derive(Debug, Args)]
pub struct CreateCommand {
    name: Option<String>,
    #[arg(short, long)]
    /// # Syntax for option value
    ///
    /// Syntax for number: [0-9]+ => sequence of digits from 0 to 9
    ///
    /// Valid syntax for duration for hours, minutes and seconds:
    ///
    /// <number>[:<number>][:<number>]
    ///
    /// Valid syntax for duration for minutes and seconds:
    ///
    /// <number>[:<number>]
    ///
    /// Valid syntax for duration for seconds:
    ///
    /// <number>
    ///
    /// # Examples
    ///
    /// Duration with 12 hours, 56 minutes and 12 seconds
    ///
    /// 12:56:12
    ///
    /// Duration with 56 minutes and 12 seconds
    ///
    /// 56:12
    ///
    /// Duration with 12 seconds
    ///
    /// 12
    to_count_down: Option<ClockDuration>,
    #[arg(short, long ,value_parser = parse_start_date)]
    /// Valid syntax: <year>-<month>-<day> <hours>:<minutes>:<seconds>
    /// year: positive number
    /// months: [0-1]?<1-9>
    /// year: positive number
    /// year: positive number
    /// year: positive number
    /// year: positive number
    start_date: Option<UtcDateTime>,
}

use thiserror::Error;

#[derive(Debug, Clone, Copy, Error)]
pub enum InvalidStartDate {
    #[error("{0}")]
    InFuture(#[from] InvalidDateInFuture),
    #[error("{0}")]
    Invalid(#[from] InvalidDateFormat),
}

impl CreateCommand {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn to_count_down(&self) -> Option<ClockDuration> {
        self.to_count_down
    }

    pub fn start_date(&self) -> Option<UtcDateTime> {
        self.start_date
    }
}

fn parse_start_date(s: &str) -> Result<UtcDateTime, InvalidStartDate> {
    let valid_date: ParsedDate = s.parse()?;
    let date: UtcDateTime = valid_date.into();
    validate_if_date_is_not_in_future(&UtcTimeImpl, date)?;
    let as_local = Local.from_local_datetime(&date.naive_utc()).unwrap();
    let date = Utc.from_local_datetime(&as_local.naive_utc()).unwrap();
    Ok(date)
}
