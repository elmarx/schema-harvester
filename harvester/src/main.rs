use crate::args::Command;
use clap::Parser;
use eros::AnyError;

mod args;
mod cli;
mod daemon;
mod validate;

fn main() -> eros::Result<(), AnyError> {
    let args = args::Cli::parse();

    match args.command {
        Command::Daemon => daemon::run(),
        Command::Analyse { file } => cli::run(file)?,
        Command::MetaValidate { file } => validate::meta(file),
    }

    Ok(())
}
