use rustdds::*;
use rustdds::serialization::CDRSerializerAdapter;
use serde::Serialize;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Clone, Debug)]
struct SomeType {
    a: i32,
}

fn main() {
    // Create a DomainParticipant
    let domain_participant = DomainParticipant::new(0).expect("Failed to create DomainParticipant");

    // Define QoS policies for the publisher
    let qos = QosPolicyBuilder::new()
        .reliability(policy::Reliability::Reliable { max_blocking_time: rustdds::Duration::ZERO })
        .build();

    // Create a Publisher
    let publisher = domain_participant.create_publisher(&qos).expect("Failed to create publisher");
    println!("Publisher created.");

    // Create a topic named "some_topic" of type "SomeType" with NoKey
    let some_topic = domain_participant.create_topic(
        "some_topic".to_string(),
        "SomeType".to_string(),
        &qos,
        TopicKind::NoKey,
    ).expect("Failed to create topic");
    println!("Topic created.");

    // Create a DataWriter for publishing SomeType data
    let writer = publisher
        .create_datawriter_no_key::<SomeType, CDRSerializerAdapter<SomeType>>(
            &some_topic,
            None,
        )
        .expect("Failed to create data writer");
    println!("Data writer created.");

    // Create a thread to continuously publish data
    thread::spawn(move || {
        let mut count = 1;
        loop {
            let some_data = SomeType { a: count };

            // Publish data
            writer.write(some_data.clone(), None).expect("Failed to write data");
            println!("Data published: {:?}", some_data);

            // Increment count for the next data
            count += 1;

            // Sleep for 1 second
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Keep the main thread alive
    loop {
        thread::sleep(Duration::from_secs(10)); // Sleep to keep the main thread alive
    }
}
