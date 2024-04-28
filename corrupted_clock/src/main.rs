use std::process::ExitCode;

use clap::Parser;
use corrupted_clock::{
    cli_args::{AppCliArgs, AppSubCommands},
    handle_subcommands, AppResult,
};
use log::error;

fn main() -> ExitCode {
    env_logger::init();
    let args = AppCliArgs::parse();
    match subcommand(args) {
        Ok(opt_text) => {
            if let Some(text) = opt_text {
                println!("{}", text);
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            error!("{}", error);
            ExitCode::FAILURE
        }
    }
}

fn subcommand(args: AppCliArgs) -> AppResult<Option<String>> {
    match args.command() {
        AppSubCommands::Create(command_args) => {
            handle_subcommands::create(&args, &command_args)?;
            Ok(None)
        }
        AppSubCommands::Delete(delete_args) => {
            handle_subcommands::delete(&args, delete_args).map(|_| None)
        }
        AppSubCommands::Resume(clock_ref) => {
            handle_subcommands::resume(&args, &clock_ref).map(|_| None)
        }
        AppSubCommands::Pause(clock_ref) => {
            handle_subcommands::pause(&args, &clock_ref).map(|_| None)
        }
        AppSubCommands::Reset(clock_ref) => {
            handle_subcommands::reset(&args, &clock_ref).map(|_| None)
        }
        AppSubCommands::List(list_args) => handle_subcommands::list(&args, list_args).map(Some),
        AppSubCommands::Get(get_args) => handle_subcommands::get_clock(&args, &get_args).map(Some),
    }
}
