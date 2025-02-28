use crate::model::NodeType;
use crate::renderer::Render;
use serde_json::{Value, json};

impl Render for NodeType {
    fn render(&self) -> Value {
        match self {
            NodeType::String(s) => s.render(),
            NodeType::Integer(_) => json!({"type": "integer"}),
            NodeType::Number(_) => json!({"type": "number"}),
            NodeType::Boolean => json!({"type": "boolean"}),
            NodeType::Null => json!({"type": "null"}),
            NodeType::Array(a) => a.render(),
            NodeType::Object(o) => o.render(),
            NodeType::Any(a) => a.render(),
        }
    }
}
