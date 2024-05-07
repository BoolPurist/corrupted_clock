use std::path::{Path, PathBuf};

use crate::constants;
use clap::{Parser, Subcommand};

mod clock_kind;
mod clock_reference;
mod column_show_arg;
mod create_command;
mod existing_clock_reference;
mod get_clock_args;
mod list_args;
mod many_clock_reference_kind;

pub use clock_kind::{ClockKind, ClockKindArg};
pub use clock_reference::ClockReference;
pub use column_show_arg::ColumnShowArg;
pub use create_command::CreateCommand;
pub use existing_clock_reference::{ExistingClockKindReference, ExistingClockReference};
pub use get_clock_args::GetClockArgs;
pub use list_args::ListArgs;
pub use many_clock_reference_kind::ManyClockReferenceKind;

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
    /// alias: c
    #[command(alias = "c")]
    Create(CreateCommand),
    /// alias: g
    #[command(alias = "g")]
    Get(GetClockArgs),
    /// alias: l
    #[command(alias = "l")]
    List(ListArgs),
    /// alias: d
    #[command(alias = "d")]
    Delete(ExistingClockReference),
    /// alias: r
    #[command(alias = "r")]
    Resume(ExistingClockReference),
    /// alias: p
    #[command(alias = "p")]
    Pause(ExistingClockReference),
    /// alias: rs
    #[command(alias = "rs")]
    Reset(ExistingClockReference),
}
