use std::collections::VecDeque;

use corrupted_clock_util::timing::{CountDown, Stopwatch, TimeImpl, Timer, UtcDateTime};
use prettytable::{Cell, Row, Table};

use crate::{constants::NOT_AVIABLE_TXT, listing_items_param::ListingItemsParams, AtLeastOne};

pub fn count_down_rows<'a, T>(
    list_args: ListingItemsParams,
    count_downs: impl IntoIterator<Item = (&'a str, &'a CountDown<T>)>,
) -> String
where
    T: Default + TimeImpl + 'a,
{
    item_rows(
        list_args,
        count_downs,
        || stopwatch_header().chain(count_down_headers()),
        |name_stop_watch| {
            stopwatch_fields((name_stop_watch.0, name_stop_watch.1.stopwatch()))
                .chain(count_down_fields(name_stop_watch.1))
        },
    )
}

pub fn stop_watch_rows<'a, T>(
    list_args: ListingItemsParams,
    count_downs: impl IntoIterator<Item = (&'a str, &'a Stopwatch<T>)>,
) -> String
where
    T: Default + TimeImpl + 'a,
{
    item_rows(
        list_args,
        count_downs,
        || stopwatch_header(),
        |name_stop_watch| stopwatch_fields(name_stop_watch),
    )
}

fn item_rows<'a, T: 'a, F, H>(
    list_args: ListingItemsParams,
    count_downs: impl IntoIterator<Item = (&'a str, &'a T)>,
    on_headers: impl FnOnce() -> H,
    on_fields: impl Fn((&str, &T)) -> F,
) -> String
where
    F: Iterator<Item = String>,
    H: Iterator<Item = Cell>,
{
    let stop_watches: Vec<_> = count_downs.into_iter().collect();

    let mut headers: VecDeque<Cell> = on_headers().collect();
    let elements_num = headers.len();
    let colum_steps = column_draw_steps(elements_num as u32, list_args.column_num());
    let mut tables: Vec<Table> = std::iter::repeat(Table::new())
        .take(colum_steps.len())
        .collect();

    for (next, &split_off_at) in tables.iter_mut().zip(colum_steps.iter()) {
        let split_off_at = split_off_at as usize;
        let next_slice = headers.drain(0..split_off_at);
        next.add_row(Row::from_iter(next_slice));
    }

    for name_stop_watch in stop_watches {
        let mut fields = on_fields(name_stop_watch).map(|e| Cell::new(&e));
        for (table, &column_num) in tables.iter_mut().zip(colum_steps.iter()) {
            let for_row = fields.by_ref().take(column_num as usize);
            table.add_row(Row::from_iter(for_row));
        }
    }

    tables
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n")
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

fn column_draw_steps(colum_num: u32, opt_column_num_per_row: Option<AtLeastOne>) -> Vec<u32> {
    match opt_column_num_per_row {
        None => std::iter::once(colum_num).collect(),
        Some(per_row) => {
            let num_per_row = per_row.value();
            if num_per_row >= colum_num {
                std::iter::once(colum_num).collect()
            } else {
                let module_column: u32 = colum_num % num_per_row;
                let div_column: u32 = colum_num / num_per_row;
                let rows = std::iter::repeat(num_per_row).take(div_column as usize);
                if module_column == 0 {
                    rows.collect()
                } else {
                    rows.chain(std::iter::once(module_column)).collect()
                }
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn colum_draw_steps_more_per_row_than_columns() {
        let actual = column_draw_steps(2, Some(AtLeastOne::new(4).unwrap()));
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn colum_draw_steps_more_columns_than_per_row_even() {
        let actual = column_draw_steps(8, Some(AtLeastOne::new(4).unwrap()));
        insta::assert_debug_snapshot!(actual);
    }

    #[test]
    fn colum_draw_steps_more_columns_than_per_row_uneven() {
        let actual = column_draw_steps(10, Some(AtLeastOne::new(4).unwrap()));
        insta::assert_debug_snapshot!(actual);
    }
}
