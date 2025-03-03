use crate::model::ArrayNode;
use crate::renderer::Render;
use serde_json::{Map, Value};

fn render_array_map(node_type: &ArrayNode) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert("type".to_string(), Value::String("array".to_string()));
    node_type
        .items
        .as_ref()
        .map(|node_type| map.insert("items".to_string(), node_type.render()));
    map
}

impl Render for ArrayNode {
    fn render(&self) -> Value {
        Value::Object(render_array_map(self))
    }
}

#[cfg(test)]
mod test {
    use maplit::btreeset;
    use serde_json::json;

    use crate::model::{ArrayNode, IntegerNode, NodeType, StringNode};
    use crate::renderer::Render;

    #[test]
    fn test_array() {
        let hypothesis: NodeType = ArrayNode::from(btreeset![
            StringNode::default().into(),
            IntegerNode::new().into()
        ])
        .into();

        let actual = hypothesis.render();

        assert_eq!(
            actual,
            json!(
                {
                    "type": "array",
                    "items": {
                        "anyOf": [
                            {
                                "type": "integer"
                            },
                            {
                                "type": "string"
                            }
                        ]
                    }
                }
            )
        );
    }

    #[test]
    fn test_array_single_type() {
        let hypothesis: NodeType = ArrayNode::new(StringNode::default().into()).into();

        let actual = hypothesis.render();

        assert_eq!(
            actual,
            json!(
                {
                    "type": "array",
                    "items": {
                        "type": "string"
                    }
                }
            )
        );
    }

    #[test]
    fn test_empty_array() {
        let hypothesis: NodeType = ArrayNode::default().into();

        let actual = hypothesis.render();

        assert_eq!(actual, json!({ "type": "array" }));
    }
}
