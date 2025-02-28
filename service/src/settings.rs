use std::collections::HashMap;

use crate::log;
use crate::utils::VecExt;
use config::{ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../config.default.toml");

type PropertyConfig = (String, String);

/// type to accept all values allowed by rdkafka.
/// rdkafka expects all properties as Into<String>, this enables to write numbers into toml without quotes
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum KafkaPropertyValue {
    String(String),
    Bool(bool),
    Integer(i64),
}

impl From<&KafkaPropertyValue> for String {
    fn from(v: &KafkaPropertyValue) -> Self {
        match v {
            KafkaPropertyValue::String(s) => s.clone(),
            KafkaPropertyValue::Bool(b) => b.to_string(),
            KafkaPropertyValue::Integer(i) => i.to_string(),
        }
    }
}

impl From<KafkaPropertyValue> for String {
    fn from(v: KafkaPropertyValue) -> Self {
        match v {
            KafkaPropertyValue::String(s) => s,
            KafkaPropertyValue::Bool(b) => b.to_string(),
            KafkaPropertyValue::Integer(i) => i.to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Kafka {
    #[serde(flatten, default)]
    pub properties: HashMap<String, KafkaPropertyValue>,
}

#[derive(Debug, Deserialize)]
pub struct KafkaSource {
    #[serde(flatten, default)]
    pub properties: HashMap<String, KafkaPropertyValue>,
}

#[derive(Debug, Deserialize)]
pub struct KafkaSink {
    /// topic where to publish updated schemas to
    pub topic: String,

    #[serde(flatten, default)]
    pub properties: HashMap<String, KafkaPropertyValue>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub kafka: Kafka,
    pub kafka_source: KafkaSource,
    pub kafka_sink: KafkaSink,

    pub logging: log::Format,
    pub management_port: u16,
}

#[derive(Debug)]
pub struct Setting {
    pub config: Config,
    pub kafka_sink_properties: Vec<PropertyConfig>,
    pub kafka_source_properties: Vec<PropertyConfig>,
}

impl Setting {
    pub(crate) fn emerge() -> Result<Setting, ConfigError> {
        let config_file = std::env::var("HARVESTER_CONFIG").unwrap_or("config.toml".to_string());

        let settings = config::Config::builder()
            .add_source(File::from_str(DEFAULT_CONFIG, FileFormat::Toml))
            .add_source(config::File::with_name(&config_file))
            .add_source(Environment::with_prefix("HARVESTER").separator("_"))
            .build();

        let (kafka_source_properties, kafka_sink_properties) =
            kafka_sink_source_from_env(std::env::vars());

        settings?.try_deserialize::<Config>().map(|config| Setting {
            config,
            kafka_sink_properties,
            kafka_source_properties,
        })
    }
}

fn rdkafka_prop(s: &str) -> String {
    s.replace('_', ".").to_lowercase()
}

/// read sink- and source-properties from environment, respecting common defaults
fn kafka_sink_source_from_env(
    env_vars: impl Iterator<Item = (String, String)>,
) -> (Vec<PropertyConfig>, Vec<PropertyConfig>) {
    let mut kafka_source_properties = vec![];
    let mut kafka_sink_properties = vec![];

    for (k, value) in env_vars
        .into_iter()
        .filter(|(k, _)| k.starts_with("KAFKA_"))
    {
        match (
            k.strip_prefix("KAFKA_SOURCE_"),
            k.strip_prefix("KAFKA_SINK_"),
            k.strip_prefix("KAFKA_"),
        ) {
            (Some(source_prop), _, _) => {
                kafka_source_properties.insert_if_absent(rdkafka_prop(source_prop), value);
            }
            (_, Some(sink_prop), _) => {
                kafka_sink_properties.insert_if_absent(rdkafka_prop(sink_prop), value);
            }
            (_, _, Some(default_prop)) => {
                let prop = rdkafka_prop(default_prop);
                kafka_sink_properties.upsert(prop.clone(), value.clone());
                kafka_source_properties.upsert(prop, value.clone())
            }
            _ => {}
        }
    }

    (kafka_source_properties, kafka_sink_properties)
}

#[cfg(test)]
mod test {
    use super::kafka_sink_source_from_env;

    #[test]
    fn test_kafka_sink_source_merges_into_defaults() {
        let env_vars = vec![
            ("XYZ".to_string(), "short".to_string()),
            (
                "KAFKA_BOOTSTRAP_SERVERS".to_string(),
                "localhost:9092".to_string(),
            ),
            (
                "KAFKA_SOURCE_SASL_USERNAME".to_string(),
                "read-only-harvester".to_string(),
            ),
            (
                "KAFKA_SINK_SASL_USERNAME".to_string(),
                "read-write-harvester".to_string(),
            ),
            (
                "KAFKA_SSL_CA_LOCATION".to_string(),
                "/var/run/secrets/ca.pem".to_string(),
            ),
        ];

        let (actual_source, actual_sink) = kafka_sink_source_from_env(env_vars.into_iter());
        let expected_source: Vec<_> = vec![
            ("bootstrap.servers", "localhost:9092"),
            ("sasl.username", "read-only-harvester"),
            ("ssl.ca.location", "/var/run/secrets/ca.pem"),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect();

        let expected_sink: Vec<_> = vec![
            ("bootstrap.servers", "localhost:9092"),
            ("sasl.username", "read-write-harvester"),
            ("ssl.ca.location", "/var/run/secrets/ca.pem"),
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect();

        assert_eq!(actual_source, expected_source);
        assert_eq!(actual_sink, expected_sink);
    }
}
