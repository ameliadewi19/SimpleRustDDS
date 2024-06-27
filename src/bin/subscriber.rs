use rustdds::*;
use rustdds::serialization::CDRDeserializerAdapter;
use serde::Deserialize;
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Clone, Debug)]
struct SomeType {
    #[allow(dead_code)]
    a: i32,
}

fn main() {
    let domain_participant = DomainParticipant::new(0).expect("Failed to create DomainParticipant");

    let qos = QosPolicyBuilder::new()
        .reliability(policy::Reliability::Reliable { max_blocking_time: rustdds::Duration::ZERO })
        .build();

    let subscriber = domain_participant.create_subscriber(&qos).expect("Failed to create subscriber");
    println!("Subscriber created.");

    let some_topic = domain_participant.create_topic(
        "some_topic".to_string(),
        "SomeType".to_string(),
        &qos,
        TopicKind::NoKey,
    ).expect("Failed to create topic");
    println!("Topic created.");

    // Add a short delay after creating the data reader
    thread::sleep(Duration::from_millis(100));

    let mut reader = subscriber
        .create_datareader_no_key::<SomeType, CDRDeserializerAdapter<SomeType>>(
            &some_topic,
            None,
        )
        .expect("Failed to create data reader");
    println!("Data reader created.");

    loop {
        match reader.take_next_sample() {
            Ok(Some(data_sample)) => {
                let actual_data = data_sample.value();
                println!("Received data: {:?}", actual_data);
            }
            Ok(None) => {
                // No data available yet
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(e) => {
                println!("Error reading data: {:?}", e);
            }
        }
    }
}
