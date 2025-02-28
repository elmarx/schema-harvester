#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct IntegerNode {}

impl IntegerNode {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}
