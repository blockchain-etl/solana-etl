//! This module contains implementation details for
//! StreamPublisherConnection when the `GOOGLE_PUBSUB`
//! feature is enabled.  This allows StreamPublisherConnection
//! to connect and publish to the GCP's PubSub service.
use log::info;
use log::warn;
use std::time;
use tokio::time::sleep;

use google_cloud_auth::credentials::CredentialsFile;
use google_cloud_googleapis::pubsub::v1::PubsubMessage;
use google_cloud_pubsub::{
    client::{Client, ClientConfig},
    publisher::Publisher,
};

use prost::Message;

use super::publish::{StreamPublisherConnection, StreamPublisherConnectionClient};

/// Establishes the connection to the Google Cloud Pub/Sub extracting the credentials
/// and information from the .env file.  This function creates the connection for
/// using a single publisher.
/// Must have the `GCP_CREDENTIAL_JSON_PATH` filepath pointing to the credentials json file,
/// and have `GOOGLE_PUBSUB_TOPIC` string saved in the .env file.
pub async fn connect(queue_name: &str) -> StreamPublisherConnection {
    let gcp_credentials_file = {
        let gcp_credentials_json_path = dotenvy::var("GCP_CREDENTIALS_JSON_PATH")
            .expect("GCP_CREDENTIALS_JSON_PATH should exist in .env file")
            .parse::<String>()
            .unwrap();

        CredentialsFile::new_from_file(gcp_credentials_json_path)
            .await
            .expect("GCP credentials file exists")
    };

    let topic_name = dotenvy::var(queue_name)
        .unwrap_or_else(|_| panic!("{} should exist in .env file", queue_name))
        .parse::<String>()
        .unwrap();

    // Attempt to open the credentials file to create the configuration
    let gcp_config = ClientConfig::default()
        .with_credentials(gcp_credentials_file)
        .await
        .unwrap();

    // Attempt to create the client using the configuration from above
    let gcp_client = Client::new(gcp_config).await.unwrap();

    // Use the client to connect to the specific topic.
    connect_to_topic(gcp_client.clone(), &topic_name).await
}

/// Establishes a connection to the Google Cloud Pub/Sub Topic.  Assumes that the
/// pubsub topic has already been created in Google Cloud Platform (GCP), and panics
/// if the topic does not exist.
/// Should provide the GCP Client and the topic_name, where `topic_name` **is the name
/// of a property in the .env file**.  Not the actual topic name itself.
async fn connect_to_topic(
    gcp_client: google_cloud_pubsub::client::Client,
    topic_name: &str,
) -> StreamPublisherConnection {
    let google_pubsub_topic = dotenvy::var(topic_name)
        .expect("GOOGLE_PUBSUB_TOPIC should exist in .env file")
        .parse::<String>()
        .unwrap();

    // NOTE: assumes that this pubsub topic has already been created in GCP.
    let topic = gcp_client.topic(&google_pubsub_topic);

    if !topic.exists(None).await.unwrap() {
        panic!("Topic {} doesn't exist! Terminating...", topic_name);
    } else {
        info!("Topic exists. Proceeding...");
    }
    let publisher = topic.new_publisher(None);
    StreamPublisherConnection {
        client: StreamPublisherConnectionClient::GcpPubSub(publisher),
        queue_name: topic_name.to_string(),
    }
}

/// creates a PubsubMessage object using the bytes
fn prepare_message(serialized_block: Vec<u8>) -> PubsubMessage {
    PubsubMessage {
        data: serialized_block,
        ..Default::default()
    }
}

#[allow(non_snake_case)]
impl StreamPublisherConnectionClient {
    /// Sends a message to a Google Pub/Sub topic
    pub async fn publish(&self, msg: Vec<u8>) {
        let StreamPublisherConnectionClient::GcpPubSub(Publisher) = self;
        // publish the message
        let prepared_msg = prepare_message(msg);
        publish_with_backoff(Publisher, prepared_msg).await;
    }

    /// Sends a batch of messages to a Google Pub/Sub topic
    pub async fn publish_batch(&self, msg_batch: Vec<Vec<u8>>) {
        let StreamPublisherConnectionClient::GcpPubSub(Publisher) = self;
        // publish the message batch

        let prepared_msgs: Vec<PubsubMessage> =
            msg_batch.into_iter().map(prepare_message).collect();
        let message_chunks = prepared_msgs.chunks(900);
        for chunk in message_chunks.into_iter() {
            publish_batch_with_backoff(Publisher, chunk.to_vec()).await;
        }
    }

    pub async fn disconnect(&mut self) {
        let StreamPublisherConnectionClient::GcpPubSub(Publisher) = self;
        let gcp_publisher = Publisher;
        gcp_publisher.shutdown().await;
    }
}

/// Publishes a message to google cloud pub/sub.
/// Each time publishing fails, the sleep time is increased by 1 second.
async fn publish_with_backoff(publisher: &Publisher, message: PubsubMessage) {
    let awaiter = publisher.publish(message.clone()).await;
    let mut res = awaiter.get().await;
    let mut backoff = 0;
    loop {
        info!("Message publish result: {:?}", res);
        match res {
            Ok(_) => break,
            Err(_) => {
                warn!("publish failed for publisher: {:?}", publisher);
                let seconds = time::Duration::from_secs(backoff);
                sleep(seconds).await;
                backoff += 1;
                let awaiter = publisher.publish(message.clone()).await;
                res = awaiter.get().await;
            }
        }
    }
}

/// Attempts to publish a batch of messages to google cloud pub/sub.
/// If publishing fails, each individual message is published separately.
async fn publish_batch_with_backoff(publisher: &Publisher, messages: Vec<PubsubMessage>) {
    let awaiters = publisher.publish_bulk(messages.clone()).await;
    for (i, awaiter) in awaiters.into_iter().enumerate() {
        let res = awaiter.get().await;
        match res {
            Err(_) => {
                let msg = messages[i].clone();
                publish_with_backoff(publisher, msg).await;
            }
            Ok(_) => continue,
        }
    }
}

impl StreamPublisherConnection {
    /// Sends the message to the client
    pub async fn publish<T: Message>(&self, msg: T) {
        self.client.publish(msg.encode_to_vec()).await;
    }
    /// Sends the messages to the client
    pub async fn publish_batch<T: Message>(&self, msgs: Vec<T>) {
        self.client
            .publish_batch(msgs.iter().map(|msg| msg.encode_to_vec()).collect())
            .await;
    }

    /// Sends the message to the client
    pub async fn disconnect(mut self) {
        self.client.disconnect().await;
    }
}
