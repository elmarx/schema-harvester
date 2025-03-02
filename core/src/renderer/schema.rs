use crate::SchemaHypothesis;
use crate::renderer::Render;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct SerSchema {
    #[serde(rename = "$schema")]
    schema: &'static str,
    #[serde(rename = "$id")]
    id: String,
    title: String,
    description: String,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub root: Option<Value>,
}

impl From<&SchemaHypothesis> for SerSchema {
    fn from(schema: &SchemaHypothesis) -> Self {
        let root = schema.root.as_ref().map(|root| root.render());

        SerSchema {
            schema: "http://json-schema.org/draft-07/schema#",
            id: schema.id.clone(),
            title: schema.title.clone(),
            description: schema.description.clone(),
            root,
        }
    }
}

impl Render for SchemaHypothesis {
    fn render(&self) -> serde_json::Value {
        let schema = SerSchema::from(self);

        serde_json::to_value(schema).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{SchemaHypothesis, renderer::Render};

    #[test]
    fn test_render_schema() {
        let sample = SchemaHypothesis::new(
            "id".to_owned(),
            "title".to_owned(),
            "description".to_owned(),
        );
        let actual = sample.render();
    }
}
