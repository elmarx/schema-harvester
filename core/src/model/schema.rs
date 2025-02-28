use crate::model::NodeType;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SchemaHypothesis {
    pub id: String,
    pub title: String,
    pub description: String,
    pub root: Option<NodeType>,
}

impl SchemaHypothesis {
    pub fn new(id: String, title: String, description: String) -> Self {
        SchemaHypothesis {
            id,
            title,
            description,
            root: None,
        }
    }
}
