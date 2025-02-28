use crate::generator::init_task;
use crate::kafka::{ConsumerExt, init_source};
use crate::settings::Setting;
use anyhow::Context;
use rdkafka::Message;
use rdkafka::consumer::Consumer;
use std::collections::HashMap;
#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

mod generator;
mod kafka;
mod log;
mod management;
mod settings;
mod utils;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Setting::emerge().context("reading config")?;

    log::init(settings.config.logging);

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
    let sink_topic = settings.config.kafka_sink.topic;

    let topics = if settings
        .config
        .kafka_source
        .topics
        .contains(&"*".to_string())
    {
        consumer
            .list_topics()
            .await?
            .into_iter()
            .filter(|t| *t != sink_topic)
            .collect()
    } else {
        settings.config.kafka_source.topics
    };

    let topics_ref = topics.iter().map(AsRef::as_ref).collect::<Vec<&str>>();

    consumer
        .subscribe(&topics_ref)
        .context("failed to subscribe to topic(s)")?;

    let topic_tasks: HashMap<_, _> = topics
        .into_iter()
        .map(init_task(&producer, &sink_topic))
        .collect();

    tokio::task::spawn(management::run(settings.config.management_port));

    loop {
        let message = consumer.recv().await?;
        let topic = message.topic().to_string();

        let task = topic_tasks
            .get(&topic)
            .context("got a message for a topic we're not subscribed to")?;

        task.send(message.detach())
            .await
            .context("failed to send message to handler")?;
    }
}
