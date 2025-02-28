use crate::model::{AnyNode, NodeType, StringNode};
use maplit::btreeset;

pub fn merge(a: StringNode, b: StringNode) -> NodeType {
    match (&a.format, &b.format) {
        // if one is more specific than the other… just drop the type :/
        (Some(_), None) | (None, Some(_)) => StringNode::default().into(),
        // if they're the same, keep it
        (None, None) => StringNode::default().into(),
        (Some(a), Some(b)) if a == b => StringNode::new(Some(a.to_owned())).into(),
        (Some(_), Some(_)) => AnyNode::new(btreeset![a.into(), b.into()]).into(),
    }
}
