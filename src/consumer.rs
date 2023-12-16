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

    while let Some(message) = consumer.stream().next().await {
        match message {
            Ok(msg) => {
                // Prepare the message
                let prepared_message = match msg.payload_view::<str>() {
                    Some(Ok(payload)) => {
                        format!("Prepared payload: {}, len: {}", payload, payload.len())
                    }
                    Some(Err(_)) => "Message payload is not a string".to_owned(),
                    None => "No payload".to_owned(),
                };

                println!("{}", &prepared_message[..30]);

                tokio::spawn(async move {
                    println!("start!");
                    // For example, processing data, making API calls, etc.
                    tokio::time::sleep(tokio::time::Duration::from_millis(10_000)).await;
                    println!("Done!");
                });
            }
            Err(e) => eprintln!("Error receiving message: {:?}", e),
        }
    }
}
