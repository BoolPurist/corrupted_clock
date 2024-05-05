mod not_found_clock_err;

use std::path::PathBuf;

use corrupted_clock_util::{
    data_store,
    timing::{ClockTable, CountDown, Stopwatch, TimeImpl, Timer as _, UtcTimeImpl},
};
use log::{info, warn};

use crate::{
    cli_args::{
        AppCliArgs, ClockKind, CreateCommand, ExistingClockKindReference, GetClockArgs, ListArgs,
        ManyClockReferenceKind,
    },
    path_utils, table_drawing, AppResult,
};

use self::not_found_clock_err::NotFoundClockErr;

pub fn create(general_args: &AppCliArgs, args: &CreateCommand) -> AppResult {
    let LoadedAppStateFile {
        path_to_app_file,
        mut app_state,
    } = load_app_state(general_args)?;

    let name = args
        .name()
        .map(|ref_str| ref_str.to_string())
        .unwrap_or_else(|| {
            let now = corrupted_clock_util::local_now();
            let name = corrupted_clock_util::chrono_time_to_str(now);
            info!(
                "Name of clock item is '{}' since no name was provided",
                &name
            );
            name
        });
    let opt_start_date = args.start_date();
    match args.to_count_down() {
        Some(count_down) => {
            info!("Stopwatch under the name '{}' is created", name);
            let count_down = if let Some(start_date) = opt_start_date {
                CountDown::new_with_start(count_down, start_date)?
            } else {
                CountDown::new(count_down)
            };
            app_state.add_count_down(name, count_down)?;
        }
        None => {
            info!("Count down under the name '{}' is created", name);
            let stopwatch = if let Some(start_date) = opt_start_date {
                Stopwatch::new_with_impl_and_start_date(UtcTimeImpl, start_date)?
            } else {
                Stopwatch::new()
            };
            app_state.add_stopwatch(name, stopwatch)?
        }
    };

    data_store::save_app_state(&path_to_app_file, &app_state)?;
    Ok(())
}

pub fn resume(general_args: &AppCliArgs, args: &ExistingClockKindReference) -> AppResult {
    handle_modify_with_save(general_args, args, |sw| sw.resume(), |sw| sw.resume())
}

pub fn reset(general_args: &AppCliArgs, args: &ExistingClockKindReference) -> AppResult {
    handle_modify_with_save(general_args, args, |sw| sw.reset(), |cd| cd.reset())
}

pub fn pause(general_args: &AppCliArgs, args: &ExistingClockKindReference) -> AppResult {
    handle_modify_with_save(general_args, args, |sw| sw.pause(), |cd| cd.pause())
}

pub fn delete(general_args: &AppCliArgs, args: ExistingClockKindReference) -> AppResult {
    let LoadedAppStateFile {
        mut app_state,
        path_to_app_file,
    } = load_app_state(general_args)?;
    match args {
        ExistingClockKindReference::All(kind) => {
            let (remove_sws, remove_cds) = match kind {
                ManyClockReferenceKind::All => (true, true),
                ManyClockReferenceKind::Stopwatch => (true, false),
                ManyClockReferenceKind::CountDown => (false, true),
            };
            if remove_sws {
                app_state.remove_all_stopwatches();
                info!("All stop watches were removed");
            }
            if remove_cds {
                app_state.remove_all_count_donws();
                info!("All count downs were removed");
            }
        }
        ExistingClockKindReference::Single(single) => {
            let name = single.name();
            let kind = single.kind();
            match kind {
                ClockKind::StopWatch => {
                    if !app_state.has_stop_watch(name) {
                        return Err(NotFoundClockErr::new(name.to_owned(), kind).into());
                    } else {
                        app_state.remove_stopwatch(name);
                        info!("Stop watch under the name '{}' was removed", name);
                    }
                }
                ClockKind::CountDown => {
                    if !app_state.has_count_down(name) {
                        return Err(NotFoundClockErr::new(name.to_owned(), kind).into());
                    } else {
                        app_state.remove_count_down(name);
                        info!("Count down under the name '{}' was removed", name);
                    }
                }
            }
        }
    }

    data_store::save_app_state(&path_to_app_file, &app_state)?;
    Ok(())
}

pub fn list(general_args: &AppCliArgs, args: &ListArgs) -> AppResult<String> {
    let LoadedAppStateFile { app_state, .. } = load_app_state(general_args)?;

    let output = draw_tables_of_cds_sws(&app_state, args.kind());
    Ok(output)
}

pub fn get_clock(general_args: &AppCliArgs, args: &GetClockArgs) -> AppResult<String> {
    let LoadedAppStateFile { app_state, .. } = load_app_state(general_args)?;
    let referecne = args.reference();
    let name = referecne.name();
    match referecne.kind() {
        ClockKind::CountDown => {
            let count_down = app_state
                .get_count_down(name)
                .ok_or_else(|| NotFoundClockErr::new(name.to_owned(), ClockKind::CountDown))?;
            let table = table_drawing::count_down_rows([(name, count_down)].into_iter());
            Ok(table.to_string())
        }
        ClockKind::StopWatch => {
            let stop_watch = app_state
                .get_stopwatch(name)
                .ok_or_else(|| NotFoundClockErr::new(name.to_owned(), ClockKind::StopWatch))?;
            let table = table_drawing::stop_watch_rows([(name, stop_watch)].into_iter());
            Ok(table.to_string())
        }
    }
}

struct LoadedAppStateFile {
    path_to_app_file: PathBuf,
    app_state: ClockTable,
}

fn load_app_state(general_args: &AppCliArgs) -> AppResult<LoadedAppStateFile> {
    let user_dir = path_utils::get_user_data_dir(general_args)?;
    let app_state_file = path_utils::get_path_app_state_file(&user_dir);
    let app_state = data_store::load_app_state(&app_state_file).unwrap_or_else(|error| {
        warn!(
            "Stopwatches and count downs could not be loaded due the following error: {}",
            error
        );
        ClockTable::default()
    });
    Ok(LoadedAppStateFile {
        path_to_app_file: app_state_file,
        app_state,
    })
}

fn handle_modify_with_save(
    general_args: &AppCliArgs,
    reference: &ExistingClockKindReference,
    mut on_stopwatch: impl FnMut(&mut Stopwatch),
    mut on_count_down: impl FnMut(&mut CountDown),
) -> AppResult {
    let LoadedAppStateFile {
        mut app_state,
        path_to_app_file,
    } = load_app_state(general_args)?;
    match reference {
        ExistingClockKindReference::All(kind) => {
            let (on_sws, on_cds) = match kind {
                ManyClockReferenceKind::All => (true, true),
                ManyClockReferenceKind::Stopwatch => (true, false),
                ManyClockReferenceKind::CountDown => (false, true),
            };
            if on_sws {
                for stopwatch in app_state.mut_all_stop_watches() {
                    on_stopwatch(stopwatch);
                }
                info!("Modification was done on all stop watches");
            }
            if on_cds {
                for count_down in app_state.mut_all_count_downs() {
                    on_count_down(count_down);
                }
                info!("Modification was done on all count downs");
            }
        }
        ExistingClockKindReference::Single(reference) => {
            let name = reference.name();
            match reference.kind() {
                ClockKind::StopWatch => match app_state.mut_stopwatch(name) {
                    Some(sw) => {
                        info!(
                            "Modification was done on the stop watch with name `{}`",
                            name
                        );
                        on_stopwatch(sw)
                    }
                    None => {
                        return Err(NotFoundClockErr::new(name.to_owned(), reference.kind()).into())
                    }
                },
                ClockKind::CountDown => match app_state.mut_count_down(name) {
                    Some(sw) => {
                        info!(
                            "Modification was done on the count down with name `{}`",
                            name
                        );
                        on_count_down(sw)
                    }
                    None => {
                        return Err(NotFoundClockErr::new(name.to_owned(), reference.kind()).into())
                    }
                },
            }
        }
    }

    data_store::save_app_state(&path_to_app_file, &app_state)?;
    Ok(())
}

fn draw_tables_of_cds_sws<T>(app_state: &ClockTable<T>, clock_kind: Option<ClockKind>) -> String
where
    T: Default + TimeImpl,
{
    let (does_stopwatches, does_count_downs) = match clock_kind {
        Some(ClockKind::CountDown) => (false, true),
        Some(ClockKind::StopWatch) => (true, false),
        None => (true, true),
    };
    let mut output = String::default();

    if does_stopwatches {
        let stop_watches = {
            let mut to_sort: Vec<_> = app_state.all_stopwatches().collect();
            to_sort.sort_by(|(l_key, _), (r_key, _)| l_key.cmp(&r_key));
            to_sort
        };
        let sw_table = table_drawing::stop_watch_rows(stop_watches).to_string();
        let to_push = format!(
            "Stopwatches\n\
            {}\n",
            sw_table
        );
        output.push_str(&to_push);
    }
    if does_count_downs {
        let count_downs = {
            let mut to_sort: Vec<_> = app_state.all_count_downs().collect();
            to_sort.sort_by(|(l_key, _), (r_key, _)| l_key.cmp(&r_key));
            to_sort
        };
        let cd_table = table_drawing::count_down_rows(count_downs).to_string();
        let to_push = format!(
            "Countdowns\n\
            {}\n",
            cd_table
        );
        output.push_str(&to_push);
    }

    output
}

#[cfg(test)]
mod testing {
    use std::collections::HashMap;

    use chrono::{TimeDelta, TimeZone, Utc};
    use corrupted_clock_util::timing::{
        mocking_time::MockTimeImpl, ClockTable, CountDown, Stopwatch, Timer as _,
    };

    use crate::{cli_args::ClockKind, handle_subcommands::draw_tables_of_cds_sws};

    #[test]
    fn draw_tables_for_list_subcommand() {
        fn assert_case(
            case_name: &str,
            clock_kind: Option<ClockKind>,
            input: ClockTable<MockTimeImpl>,
        ) {
            let actual = draw_tables_of_cds_sws(&input, clock_kind);
            insta::assert_snapshot!(case_name, actual);
        }

        assert_case(
            "Drawing table from no stopwatches or count downs",
            None,
            ClockTable::default(),
        );
        let time = MockTimeImpl::new(Utc.with_ymd_and_hms(2024, 5, 1, 8, 20, 40).unwrap());
        let unpaused_sw = (
            "Stopwatch on the first day".to_string(),
            Stopwatch::new_with_impl(time.clone()),
        );
        time.add_to_now(TimeDelta::days(1));
        let mut paused_sw = (
            "Stopwatch on the second day and paused after one day".to_string(),
            Stopwatch::new_with_impl(time.clone()),
        );
        time.add_to_now(TimeDelta::days(1));
        paused_sw.1.pause();
        time.add_to_now(TimeDelta::hours(3));
        let stopwatches: HashMap<String, Stopwatch<MockTimeImpl>> = [unpaused_sw, paused_sw].into();
        let count_downs: HashMap<String, CountDown<MockTimeImpl>> = Default::default();
        let data: ClockTable<MockTimeImpl> = ClockTable::new(stopwatches, count_downs);
        assert_case("Drawing table from stopwatches or count downs", None, data);
    }
}
