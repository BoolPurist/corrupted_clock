use std::{
    io,
    path::{Path, PathBuf},
};

use thiserror::Error;

use crate::{constants, timing::ClockTable};

#[derive(Debug, Error)]
pub enum LoadError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error(
        "Failed to initialize save file for stop watches and count downs\n\
        Due to the following error\n: {0}"
    )]
    InitFailed(#[from] SaveError),
}

pub fn load_app_state(path: &Path) -> Result<ClockTable, LoadError> {
    ensure_user_dir_and_save_file(path)?;
    let content = std::fs::read_to_string(path)?;
    let content = serde_json::from_str(&content)?;
    Ok(content)
}

fn ensure_user_dir_and_save_file(path: &Path) -> Result<(), SaveError> {
    if let Some(directory) = path.parent() {
        std::fs::create_dir_all(directory)?;
    }
    if matches!(path.try_exists(), Ok(false)) {
        let new_state = ClockTable::default();
        write_app_state(path, &new_state)?;
    }

    Ok(())
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
}

pub fn save_app_state(path: &Path, new_state: &ClockTable) -> Result<(), SaveError> {
    ensure_user_dir_and_save_file(path)?;
    write_app_state(path, new_state)?;
    Ok(())
}

fn write_app_state(path: &Path, new_state: &ClockTable) -> Result<(), SaveError> {
    let to_save = serde_json::to_string_pretty(&new_state)?;
    std::fs::write(path, to_save)?;
    Ok(())
}

pub fn path_to_app_dirs(
    on_get_data_dirs: impl FnOnce() -> Option<PathBuf>,
    on_current_exe: impl FnOnce() -> Result<PathBuf, io::Error>,
) -> Result<PathBuf, io::Error> {
    match on_get_data_dirs() {
        Some(data_dir) => Ok(data_dir.join(constants::APP_NAME)),
        None => on_current_exe().map(|file_path| {
            file_path
                .parent()
                .map(|ref_path| ref_path.to_path_buf())
                .unwrap_or_default()
        }),
    }
}

pub fn path_to_app_state_file(
    on_get_data_dirs: impl FnOnce() -> Option<PathBuf>,
    on_current_exe: impl FnOnce() -> Result<PathBuf, io::Error>,
) -> Result<PathBuf, io::Error> {
    path_to_app_dirs(on_get_data_dirs, on_current_exe)
        .map(|path| path.join(constants::STATE_FILE_NAME))
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_path_to_app_dirs() {
        fn assert_case(
            case_name: &str,
            on_data_dirs: impl FnOnce() -> Option<PathBuf>,
            on_current_exe: impl FnOnce() -> Result<PathBuf, io::Error>,
        ) {
            let actual = path_to_app_dirs(on_data_dirs, on_current_exe);
            insta::assert_debug_snapshot!(case_name, actual);
        }

        const ROOT_DIR: &str = "/home/some_user";
        assert_case(
            "from_user_data_dirs",
            || Some(PathBuf::from(ROOT_DIR)),
            || panic!("Should not fallback to the current exe path !"),
        );
        assert_case(
            "from_executable_path",
            || None,
            || Ok(PathBuf::from(ROOT_DIR).join("Exe.exe")),
        );
        assert_case(
            "no_executable_or_data_dir",
            || None,
            || {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "No location found for saving the app state",
                ))
            },
        );
    }
}
