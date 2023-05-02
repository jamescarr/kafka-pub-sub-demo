use rdkafka::{
    config::ClientConfig,
    consumer::{stream_consumer::StreamConsumer, Consumer},
    Message,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CustomMessage {
    message: String,
    timestamp: f64,
}

fn main() {
    let brokers = "kafka:9092";
    let topic = "test-topic";

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("group.id", "rust-consumer-group")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[topic])
        .expect("Failed to subscribe to topic");

    for message in consumer.start().wait() {
        if let Some(Err(err)) = message {
            eprintln!("Error while reading from stream: {}", err);
        }

        if let Some(Ok(msg)) = message {
            if let Some(payload) = msg.payload() {
                let payload_str = match std::str::from_utf8(payload) {
                    Ok(v) => v,
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                let deserialized_msg: CustomMessage =
                    serde_json::from_str(payload_str).expect("Failed to deserialize message");
                println!(
                    "Received message at offset {}: {:?}",
                    msg.offset(),
                    deserialized_msg
                );
            }
        }
    }
}
