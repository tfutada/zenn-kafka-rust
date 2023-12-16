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

    for i in 0..10 {
        let message = format!("Message {}", i); // Create the message
        let future = producer.send(
            FutureRecord::to("my-topic-1")
                .payload(&message.clone()), // Clone the message and pass the clone
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
