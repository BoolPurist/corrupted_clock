use corrupted_clock_util::timing::{CountDown, Stopwatch, Timer};
use prettytable::{row, Table};

pub fn count_down_rows<'a>(
    count_downs: impl IntoIterator<Item = (&'a str, &'a CountDown)>,
) -> Table {
    let count_downs = count_downs.into_iter();
    let mut table = Table::new();
    table.add_row(row![
        "Name",
        "Created at",
        "Is paused",
        "Passed Time",
        "Paused Time",
        "Left Time"
    ]);
    for name_count_down in count_downs {
        let (_, cd) = name_count_down;
        let row = basic_fields(name_count_down).chain([cd.left_time().to_string()]);
        table.add_row(row.collect());
    }
    table
}

pub fn stop_watch_rows<'a>(
    count_downs: impl IntoIterator<Item = (&'a str, &'a Stopwatch)>,
) -> Table {
    let stop_watches = count_downs.into_iter();

    let mut table = Table::new();
    table.add_row(row![
        "Name",
        "Created at",
        "Is paused",
        "Passed Time",
        "Paused Time"
    ]);

    for name_stop_watch in stop_watches {
        table.add_row(basic_fields(name_stop_watch).collect());
    }
    table
}

fn basic_fields<'a>((name, stop_watch): (&'a str, &'a impl Timer)) -> impl Iterator<Item = String> {
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
