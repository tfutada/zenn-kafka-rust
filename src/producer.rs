use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let mut futures = Vec::new();
    let mut messages = Vec::new(); // Create a vector to store the cloned messages
    for i in 0..10 {
        let message = format!("Message {}", i);
        messages.push(message);
    }

    for msg in &messages {
        let future = producer.send(
            FutureRecord::<(), String>::to("my-topic-1")
                .payload(msg), // Use the cloned message
            Timeout::Never,
        );
        futures.push(future);
    }


    for future in futures {
        match future.await {
            Ok(delivery) => println!("Sent: {:?}", delivery),
            Err((e, _)) => eprintln!("Error: {:?}", e),
        }
    }

    println!("Messages sent");
}
