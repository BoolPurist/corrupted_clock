pub mod cli_args;

pub mod constants;
pub mod handle_subcommands;
pub mod path_utils;
pub mod table_drawing;
pub type AppError = Box<dyn std::error::Error>;
pub type AppResult<T = ()> = Result<T, AppError>;
