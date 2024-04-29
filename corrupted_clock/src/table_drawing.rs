use corrupted_clock_util::timing::{CountDown, Stopwatch, Timer};
use prettytable::{Cell, Row, Table};

fn stopwatch_header() -> impl Iterator<Item = Cell> {
    [
        "Name",
        "Created at",
        "Is paused",
        "Passed Time",
        "Paused Time",
    ]
    .map(Cell::new)
    .into_iter()
}

fn count_down_headers() -> impl Iterator<Item = Cell> {
    ["Count down", "Left Time"].map(Cell::new).into_iter()
}

pub fn count_down_rows<'a>(
    count_downs: impl IntoIterator<Item = (&'a str, &'a CountDown)>,
) -> Table {
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

pub fn stop_watch_rows<'a>(
    count_downs: impl IntoIterator<Item = (&'a str, &'a Stopwatch)>,
) -> Table {
    let stop_watches = count_downs.into_iter();

    let mut table = Table::new();
    table.add_row(Row::new(stopwatch_header().collect()));

    for name_stop_watch in stop_watches {
        table.add_row(stopwatch_fields(name_stop_watch).collect());
    }
    table
}

fn stopwatch_fields<'a>(
    (name, stop_watch): (&'a str, &'a impl Timer),
) -> impl Iterator<Item = String> {
    let local_create_at = corrupted_clock_util::convert_utc_to_local(stop_watch.created_at());
    [
        name.to_string(),
        local_create_at.to_string(),
        stop_watch.is_paused().to_string(),
        stop_watch.passed().to_string(),
        stop_watch.paused_time().to_string(),
    ]
    .into_iter()
}

fn count_down_fields<'a>(cd: &CountDown) -> impl Iterator<Item = String> {
    [cd.count_down_time().to_string(), cd.left_time().to_string()].into_iter()
}
