use crate::kafka::init_source;
use crate::settings::Setting;
use anyhow::Context;
use futures::StreamExt;
use rdkafka::Message;
use rdkafka::consumer::{Consumer, DefaultConsumerContext, MessageStream};
use rdkafka::producer::FutureRecord;
use schema_harvester::{SchemaHypothesis, generate_hypothesis};
use serde_json::Value;
use std::time::Duration;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

mod kafka;
mod management;
mod settings;
mod utils;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Setting::emerge().context("reading config")?;

    let consumer = init_source(
        &settings.config.kafka,
        &settings.config.kafka_source,
        settings.kafka_source_properties,
    )?;
    let producer = kafka::init_sink(
        &settings.config.kafka,
        &settings.config.kafka_sink,
        settings.kafka_sink_properties,
    )?;
    let topic = settings.config.kafka_sink.topic;

    consumer
        .subscribe(&["events"])
        .context("failed to subscribe to topic")?;

    tokio::task::spawn(management::run(settings.config.management_port));

    let stream: MessageStream<DefaultConsumerContext> = consumer.stream();

    // turn the stream of messages into a stream of schema hypothesis
    let mut schema_stream = stream.map(|m| -> Result<Option<SchemaHypothesis>, anyhow::Error> {
        let schema = m?
            .payload_view::<str>()
            .transpose()?
            .map(serde_json::from_str::<Value>)
            .transpose()?
            .as_ref()
            .map(generate_hypothesis);

        Ok(schema)
    });

    let mut current_hypothesis: Option<SchemaHypothesis> = None;

    loop {
        let new = schema_stream.next().await.unwrap();

        // generate a new hypothesis
        let new_hypothesis = match (current_hypothesis.clone(), new) {
            (None, Ok(h)) => h,
            (Some(cur), Ok(None)) => Some(cur),
            (Some(cur), Ok(Some(new))) => Some(schema_harvester::merge_hypothesis(cur, new)),
            (cur, Err(e)) => {
                eprintln!("{:#?}", e);
                cur
            }
        };

        // if the merged hypothesis is a different one than the one we used to know, print it
        if new_hypothesis != current_hypothesis {
            current_hypothesis = new_hypothesis;
            let current_hypothesis =
                schema_harvester::render_schema(current_hypothesis.as_ref().unwrap());
            let record = FutureRecord::to(topic.as_str())
                .key("events")
                .payload(&current_hypothesis);
            let delivery_status = producer
                .send::<_, _, _>(record, Duration::from_secs(0))
                .await;

            delivery_status.unwrap();
        }
    }

    Ok(())
}
