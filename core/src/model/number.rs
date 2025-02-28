#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct NumberNode {}

impl NumberNode {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}
