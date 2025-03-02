use crate::format::{is_valid_date, is_valid_datetime, is_valid_time, is_valid_uuid};
use strum_macros;
use strum_macros::IntoStaticStr;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct StringNode {
    pub format: Option<Format>,
}

/// string format, see https://www.learnjsonschema.com/2020-12/format-annotation/format/
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Format {
    DateTime,
    Date,
    Time,
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
    Uuid,
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

    #[must_use]
    pub fn formatted(format: Format) -> Self {
        Self {
            format: Some(format),
        }
    }
}

impl From<&str> for StringNode {
    fn from(value: &str) -> Self {
        if is_valid_datetime(value) {
            return StringNode::formatted(Format::DateTime);
        }

        if is_valid_date(value) {
            return StringNode::formatted(Format::Date);
        }

        if is_valid_time(value) {
            return StringNode::formatted(Format::Time);
        }

        if is_valid_uuid(value) {
            return StringNode::formatted(Format::Uuid);
        }

        StringNode::default()
    }
}

#[cfg(test)]
mod test {
    use super::Format;
    use super::StringNode;
    use test_case::test_case;

    #[test]
    fn test_string_node() {
        let sample: StringNode = "test".into();
        let expected = StringNode { format: None };
        assert_eq!(sample, expected);
    }

    #[test_case("2000-01-01T00:00:00.000Z", Some(Format::DateTime))]
    #[test_case("2000-13-01T00:00:00.000Z", None)]
    #[test_case("2000-02-30T00:00:00.000Z", None)]
    #[test_case("2000-01-01T25:00:00.000Z", None)]
    #[test_case("abcde", None)]
    #[test_case("2000-01-01", Some(Format::Date))]
    #[test_case("2000-13-01", None)]
    #[test_case("2000-02-30", None)]
    #[test_case("15:33:00Z", Some(Format::Time))]
    fn test_temporal_formats(input: &str, expected: Option<Format>) {
        let sample: StringNode = input.into();
        assert_eq!(sample.format, expected);
    }

    #[test]
    fn test_uuid() {
        let actual: StringNode = "f3fa7e18-549f-4ee1-8aeb-1bb8cbf7e956".into();
        let expected = StringNode::formatted(Format::Uuid);
        assert_eq!(actual, expected)
    }
}
