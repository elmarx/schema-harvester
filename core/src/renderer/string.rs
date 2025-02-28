use crate::model::StringNode;
use crate::renderer::Render;
use serde_json::json;

impl Render for &StringNode {
    fn render(&self) -> serde_json::Value {
        match &self.format {
            None => json!({
                "type": "string",
            }),
            Some(format) => {
                let format: &str = format.into();
                json!({
                    "type": "string",
                    "format": format
                })
            }
        }
    }
}
