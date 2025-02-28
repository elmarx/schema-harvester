#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StringNode {}

impl StringNode {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}
