//! various hints for the schema-generation

/// use (multiple) sub-schemas for a given property
enum Reference {
    /// a schema tagged internally with a discriminator, e.g.
    /// ```json
    /// { "type": "car", "color": "red" }
    /// ```
    InternallyTagged {
        /// a property defining the sub-schema to use, should be a
        /// [JSON pointer](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html#method.pointer)
        discriminator: String,
    },
    /// a schema tagged with a discriminator in an envelope, e.g.:
    /// ```json
    /// { "type": "car", "data": { "color": "red" } }
    /// ```
    ///
    /// this is the format for [CloudEvents](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/spec.md#example)
    ExternallyTagged {
        /// a property defining the sub-schema to use, should be a
        /// [JSON pointer](https://docs.rs/serde_json/latest/serde_json/value/enum.Value.html#method.pointer)
        discriminator: String,
        /// property where the data of the sub-schema live. Must be a JSON-pointer
        data: String,
    },
}
