use crate::settings::{Kafka, KafkaSink, KafkaSource};
use anyhow::Context;
use rdkafka::ClientConfig;

pub fn init_source(
    default_config: &Kafka,
    source_config: &KafkaSource,
    source_properties: Vec<(String, String)>,
) -> anyhow::Result<rdkafka::consumer::StreamConsumer> {
    let mut cfg = ClientConfig::new();
    cfg.extend(
        default_config
            .properties
            .iter()
            .map(|(k, v)| (k.clone(), v.into())),
    );
    cfg.extend(
        source_config
            .properties
            .iter()
            .map(|(k, v)| (k.clone(), v.into())),
    );
    cfg.extend(source_properties);

    cfg.create().context("invalid kafka source configuration")
}

pub fn init_sink(
    default_config: &Kafka,
    sink_config: &KafkaSink,
    sink_properties: Vec<(String, String)>,
) -> anyhow::Result<rdkafka::producer::FutureProducer> {
    let mut cfg = ClientConfig::new();
    cfg.extend(
        default_config
            .properties
            .iter()
            .map(|(k, v)| (k.clone(), v.into())),
    );
    cfg.extend(
        sink_config
            .properties
            .iter()
            .map(|(k, v)| (k.clone(), v.into())),
    );
    cfg.extend(sink_properties);

    cfg.create().context("invalid kafka sink configuration")
}
