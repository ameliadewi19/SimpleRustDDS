use rustdds::*;
use rustdds::serialization::{CDRSerializerAdapter, CDRDeserializerAdapter};
use serde::{Serialize, Deserialize};

fn main() {
    // DomainParticipant is always necessary
    let domain_participant = DomainParticipant::new(0).expect("Failed to create DomainParticipant");

    let qos = QosPolicyBuilder::new()
        .reliability(policy::Reliability::Reliable { max_blocking_time: rustdds::Duration::ZERO })
        .build();

    // DDS Subscriber, only one is necessary for each thread (slight difference to DDS specification)
    let subscriber = domain_participant.create_subscriber(&qos).expect("Failed to create subscriber");

    // DDS Publisher, only one is necessary for each thread (slight difference to DDS specification)
    let publisher = domain_participant.create_publisher(&qos).expect("Failed to create publisher");

    // Some DDS Topic that we can write and read from (basically only binds readers and writers together)
    let some_topic = domain_participant.create_topic(
        "some_topic".to_string(),
        "SomeType".to_string(),
        &qos,
        TopicKind::NoKey,
    ).expect("Failed to create topic");

    // Used type needs Serialize for writers and Deserialize for readers
    #[derive(Serialize, Deserialize, Debug)]
    struct SomeType {
        a: i32
    }

    // Creating DataReader requires type and deserializer adapter (which is recommended to be CDR).
    // Reader needs to be mutable if any operations are used.
    let mut reader = subscriber
        .create_datareader_no_key::<SomeType, CDRDeserializerAdapter<SomeType>>(
            &some_topic,
            None,
        )
        .expect("Failed to create data reader");

    // Creating DataWriter required type and serializer adapter (which is recommended to be CDR).
    let writer = publisher
        .create_datawriter_no_key::<SomeType, CDRSerializerAdapter<SomeType>>(
            &some_topic,
            None,
        )
        .expect("Failed to create data writer");

    // Readers implement mio Evented trait and thus function the same way as std::sync::mpsc and can be handled the same way for reading the data

    let some_data = SomeType { a: 1 };

    // This should send the data to all who listen to the "some_topic" topic.
    writer.write(some_data, None).expect("Failed to write data");

    // ... Some data has arrived at some point for the reader
    let data_sample = if let Ok(Some(value)) = reader.take_next_sample() {
        value
    } else {
        // no data has arrived
        return;
    };

    // Getting reference to actual data from the data sample
    let actual_data = data_sample.value();

    println!("Received data: {:?}", actual_data);
}
