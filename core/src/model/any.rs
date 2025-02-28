use std::collections::BTreeSet;

use crate::model::node_type::NodeType;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AnyNode {
    pub nodes: BTreeSet<NodeType>,
}

impl AnyNode {
    #[must_use]
    pub fn new(nodes: BTreeSet<NodeType>) -> Self {
        Self { nodes }
    }
}
