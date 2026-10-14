use crate::args::into_reader;
use eros::AnyError;
use schema_harvester::model::NodeType;
use schema_harvester::{SchemaHypothesis, render_schema};
use std::path::PathBuf;

pub fn run(file: Option<PathBuf>) -> eros::Result<(), AnyError> {
    let reader = into_reader(file)?;

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
