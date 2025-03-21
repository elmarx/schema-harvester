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

#[cfg(test)]
mod test {
    use serde_json::json;

    use crate::model::{NodeType, StringFormat, StringNode};
    use crate::renderer::Render;

    #[test]
    fn render_string_without_type() {
        let node: NodeType = StringNode::default().into();

        let actual = node.render();

        assert_eq!(actual, json!({ "type": "string" }));
    }

    #[test]
    fn render_string_with_type() {
        let node: NodeType = StringNode::new(Some(StringFormat::DateTime)).into();

        let actual = node.render();

        assert_eq!(actual, json!({ "type": "string", "format": "date-time" }));
    }
}
