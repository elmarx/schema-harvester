#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct DateNode {}

impl DateNode {
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}
