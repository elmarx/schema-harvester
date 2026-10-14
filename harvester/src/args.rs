use std::path::PathBuf;

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
    // validate the given file is a valid json-schema
    MetaValidate {
        file: Option<PathBuf>,
    },
}
