//! This module contains implementation details for
//! StreamPublisherConnection when the `RABBITMQ_STREAM`
//! feature is enabled.  This allows StreamPublisherConnection
//! to connect and publish to the RabbitMQ Stream (not to be
//! confused with RabbitMQ Classic Queue)

// Standard imports
use log::info;

// 3rd party imports
use rabbitmq_stream_client::types::Message;

// local imports
use super::publish::{StreamPublisherConnection, StreamPublisherConnectionClient};

impl StreamPublisherConnectionClient {
    /// Sends a message to the RabbitMQ Stream server.
    #[inline]
    pub async fn publish(&self, msg: Vec<u8>) {
        let StreamPublisherConnectionClient::RabbitMQStream(rabbitmq_publisher) = self;
        rabbitmq_publisher
            .send_with_confirm(Message::builder().body(msg).build())
            .await
            .expect("FATAL: could not send the rabbitmq message to the stream queue");
    }

    /// Disconnects from the RabbitMQ server stream
    pub async fn disconnect(self) {
        let StreamPublisherConnectionClient::RabbitMQStream(rabbitmq_producer) = self;
        rabbitmq_producer
            .close()
            .await
            .expect("FATAL: could not close the producer");
    }
}

/// Connects to the RabbitMQ Classic queue system.
/// Expects the following parameters to be stored in the .env file:
/// - `RABBITMQ_ADDRESS`
/// - `RABBITMQ_PORT`
/// - `environment`
/// - `RABBITMQ_PASSWORD`
///
/// NOTE: We also expect whatever string is passed for `queue_name` to
/// also appear in the .env file with the name that is going to be used.
/// This means you do not pass the queue name for `queue_name`, rather
/// the name of the parameter in the .env file that reflects the name
/// for the queue.
pub async fn connect(queue_name: &str) -> StreamPublisherConnection {
    // Extract values from the .env
    let rabbitmq_address = dotenvy::var("RABBITMQ_ADDRESS")
        .expect("RABBITMQ_ADDRESS should exist in .env file")
        .parse::<String>()
        .unwrap();
    let rabbitmq_port = dotenvy::var("RABBITMQ_PORT")
        .expect("RABBITMQ_PORT should exist in .env file")
        .parse::<u16>()
        .unwrap();
    let rabbitmq_environment = rabbitmq_stream_client::Environment::builder()
        .host(&rabbitmq_address)
        .port(rabbitmq_port)
        .build()
        .await
        .expect("FATAL: could not create rabbitmq environment");
    let rabbitmq_queue_name = dotenvy::var(queue_name)
        .unwrap_or_else(|_| panic!("{} should exist in .env file", queue_name))
        .parse::<String>()
        .unwrap();
    info!("Successfully created the rabbitmq environment");

    // this will cause a panic if the stream HAS been created:
    //environment.stream_creator().create(constants::STREAM_NAME).await.expect("FATAL: stream already exists");

    // this will cause a panic if the stream has NOT been created:
    let producer = rabbitmq_environment
        .producer()
        .build(&rabbitmq_queue_name)
        .await
        .expect("FATAL: stream has not yet been created");

    StreamPublisherConnection {
        client: StreamPublisherConnectionClient::RabbitMQStream(producer),
        queue_name: rabbitmq_queue_name,
    }
}

impl StreamPublisherConnection {
    /// Sends the message to the client
    #[inline]
    pub async fn publish<T: prost::Message>(&self, msg: T) {
        self.client.publish(msg.encode_to_vec()).await;
    }
    /// Sends the message to the client
    pub async fn disconnect(self) {
        self.client.disconnect().await;
    }
}
