use corrupted_clock_util::timing::{CountDown, Stopwatch, TimeImpl, Timer, UtcDateTime};
use prettytable::{Cell, Row, Table};

use crate::constants::NOT_AVIABLE_TXT;

pub fn count_down_rows<'a, T>(
    count_downs: impl IntoIterator<Item = (&'a str, &'a CountDown<T>)>,
) -> Table
where
    T: Default + TimeImpl + 'a,
{
    let count_downs = count_downs.into_iter();
    let mut table = Table::new();
    let header = stopwatch_header().chain(count_down_headers()).collect();
    table.add_row(Row::new(header));
    for name_count_down in count_downs {
        let (_, cd) = name_count_down;
        let row = stopwatch_fields(name_count_down).chain(count_down_fields(cd));
        table.add_row(row.collect());
    }
    table
}

pub fn stop_watch_rows<'a, T>(
    count_downs: impl IntoIterator<Item = (&'a str, &'a Stopwatch<T>)>,
) -> Table
where
    T: Default + TimeImpl + 'a,
{
    type CellVec = Vec<Cell>;
    let stop_watches = count_downs.into_iter();

    let mut table = Table::new();
    let headers: CellVec = stopwatch_header().collect();
    let elements_num = headers.len();
    table.add_row(Row::new(headers));

    for name_stop_watch in stop_watches {
        let fields: CellVec = stopwatch_fields(name_stop_watch)
            .map(|e| Cell::new(&e))
            .collect();
        debug_assert_eq!(elements_num, fields.len());
        table.add_row(fields.into());
    }
    table
}

fn stopwatch_header() -> impl Iterator<Item = Cell> {
    [
        "Name",
        "Created at",
        "Started at",
        "Is paused",
        "Passed Time",
        "Paused Time",
        "Last resumed at",
        "Last paused at",
    ]
    .map(Cell::new)
    .into_iter()
}

fn stopwatch_fields<'a, T>((name, stop_watch): (&'a str, &'a T)) -> impl Iterator<Item = String>
where
    T: Timer + 'a,
{
    fn to_local_short_table_field(date: UtcDateTime) -> String {
        corrupted_clock_util::chrono_time_to_str(corrupted_clock_util::convert_utc_to_local(date))
    }

    fn convert_to_opt_table_field(opt: Option<UtcDateTime>) -> String {
        if let Some(date) = opt {
            to_local_short_table_field(date)
        } else {
            NOT_AVIABLE_TXT.to_string()
        }
    }

    let local_create_at = to_local_short_table_field(stop_watch.created_at());
    let local_started_at = to_local_short_table_field(stop_watch.start_moment());
    let last_resumed = convert_to_opt_table_field(stop_watch.last_resumed_at());
    let last_paused = convert_to_opt_table_field(stop_watch.last_paused_at());

    [
        name.to_string(),
        local_create_at,
        local_started_at,
        stop_watch.is_paused().to_string(),
        stop_watch.passed().to_string(),
        stop_watch.paused_time().to_string(),
        last_resumed,
        last_paused,
    ]
    .into_iter()
}

fn count_down_headers() -> impl Iterator<Item = Cell> {
    ["Count down", "Left Time"].map(Cell::new).into_iter()
}

fn count_down_fields<'a, T>(cd: &CountDown<T>) -> impl Iterator<Item = String>
where
    T: Default + TimeImpl,
{
    [cd.count_down_time().to_string(), cd.left_time().to_string()].into_iter()
}
