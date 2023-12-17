use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use tokio; // Ensure you have the tokio runtime available

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let mut tasks = Vec::new();

    for i in 0..10 {
        let message = "a".repeat(100_000);
        let producer_clone = producer.clone();

        // Spawn a new task for sending each message
        let task = tokio::spawn(async move {
            let record = FutureRecord::to("my-topic-1")
                .key("my-key-1")
                .payload(&message);

            match producer_clone.send(record, Timeout::Never).await {
                Ok(delivery) => println!("Sent: {:?}", delivery),
                Err((e, _)) => eprintln!("Error: {:?}", e),
            }
        });

        tasks.push(task);
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.expect("Task failed to complete");
    }

    println!("All messages sent");
}
