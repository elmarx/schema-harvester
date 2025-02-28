use rdkafka::Message;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use schema_harvester::model::NodeType;
use schema_harvester::{SchemaHypothesis, render_schema};
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::info;

pub fn init_task(
    producer: &FutureProducer,
    sink_topic: &str,
) -> impl Fn(String) -> (String, Sender<OwnedMessage>) {
    |source_topic: String| {
        let producer = producer.clone();
        let sink_topic = sink_topic.to_string();

        let (tx, rx) = tokio::sync::mpsc::channel::<OwnedMessage>(10);

        info!("Subscribing to topic: {}", source_topic);
        tokio::spawn(task(producer, source_topic.clone(), sink_topic, rx));

        (source_topic, tx)
    }
}

pub async fn task(
    producer: FutureProducer,
    source_topic: String,
    sink_topic: String,
    mut rx: Receiver<OwnedMessage>,
) {
    let mut current_hypothesis = SchemaHypothesis::new(
        "https:://github.com/elmarx/schema-harvester".to_string(),
        source_topic.clone(),
        format!("Auto-generated schema for {}", source_topic),
    );

    while let Some(message) = rx.recv().await {
        let payload = message.payload();

        // TODO: proper error-handling (or rather skip null-messages)
        let payload = payload.unwrap();

        let payload = serde_json::from_slice(payload);

        // TODO: proper error-handling
        let payload = payload.unwrap();

        let hypothesis = NodeType::from(&payload);

        let new_hypothesis = current_hypothesis.clone().merge(hypothesis);

        // if the merged hypothesis is a different one than the one we used to know, print it
        if new_hypothesis != current_hypothesis {
            current_hypothesis = new_hypothesis;
            let current_hypothesis = render_schema(&current_hypothesis);
            let record = FutureRecord::to(&sink_topic)
                .key(&source_topic)
                .payload(&current_hypothesis);
            let delivery_status = producer
                .send::<_, _, _>(record, Duration::from_secs(0))
                .await;

            // TODO: proper error-handling
            delivery_status.unwrap();
        }
    }
}
