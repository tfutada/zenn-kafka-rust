use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use futures::{StreamExt, TryStreamExt};
use tokio;
use std::sync::Arc;

// debug log
// RUST_LOG=librdkafka=trace,rdkafka::client=debug
#[tokio::main]
async fn main() {
    env_logger::init();
    let consumer = Arc::new(ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("enable.partition.eof", "false")
        .set("debug", "consumer")
        .set("group.id", "my-group-3")
        .create::<StreamConsumer>()
        .expect("Failed to create client"));

    consumer.subscribe(&["my-topic-1"]).unwrap();

    while let Some(message) = consumer.stream().next().await {
        match message {
            Ok(msg) => {
                // Extract a payload from the message.
                let tailored_msg = match msg.payload_view::<str>() {
                    Some(Ok(payload)) => {
                        format!("Prepared payload: {}, len: {}", payload, payload.len())
                    }
                    Some(Err(_)) => "Message payload is not a string".to_owned(),
                    None => "No payload".to_owned(),
                };

                // Do a heavy task with a tokio thread.
                tokio::spawn(async move {
                    println!("process the msg: {}", &tailored_msg[..30]);
                    // For example, run a batch processing, making API calls, etc.
                    tokio::time::sleep(tokio::time::Duration::from_millis(10_000)).await;
                    println!("tokio Done!");
                });
            }
            Err(e) => eprintln!("Error receiving message: {:?}", e),
        }
    }

    println!("Fine!");
}
