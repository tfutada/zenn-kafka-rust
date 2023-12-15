use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};

use futures::{StreamExt, TryStreamExt};

#[tokio::main]
async fn main() {
    let consumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("enable.partition.eof", "false")
        .set("group.id", "my-group-1")
        .create::<StreamConsumer>()
        .expect("Failed to create client");

    consumer.subscribe(&["my-topic-1"]).unwrap();

    let stream_processor = consumer.stream().try_for_each(|msg| {
        async move {
            let payload = msg.payload()
                .and_then(|p| std::str::from_utf8(p).ok())
                .unwrap_or("[Empty message]");
            println!("Received message: {}", payload);
            Ok(())
        }
    });

    stream_processor.await.expect("stream processing failed");
}
