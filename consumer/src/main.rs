use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
};

#[tokio::main]
async fn main() {
    println!("Consumer starting...");
    let consumer: StreamConsumer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            "kafka-cluster-kafka-bootstrap.kafka.svc:9092",
        )
        .set("group.id", "consumer-group-1")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Can't initialize Kafka consumer.");

    consumer
        .subscribe(&["events"])
        .expect("Can't subscribe to topic");

    loop {
        match consumer.recv().await {
            Ok(message) => {
                println!("Message: {:?}", message.payload());
            }
            Err(e) => {
                eprintln!("Kafka error: {}", e);
            }
        }
    }
}
