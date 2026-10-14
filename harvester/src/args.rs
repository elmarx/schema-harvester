use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// run a service to continuously analyze data
    Daemon,
    /// analyze a single file, print the resulting schema to stdout
    Analyse {
        /// file to read (or - for stdin). if not provided, stdin is used
        file: Option<PathBuf>,
    },
    /// validate a (generated) json-schema against a (meta-) schema
    MetaValidate {
        file: Option<PathBuf>,
    },
}

pub fn into_reader(file: Option<PathBuf>) -> io::Result<Box<dyn BufRead>> {
    let file = file.unwrap_or_else(|| PathBuf::from("-"));

    let reader: Box<dyn BufRead> = if file == Path::new("-") {
        Box::new(io::stdin().lock())
    } else {
        let file = File::open(file)?;
        Box::new(BufReader::new(file))
    };

    Ok(reader)
}
