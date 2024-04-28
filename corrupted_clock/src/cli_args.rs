use std::path::{Path, PathBuf};

use crate::constants;
use clap::{Parser, Subcommand};

mod clock_kind;
mod clock_reference;
mod create_command;
mod delete_args;
mod get_clock_args;
mod list_args;

pub use clock_kind::{ClockKind, ClockKindArg};
pub use clock_reference::ClockReference;
pub use create_command::CreateCommand;
pub use delete_args::DeleteArgs;
pub use get_clock_args::GetClockArgs;
pub use list_args::ListArgs;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct AppCliArgs {
    #[arg(short, long, env = constants::env_var_prefix!("USE_DATA_USER_DIR_FALLBACK"))]
    #[cfg_attr(debug_assertions, arg(default_value_t = false))]
    #[cfg_attr(not(debug_assertions), arg(default_value_t = true))]
    use_data_user_dir: bool,
    #[arg(short, long, env = constants::env_var_prefix!("DATA_DIR"))]
    data_dir: Option<PathBuf>,
    #[command(subcommand)]
    command: AppSubCommands,
}

impl AppCliArgs {
    pub fn use_data_user_dir(&self) -> bool {
        self.use_data_user_dir
    }

    pub fn command(&self) -> &AppSubCommands {
        &self.command
    }

    pub fn data_dir(&self) -> Option<&Path> {
        self.data_dir.as_deref()
    }
}

#[derive(Debug, Subcommand)]
pub enum AppSubCommands {
    Create(CreateCommand),
    Get(GetClockArgs),
    List(ListArgs),
    Delete(DeleteArgs),
    Resume(ClockReference),
    Pause(ClockReference),
    Reset(ClockReference),
}
