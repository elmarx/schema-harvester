use crate::args::Command;
use clap::Parser;
use eros::AnyError;
use std::process::ExitCode;

mod args;
mod cli;
mod daemon;
mod validate;

fn main() -> eros::Result<ExitCode, AnyError> {
    let args = args::Cli::parse();

    match args.command {
        Command::Daemon => daemon::run(),
        Command::Analyse { file } => cli::run(file)?,
        Command::MetaValidate { file } => return validate::meta(file),
    }
    Ok(ExitCode::SUCCESS)
}
