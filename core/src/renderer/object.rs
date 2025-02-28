use crate::model::{ObjectNode, ObjectProperty};
use crate::renderer::Render;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

fn render_object_map(properties: &BTreeMap<String, ObjectProperty>) -> Map<String, Value> {
    let required_props: Vec<Value> = properties
        .iter()
        .filter_map(|(key, value)| {
            if value.required {
                Some(Value::String(key.to_string()))
            } else {
                None
            }
        })
        .collect();

    let object_properties: Map<String, Value> = properties
        .iter()
        .map(|(key, value)| {
            let node_type = &value.node_type;
            (key.to_string(), node_type.render())
        })
        .collect();

    let mut map = Map::new();

    map.insert("type".to_string(), Value::String("object".to_string()));
    map.insert("required".to_string(), Value::Array(required_props));
    map.insert("properties".to_string(), Value::Object(object_properties));

    map
}

impl Render for ObjectNode {
    fn render(&self) -> Value {
        Value::Object(render_object_map(&self.properties))
    }
}

#[cfg(test)]
mod test {
    use maplit::btreemap;
    use serde_json::json;

    use crate::model::{NodeType, ObjectNode, ObjectProperty, StringNode};
    use crate::renderer::Render;

    #[test]
    fn test_object() {
        let hypothesis: NodeType = ObjectNode::new(btreemap! {
            "name".to_string() => ObjectProperty::new(StringNode::default()),
        })
        .into();

        let actual = hypothesis.render();

        assert_eq!(
            actual,
            json!(
                {
                    "type": "object",
                    "required": ["name"],
                    "properties": {
                        "name": {
                            "type": "string"
                        }
                    }
                }
            )
        );
    }
}
