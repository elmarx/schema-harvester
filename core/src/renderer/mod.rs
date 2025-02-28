use crate::SchemaHypothesis;
use serde_json::Value;

mod any;
mod array;
mod node;
mod object;
mod string;

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn render_schema(schema: &SchemaHypothesis) -> String {
    serde_json::to_string_pretty(&render_json_schema(schema)).unwrap()
}

fn render_json_schema(schema: &SchemaHypothesis) -> Value {
    schema.root.render()
}

trait Render {
    fn render(&self) -> serde_json::Value;
}
