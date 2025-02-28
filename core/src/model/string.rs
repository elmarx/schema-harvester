#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StringNode {
    format: Option<Format>,
}

/// string format, see https://www.learnjsonschema.com/2020-12/format-annotation/format/
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Format {
    DateTime,
    Date,
    // Time,
    // Duration,
    // Email,
    // IdnEmail,
    // Hostname,
    // IdnHostname,
    // Ipv4,
    // Ipv6,
    // Uri,
    // UriReference,
    // Iri,
    // IriReference,
    // Uuid,
    // UriTemplate,
    // JsonPointer,
    // RelativeJsonPointer,
    // Regex,
}

impl StringNode {
    #[must_use]
    pub fn new(format: Option<Format>) -> Self {
        Self { format }
    }
}

impl From<&str> for StringNode {
    fn from(_value: &str) -> Self {
        // TODO: implement detection of different formats
        StringNode::new(None)
    }
}

#[cfg(test)]
mod test {
    use super::StringNode;

    #[test]
    fn test_string_node() {
        let sample: StringNode = "test".into();
        let expected = StringNode { format: None };
        assert_eq!(sample, expected);
    }
}
