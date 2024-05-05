use chrono::{DateTime, Datelike, Local, Timelike, Utc};

pub mod constants;
pub mod data_store;
pub mod parsed_date;
pub mod prelude;
pub mod timing;

pub fn convert_utc_to_local(date_time: DateTime<Utc>) -> DateTime<Local> {
    date_time.into()
}

pub fn local_now() -> DateTime<Local> {
    Local::now()
}

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
