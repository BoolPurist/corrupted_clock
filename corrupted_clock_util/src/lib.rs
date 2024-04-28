use chrono::{DateTime, Datelike, Timelike};

pub mod constants;
pub mod data_store;
pub mod prelude;
pub mod timing;

pub fn chrono_time_to_str<T>(date_time: DateTime<T>) -> String
where
    T: chrono::TimeZone,
{
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        date_time.year(),
        date_time.month(),
        date_time.day(),
        date_time.hour(),
        date_time.minute(),
        date_time.second(),
    )
}
