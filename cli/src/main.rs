use clap::Parser;
use schema_harvester::model::NodeType;
use schema_harvester::{SchemaHypothesis, render_schema};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let reader: Box<dyn Read> = get_reader(args.file);

    let deserializer = serde_json::Deserializer::from_reader(reader);
    let iterator = deserializer.into_iter::<serde_json::Value>();

    let mut current_hypothesis = SchemaHypothesis::new(
        "https:://github.com/elmarx/schema-harvester".to_string(),
        "Sample".to_string(),
        "Auto-generated schema".to_string(),
    );

    for json_document in iterator {
        let new_hypo: NodeType = (&json_document?).into();
        current_hypothesis = current_hypothesis.merge(new_hypo);
    }

    let result = render_schema(&current_hypothesis);

    println!("{result}");

    Ok(())
}

fn get_reader(path: Option<String>) -> Box<dyn Read> {
    if let Some(file_path) = path {
        // Read from a file if the `--file` option is provided.
        let file = File::open(file_path).expect("Failed to open file");
        Box::new(file)
    } else {
        // Read from standard input if the `--file` option is not provided.
        Box::new(io::stdin())
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    /// JSON file path
    file: Option<String>,
}
