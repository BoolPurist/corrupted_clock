use std::{
    io,
    path::{Path, PathBuf},
};

use corrupted_clock_util::data_store;
use log::debug;
use thiserror::Error;

use crate::cli_args::AppCliArgs;

#[derive(Debug, Error)]
pub enum NoDataDirErr {
    #[error("Provided directory for the app data must not be a file")]
    IsFile,
    #[error(
        "Could not locate data app directory.\n\
        Underlying error reason: {0}"
    )]
    Io(#[from] io::Error),
}
pub fn get_path_app_state_file(user_dir: &Path) -> PathBuf {
    let path = user_dir.join(corrupted_clock_util::constants::STATE_FILE_NAME);
    debug!("File at ({:?}) .", &path);
    path
}

pub fn get_user_data_dir(args: &AppCliArgs) -> Result<PathBuf, NoDataDirErr> {
    let path = match args.data_dir().map(|path| path.to_path_buf()) {
        Some(path) => {
            if path.is_file() {
                return Err(NoDataDirErr::IsFile);
            }

            Ok::<PathBuf, NoDataDirErr>(path)
        }
        None => {
            let actual = if args.use_data_user_dir() {
                data_store::path_to_app_dirs(|| dirs::data_dir(), || std::env::current_exe())
            } else {
                data_store::path_to_app_dirs(|| None, || std::env::current_exe())
            }?;
            Ok(actual)
        }
    }?;
    debug!("Using {:?} as the user data directory.", &path);
    Ok(path)
}
