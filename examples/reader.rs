use std::pin::pin;
use reduct_rs::ReductClient;
use futures_util::StreamExt;
#[tokio::main]
async fn main() {
    // Connect to ReductStore instance at 8383 port
    let client = ReductClient::builder().url("http://127.0.0.1:8383").build();

    // Get bucket named 'mqtt'
    let bucket = client
        .get_bucket("mqtt")
        .await
        .unwrap();

    // Read all entries from the bucket
    let entries = bucket
        .entries()
        .await
        .unwrap();

    for entry in entries {
        // Query all records from the entry
        let mut record_stream = bucket
            .query(&entry.name)
            .send()
            .await
            .unwrap();

        // Retrieve records as a stream
        let mut record_stream = pin!(record_stream);
        while let Some(record) = record_stream.next().await {
            let record = record.unwrap();
            println!(
                "MQTT topic: {}, Record: ts={}, data={}",
                entry.name,
                record.timestamp_us(),
                String::from_utf8_lossy(&record.bytes().await.unwrap()),
            );
        }
    }

}