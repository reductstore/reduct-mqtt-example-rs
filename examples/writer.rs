use reduct_rs::ReductClient;
use rumqttc::Event::Incoming;
use rumqttc::Packet::Publish;
use rumqttc::{AsyncClient, MqttOptions, QoS};

#[tokio::main]
async fn main() {
    // Connect to ReductStore instance at 8383 port
    let client = ReductClient::builder().url("http://127.0.0.1:8383").build();

    // Get ot create a bucket named 'mqtt'
    let bucket = client
        .create_bucket("mqtt")
        .exist_ok(true)
        .send()
        .await
        .unwrap();

    // Connect to the mqtt broker
    let mqtt_options = MqttOptions::new("reduct", "127.0.0.1", 1883);
    let (mqtt, mut mqtt_loop) = AsyncClient::new(mqtt_options, 10);
    // Subscribe to all topics
    mqtt.subscribe("#", QoS::AtMostOnce).await.unwrap();

    // Write received message to the bucket
    // by using topic as an entry name
    while let Ok(notification) = mqtt_loop.poll().await {
        if let Incoming(Publish(packet)) = notification {
            let topic = packet.topic;
            let payload = packet.payload;

            bucket
                .write_record(&topic)
                .data(payload.clone())
                .send()
                .await
                .unwrap();
            println!(
                "Received message {} from {} is written to the bucket",
                String::from_utf8_lossy(&payload),
                topic,
            );
        }
    }
}
