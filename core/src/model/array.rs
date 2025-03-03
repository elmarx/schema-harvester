use crate::model::any::AnyNode;
use crate::model::node_type::NodeType;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ArrayNode {
    pub items: Option<Box<NodeType>>,
}

impl ArrayNode {
    #[must_use]
    pub fn new(node_type: NodeType) -> Self {
        Self {
            items: Some(Box::new(node_type)),
        }
    }
}

impl From<BTreeSet<NodeType>> for ArrayNode {
    fn from(mut node_types: BTreeSet<NodeType>) -> Self {
        match node_types.len() {
            0 => Self::default(),
            1 => Self {
                items: node_types.pop_first().map(Box::new),
            },
            _ => Self {
                items: Some(Box::new(NodeType::Any(AnyNode::new(node_types)))),
            },
        }
    }
}
