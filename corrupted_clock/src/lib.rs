pub use non_zero::{AtLeastOne, IsZeroError, ParseNonPosZeroError};

pub mod cli_args;
pub mod constants;
pub mod handle_subcommands;
pub mod path_utils;
pub mod table_drawing;

pub type AppError = Box<dyn std::error::Error>;
pub type AppResult<T = ()> = Result<T, AppError>;

mod listing_items_param;
mod non_zero;
