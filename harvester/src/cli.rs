use eros::AnyError;
use schema_harvester::model::NodeType;
use schema_harvester::{SchemaHypothesis, render_schema};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn run(file: Option<PathBuf>) -> eros::Result<(), AnyError> {
    let file = file.unwrap_or_else(|| PathBuf::from("-"));

    let reader: Box<dyn BufRead> = if file == Path::new("-") {
        Box::new(io::stdin().lock())
    } else {
        let file = File::open(file)?;
        Box::new(BufReader::new(file))
    };

    let deserializer = serde_json::Deserializer::from_reader(reader);
    let iterator = deserializer.into_iter::<serde_json::Value>();

    let mut current_hypothesis = SchemaHypothesis::new(
        // TODO: pass these hardcoded strings via CLI flag
        "https://github.com/elmarx/schema-harvester".to_string(),
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
