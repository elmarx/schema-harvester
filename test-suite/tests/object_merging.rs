use schema_harvester::model::NodeType;
use schema_harvester::{SchemaHypothesis, render_schema};
use serde_json::{Value, json, to_string_pretty};

#[must_use]
pub fn generate_hypothesis(dom: &Value) -> SchemaHypothesis {
    SchemaHypothesis {
        id: "https:://github.com/elmarx/schema-harvester".to_string(),
        title: "Sample".to_string(),
        description: "Auto-generated schema".to_string(),
        root: Some(NodeType::from(dom)),
    }
}

#[test]
fn test_distinct_object() {
    let document = json!([
      {
        "name": "BatchManagementRequirement",
        "value": false,
        "inputHint": "SINGLE_LINE",
        "label": {
          "de": "Batch Management Requirement",
          "en": "Batch Management Requirement"
        }
      },
      {
        "name": "Brand",
        "value": "MAGGI",
        "inputHint": "SINGLE_LINE",
        "label": {
          "en": "Brand",
          "de": "Marke (DSD)"
        }
      }
    ]);

    let schema = generate_hypothesis(&document);

    let result = render_schema(&schema);
    let schema_json: Value = serde_json::from_str(&result).unwrap();

    let expected = json!({
        "$id": "https:://github.com/elmarx/schema-harvester",
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Sample",
        "description": "Auto-generated schema",
      "type": "array",
      "items": {
        "properties": {
          "inputHint": {
            "type": "string"
          },
          "label": {
            "properties": {
              "de": {
                "type": "string"
              },
              "en": {
                "type": "string"
              }
            },
            "required": [
              "de",
              "en"
            ],
            "type": "object"
          },
          "name": {
            "type": "string"
          },
          "value": {"anyOf": [{"type": "boolean"}, {"type": "string"}]}
        },
        "required": [
          "inputHint",
          "label",
          "name",
          "value"
        ],
        "type": "object"
      }
    });

    assert_eq!(schema_json, expected);
}

#[test]
fn test_single_object() {
    let document = json!([
      {
        "value": [
        {
          "id": 1
        }, {
          "name": "irgendwas"
        },
        "string",
        true,
        5
      ]
      }
    ]);

    let schema = generate_hypothesis(&document);

    let result = render_schema(&schema);
    let schema_json: Value = serde_json::from_str(&result).unwrap();

    let expected = json!({
        "$id": "https:://github.com/elmarx/schema-harvester",
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Sample",
        "description": "Auto-generated schema",
      "type": "array",
      "items": {
        "properties": {
          "value": {
            "type": "array",
            "items": {
              "anyOf": [
                {"type": "boolean"},
                {"type": "integer"},
                {
                   "type": "object",
                    "properties": {
                      "id": { "type": "integer" },
                      "name": { "type": "string"}
                    },
                    "required": []
                },
                {"type": "string"}
              ]
            }
          }
        },
        "required": [
          "value"
        ],
        "type": "object"
      }
    });

    assert_eq!(
        schema_json,
        expected,
        "{}",
        to_string_pretty(&schema_json).unwrap()
    );
}

#[test]
fn test_single_nested_object() {
    let document = json!([
        {
           "value": "some string"
        },
        {
            "value": 42
        },
        {
            "value": {
                "a": "aaa"
            }
        },
        {
            "value": {
                "b": 111
            }
        }
    ]);

    let schema = generate_hypothesis(&document);

    let result = render_schema(&schema);
    let schema_json: Value = serde_json::from_str(&result).unwrap();

    let expected = json!({
        "$id": "https:://github.com/elmarx/schema-harvester",
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Sample",
        "description": "Auto-generated schema",
          "type": "array",
          "items": {
              "type": "object",
              "properties": {
                  "value": {
                      "anyOf": [
                          {"type": "integer"},
                          {
                             "type": "object",
                              "properties": {
                                "a": { "type": "string" },
                                "b": { "type": "integer"}
                              },
                              "required": []
                          },
                          {"type": "string"}
                      ]
                  }
              },
              "required": ["value"]
          }
    });

    assert_eq!(
        schema_json,
        expected,
        "{}",
        to_string_pretty(&schema_json).unwrap()
    );
}

#[test]
fn test_array_merging() {
    let document = json!([[1], ["1"]]);

    let schema = generate_hypothesis(&document);

    let result = render_schema(&schema);
    let schema_json: Value = serde_json::from_str(&result).unwrap();

    let expected = json!({
        "$id": "https:://github.com/elmarx/schema-harvester",
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Sample",
        "description": "Auto-generated schema",
          "type": "array",
          "items": {
              "type": "array",
              "items": {"anyOf": [
                {"type": "integer"},
                {"type": "string"}
            ]}
          }
    });

    assert_eq!(
        schema_json,
        expected,
        "{}",
        to_string_pretty(&schema_json).unwrap()
    );
}
