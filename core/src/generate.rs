use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

use crate::model::{
    AnyNode, ArrayNode, IntegerNode, NodeType, NumberNode, ObjectNode, ObjectProperty, StringNode,
};
use crate::utils::SetVariances;

impl From<&Value> for NodeType {
    fn from(dom: &Value) -> Self {
        match dom {
            Value::Null => NodeType::Null,
            Value::Bool(_) => NodeType::Boolean,
            Value::Number(i) if i.is_f64() => NumberNode::new().into(),
            Value::Number(_) => IntegerNode::new().into(),
            Value::String(s) => StringNode::from(s.as_str()).into(),
            Value::Array(array_values) => {
                if array_values.is_empty() {
                    ArrayNode::default().into()
                } else {
                    ArrayNode::new(generate_node_type_for_array_values(array_values)).into()
                }
            }
            Value::Object(props) => ObjectNode::new(generate_properties(props)).into(),
        }
    }
}

fn generate_properties(properties: &Map<String, Value>) -> BTreeMap<String, ObjectProperty> {
    properties
        .iter()
        .map(|(key, value)| {
            (
                key.clone(),
                ObjectProperty {
                    required: true,
                    node_type: NodeType::from(value),
                },
            )
        })
        .collect()
}

fn generate_node_type_for_array_values(array_values: &[Value]) -> NodeType {
    let mut merged_obj_type: Option<NodeType> = None;
    let mut merged_array_type: Option<NodeType> = None;
    let mut types = BTreeSet::new();

    for value in array_values {
        let value_type = NodeType::from(value);
        match value_type {
            NodeType::Object(ObjectNode { properties: _ }) => {
                merged_obj_type = match merged_obj_type {
                    Some(acc) => Some(crate::merge::merge_node_type(acc, value_type)),
                    None => Some(value_type),
                };
            }
            NodeType::Array(_) => {
                merged_array_type = match merged_array_type {
                    Some(acc) => Some(crate::merge::merge_node_type(acc, value_type)),
                    None => Some(value_type),
                }
            }
            _ => {
                types.insert(value_type);
            }
        };
    }
    if let Some(node_type) = merged_obj_type {
        types.insert(node_type);
    }

    if let Some(node_type) = merged_array_type {
        types.insert(node_type);
    }

    match SetVariances::new(&types) {
        SetVariances::Empty => unreachable!(),
        SetVariances::OneElement(node_type) => node_type.clone(),
        SetVariances::Multiple(_) => AnyNode::new(types).into(),
    }
}

#[cfg(test)]
mod test {
    use crate::model::{
        AnyNode, ArrayNode, IntegerNode, NodeType, NumberNode, ObjectNode, ObjectProperty,
        StringNode,
    };
    use maplit::{btreemap, btreeset};
    use serde_json::json;

    #[test]
    fn test_null() {
        let dom = json!(null);
        assert_eq!(NodeType::from(&dom), NodeType::Null);
    }

    #[test]
    fn test_bool() {
        let dom = json!(true);
        assert_eq!(NodeType::from(&dom), NodeType::Boolean);
    }

    #[test]
    fn test_integer() {
        let dom = json!(10);
        assert_eq!(NodeType::from(&dom), IntegerNode::new().into());
    }

    #[test]
    fn test_number() {
        let dom = json!(10.5);
        assert_eq!(NodeType::from(&dom), NumberNode::new().into());
    }

    #[test]
    fn test_string() {
        let dom = json!("Schema-harvester");
        assert_eq!(NodeType::from(&dom), StringNode::default().into());
    }

    #[test]
    fn test_array_merge_objects() {
        let dom = json!(["one", 1, {"a": 1}, {"a": "1"}]);
        let actual = NodeType::from(&dom);
        let expected = ArrayNode::from(btreeset! {
            StringNode::default().into(),
            IntegerNode::new().into(),
            ObjectNode::new(btreemap! {
                    "a".to_string() => ObjectProperty { required: true, node_type: AnyNode::new(
                        btreeset! { StringNode::default().into(), IntegerNode::new().into() }
                    ).into()}
                }).into()
        })
        .into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_array_all_int() {
        let dom = json!([10, 15, 25]);
        assert_eq!(
            NodeType::from(&dom),
            ArrayNode::new(IntegerNode::new().into()).into()
        );
    }

    #[test]
    fn test_array_empty() {
        let dom = json!([]);
        assert_eq!(NodeType::from(&dom), ArrayNode::default().into());
    }

    #[test]
    fn test_array_int_and_string() {
        let dom = json!([42, "Hello"]);

        assert_eq!(
            NodeType::from(&dom),
            ArrayNode::from(btreeset![
                IntegerNode::new().into(),
                StringNode::default().into()
            ])
            .into()
        );
    }

    #[test]
    fn test_object() {
        let dom = json!({
            "name": "Schokoladenbrunnen",
            "length": 100
        });
        let expected = ObjectNode::new(btreemap! {
            "name".to_string() => ObjectProperty::new(StringNode::default()),
            "length".to_string() => ObjectProperty::new(IntegerNode::new()),
        })
        .into();

        assert_eq!(NodeType::from(&dom), expected);
    }
}
