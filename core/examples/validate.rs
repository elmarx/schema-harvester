use std::fs::File;
use std::io;
use std::io::Read;
use std::process::ExitCode;

/// meta-validation of a JSON-schema
///
/// Basically https://docs.rs/jsonschema/0.29.0/jsonschema/index.html#meta-schema-validation in executable
fn main() -> ExitCode {
    let file_path = std::env::args().nth(1).unwrap_or("-".to_string());

    let file_reader: Box<dyn Read> = if file_path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(file_path).expect("Failed to open file"))
    };
    let schema: serde_json::Value = serde_json::from_reader(file_reader).unwrap();

    let validation = jsonschema::meta::validate(&schema);

    if let Err(validation_error) = validation {
        println!("{:#?}", validation_error);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
