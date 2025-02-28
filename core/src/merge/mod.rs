use crate::merge::array::merge_array;
use crate::merge::object::merge_object;
use crate::model::{AnyNode, NodeType, SchemaHypothesis};
use maplit::btreeset;

mod any;
mod array;
mod object;
mod object_property;
mod string;

impl SchemaHypothesis {
    #[must_use]
    pub fn merge(self, other_root: NodeType) -> SchemaHypothesis {
        let root = if let Some(root) = self.root {
            merge_node_type(root, other_root)
        } else {
            other_root
        };

        SchemaHypothesis {
            id: self.id,
            title: self.title,
            description: self.description,
            root: Some(root),
        }
    }
}

pub fn merge_node_type(a: NodeType, b: NodeType) -> NodeType {
    match (a, b) {
        (a, b) if a == b => a,
        (NodeType::String(a), NodeType::String(b)) => string::merge(a, b),
        (NodeType::Object(a), NodeType::Object(b)) => merge_object(a, b).into(),
        (NodeType::Array(a), NodeType::Array(b)) => merge_array(a, b).into(),
        (NodeType::Any(xs), NodeType::Any(ys)) => any::merge_any(&xs, ys),
        (a @ NodeType::Any(_), b) | (b, a @ NodeType::Any(_)) => {
            merge_node_type(a, AnyNode::new(btreeset![b]).into())
        }
        (a, b) => merge_node_type(
            AnyNode::new(btreeset![a]).into(),
            AnyNode::new(btreeset![b]).into(),
        ),
    }
}

#[cfg(test)]
mod test {
    use maplit::{btreemap, btreeset};

    use crate::merge::merge_node_type;
    use crate::model::{
        AnyNode, ArrayNode, IntegerNode, NodeType, ObjectNode, ObjectProperty, StringFormat,
        StringNode,
    };

    #[test]
    fn test_merge_string() {
        let a = StringNode::default();
        let b = StringNode::default();

        let actual = merge_node_type(a.into(), b.into());

        assert_eq!(actual, StringNode::default().into());
    }

    #[test]
    fn test_merge_string_with_same_format() {
        let a = StringNode::new(Some(StringFormat::DateTime));
        let b = StringNode::new(Some(StringFormat::DateTime));

        let actual = merge_node_type(a.into(), b.into());

        assert_eq!(actual, StringNode::new(Some(StringFormat::DateTime)).into());
    }

    #[test]
    fn test_merge_string_with_different_format() {
        let a = StringNode::new(Some(StringFormat::DateTime));
        let b = StringNode::new(Some(StringFormat::Time));

        let actual = merge_node_type(a.into(), b.into());

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                StringNode::new(Some(StringFormat::DateTime)).into(),
                StringNode::new(Some(StringFormat::Time)).into()
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_string_with_format_and_no_format() {
        let a = StringNode::new(Some(StringFormat::DateTime));
        let b = StringNode::new(None);

        let actual = merge_node_type(a.into(), b.into());

        assert_eq!(actual, StringNode::default().into());
    }

    #[test]
    fn test_merge_array_without_types() {
        let a = ArrayNode::new_untyped();
        let b = ArrayNode::new_untyped();

        assert_eq!(
            merge_node_type(a.into(), b.into()),
            ArrayNode::new_untyped().into()
        );
    }

    #[test]
    fn test_merge_array_with_same_types() {
        let a = ArrayNode::new_many(btreeset!(IntegerNode::new().into()));
        let b = ArrayNode::new_many(btreeset!(IntegerNode::new().into()));

        assert_eq!(
            merge_node_type(a.into(), b.into()),
            ArrayNode::new_many(btreeset!(IntegerNode::new().into())).into()
        );
    }

    #[test]
    fn test_merge_array_with_one_empty_one_given() {
        let a = ArrayNode::new_untyped();
        let b = ArrayNode::new_many(btreeset!(IntegerNode::new().into()));

        assert_eq!(
            merge_node_type(a.into(), b.into()),
            ArrayNode::new_many(btreeset!(IntegerNode::new().into())).into()
        );
    }

    #[test]
    fn test_merge_array_with_different_types() {
        let a = ArrayNode::new_many(btreeset![
            IntegerNode::new().into(),
            StringNode::default().into()
        ])
        .into();
        let b = ArrayNode::new_many(btreeset![IntegerNode::new().into(), NodeType::Boolean]).into();

        assert_eq!(
            merge_node_type(a, b),
            ArrayNode::new_many(btreeset![
                IntegerNode::new().into(),
                StringNode::default().into(),
                NodeType::Boolean
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_array_with_objects() {
        let a = ArrayNode::new_many(btreeset![
            ObjectNode::new(btreemap! {
                "id".to_string() => ObjectProperty {
                    node_type: IntegerNode::new().into(),
                    required: true
                }
            })
            .into()
        ]);
        let b = ArrayNode::new_many(btreeset![
            ObjectNode::new(btreemap! {
                "name".to_string() => ObjectProperty {
                    node_type: StringNode::default().into(),
                    required: true
                }
            })
            .into()
        ]);

        assert_eq!(
            merge_node_type(a.into(), b.into()),
            ArrayNode::new_many(btreeset![
                ObjectNode::new(btreemap! {
                    "id".to_string() => ObjectProperty {
                        node_type: IntegerNode::new().into(),
                        required: false
                    },
                    "name".to_string() => ObjectProperty {
                        node_type: StringNode::default().into(),
                        required: false
                    }
                })
                .into()
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_object_additional_property_b() {
        let a = ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::default())
        });

        let b = ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::default()),
            String::from("name") => ObjectProperty::new(StringNode::default())
        });

        let actual = merge_node_type(a.into(), b.into());

        let expected = ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::default()),
            String::from("name") => ObjectProperty::new(StringNode::default()).optional()
        })
        .into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge_object_property_missing_in_b() {
        let a = ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::default()),
            String::from("name") => ObjectProperty::new(StringNode::default())
        });

        let b = ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::default()),
        });

        let actual = merge_node_type(a.into(), b.into());
        let expected = ObjectNode::new(btreemap! {
            String::from("id") => ObjectProperty::new(StringNode::default()),
            String::from("name") => ObjectProperty::new(StringNode::default()).optional()
        })
        .into();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_merge_different_types() {
        let a = StringNode::default().into();
        let b = IntegerNode::new().into();

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                StringNode::default().into(),
                IntegerNode::new().into()
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_any_and_type() {
        let a = AnyNode::new(btreeset![IntegerNode::new().into()]).into();
        let b = StringNode::default().into();

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                IntegerNode::new().into(),
                StringNode::default().into()
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_type_and_any() {
        let a = StringNode::default().into();
        let b = AnyNode::new(btreeset![IntegerNode::new().into()]).into();

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                IntegerNode::new().into(),
                StringNode::default().into()
            ])
            .into()
        );
    }

    #[test]
    fn test_merge_existing_type_and_any() {
        let a = AnyNode::new(btreeset![
            StringNode::default().into(),
            IntegerNode::new().into()
        ])
        .into();
        let b = StringNode::default().into();

        let actual = merge_node_type(a, b);

        assert_eq!(
            actual,
            AnyNode::new(btreeset![
                StringNode::default().into(),
                IntegerNode::new().into()
            ])
            .into()
        );
    }
}
