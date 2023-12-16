use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use futures::{StreamExt, TryStreamExt};
use tokio;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let consumer = Arc::new(ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("enable.partition.eof", "false")
        .set("group.id", "my-group-1")
        .create::<StreamConsumer>()
        .expect("Failed to create client"));

    consumer.subscribe(&["my-topic-1"]).unwrap();

    let consumer_clone = Arc::clone(&consumer);
    let mut message_stream = consumer_clone.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(msg) => {
                println!("Received message from topic: {}, partition: {}, offset: {}, timestamp: {:?}",
                         msg.topic(), msg.partition(), msg.offset(), msg.timestamp());

                // Prepare the message
                let prepared_message = match msg.payload_view::<str>() {
                    Some(Ok(payload)) => {
                        // Implement your preparation logic here
                        // For example, parsing JSON, checking message format, etc.
                        format!("Prepared payload: {}, len: {}", payload, payload.len())
                    }
                    Some(Err(_)) => "Message payload is not a string".to_owned(),
                    None => "No payload".to_owned(),
                };

                // Do heavy task using tokio::spawn
                tokio::spawn(async move {
                    // Implement heavy task here
                    println!("Performing heavy task with the message");
                    // For example, processing data, making API calls, etc.
                    tokio::time::sleep(tokio::time::Duration::from_millis(10_000)).await;
                    println!("Heavy task completed");
                });

                println!("{}", prepared_message);

                // Use consumer_clone if needed for further processing
            }
            Err(e) => eprintln!("Error receiving message: {:?}", e),
        }
    }
}
