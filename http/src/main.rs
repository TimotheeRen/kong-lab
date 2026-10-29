use std::time::Duration;

use axum::{Router, routing::get};
use rdkafka::{
    ClientConfig,
    producer::{FutureProducer, FutureRecord},
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/info", get(info));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Starting web server on :3000");
    axum::serve(listener, app).await.unwrap();
}

async fn info() -> String {
    let producer: FutureProducer = ClientConfig::new()
        .set(
            "bootstrap.servers",
            "kafka-cluster-kafka-bootstrap.kafka.svc:9092",
        )
        .set("message.timeout.ms", "1000")
        .create()
        .unwrap();

    producer
        .send(
            FutureRecord::to("events").key("event-1").payload("Hello"),
            Duration::from_secs(1),
        )
        .await
        .unwrap();

    "Mock server running on Axum: 0.8.9".to_string()
}
