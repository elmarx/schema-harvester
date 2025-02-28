use crate::model::{AnyNode, NodeType};
use crate::renderer::Render;
use serde_json::{Map, Value};
use std::collections::BTreeSet;

fn render_any_map(node_types: &BTreeSet<NodeType>) -> Map<String, Value> {
    let mut map = Map::new();
    map.insert(
        "anyOf".to_string(),
        node_types.iter().map(Render::render).collect(),
    );

    map
}

impl Render for AnyNode {
    fn render(&self) -> Value {
        Value::Object(render_any_map(&self.nodes))
    }
}

#[cfg(test)]
mod test {
    use maplit::{btreemap, btreeset};
    use serde_json::json;

    use crate::model::{AnyNode, IntegerNode, NodeType, ObjectNode, ObjectProperty, StringNode};
    use crate::renderer::Render;

    #[test]
    fn test_any() {
        let node_type: NodeType =
            AnyNode::new(btreeset![StringNode::default().into(), NodeType::Boolean]).into();

        let actual = node_type.render();

        assert_eq!(
            actual,
            json!({
                "anyOf": [
                    {"type": "boolean"},
                    {"type": "string"},
                ]
            })
        );
    }

    #[test]
    fn test_any_one() {
        let node_type: NodeType = AnyNode::new(btreeset![StringNode::default().into()]).into();

        let actual = node_type.render();

        assert_eq!(
            actual,
            json!({
                "anyOf": [
                    {"type": "string"}
                ]
            })
        );
    }

    #[test]
    fn test_any_empty() {
        let node_type: NodeType = AnyNode::new(btreeset![]).into();

        let actual = node_type.render();

        assert_eq!(
            actual,
            json!({
                "anyOf": []
            })
        );
    }

    #[test]
    fn test_any_complex_types() {
        let node_type: NodeType = AnyNode::new(btreeset![
            ObjectNode::new(btreemap! {
                "id".to_string() => ObjectProperty::new(IntegerNode::new())
            })
            .into()
        ])
        .into();

        let actual = node_type.render();

        assert_eq!(
            actual,
            json!({
                "anyOf": [
                    {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "integer"
                            }
                        },
                        "required": ["id"]
                    }
                ]
            })
        );
    }
}
