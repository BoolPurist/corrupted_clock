use std::path::PathBuf;

use corrupted_clock_util::{
    data_store,
    timing::{ClockTable, CountDown, Stopwatch, TimeImpl as _, Timer as _, UtcTimeImpl},
};
use log::warn;

use crate::{
    cli_args::{
        AppCliArgs, ClockKind, ClockReference, CreateCommand, DeleteArgs, GetClockArgs, ListArgs,
    },
    path_utils, table_drawing, AppResult,
};

use self::not_found_clock_err::NotFoundClockErr;

mod not_found_clock_err;

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
            corrupted_clock_util::chrono_time_to_str(now)
        });
    match args.to_count_down() {
        Some(count_down) => app_state.add_count_down(name, CountDown::new(count_down))?,
        None => app_state.add_stopwatch(name, Stopwatch::new())?,
    };
    data_store::save_app_state(&path_to_app_file, &app_state)?;
    Ok(())
}

pub fn resume(general_args: &AppCliArgs, args: &ClockReference) -> AppResult {
    handle_modify_with_save(
        general_args,
        args,
        |app_state, name| app_state.modify_stopwatch(name).unwrap().resume(),
        |app_state, name| app_state.modify_count_down(name).unwrap().resume(),
    )
}

pub fn reset(general_args: &AppCliArgs, args: &ClockReference) -> AppResult {
    handle_modify_with_save(
        general_args,
        args,
        |app_state, name| app_state.modify_stopwatch(name).unwrap().reset(),
        |app_state, name| app_state.modify_count_down(name).unwrap().reset(),
    )
}

pub fn pause(general_args: &AppCliArgs, args: &ClockReference) -> AppResult {
    handle_modify_with_save(
        general_args,
        args,
        |app_state, name| app_state.modify_stopwatch(name).unwrap().pause(),
        |app_state, name| app_state.modify_count_down(name).unwrap().pause(),
    )
}

pub fn delete(general_args: &AppCliArgs, args: &DeleteArgs) -> AppResult {
    handle_modify_with_save(
        general_args,
        args.reference(),
        |app_state, name| _ = app_state.remove_stopwatch(name),
        |app_state, name| _ = app_state.remove_count_down(name),
    )
}

pub fn list(general_args: &AppCliArgs, args: &ListArgs) -> AppResult<String> {
    let LoadedAppStateFile { app_state, .. } = load_app_state(general_args)?;

    let (does_stopwatches, does_count_downs) = match args.kind() {
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
    reference: &ClockReference,
    on_stopwatch: impl FnOnce(&mut ClockTable, &str),
    on_count_down: impl FnOnce(&mut ClockTable, &str),
) -> AppResult {
    let LoadedAppStateFile {
        mut app_state,
        path_to_app_file,
    } = load_app_state(general_args)?;
    let name = reference.name();
    match reference.kind() {
        ClockKind::StopWatch => {
            if app_state.has_stop_watch(name) {
                on_stopwatch(&mut app_state, name);
            } else {
                return Err(NotFoundClockErr::new(name.to_owned(), reference.kind()).into());
            }
        }
        ClockKind::CountDown => {
            if app_state.has_count_down(name) {
                on_count_down(&mut app_state, name);
            } else {
                return Err(NotFoundClockErr::new(name.to_owned(), reference.kind()).into());
            }
        }
    };

    data_store::save_app_state(&path_to_app_file, &app_state)?;
    Ok(())
}
