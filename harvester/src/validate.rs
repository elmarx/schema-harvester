use crate::args::into_reader;
use eros::AnyError;
use std::path::PathBuf;
use std::process::ExitCode;

pub fn meta(file: Option<PathBuf>) -> eros::Result<ExitCode, AnyError> {
    let reader = into_reader(file)?;

    let schema: serde_json::Value = serde_json::from_reader(reader)?;

    let validation = jsonschema::meta::validate(&schema);

    if let Err(validation_error) = validation {
        println!("{validation_error:#?}");
        Ok(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
