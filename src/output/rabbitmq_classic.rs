//! This module contains implementation details for
//! StreamPublisherConnection when the `RABBITMQ_CLASSIC`
//! feature is enabled.  This allows StreamPublisherConnection
//! to connect and publish to the RabbitMQ Classic (not to be
//! confused with RabbitMQ Stream)

use super::publish::{StreamPublisherConnection, StreamPublisherConnectionClient};
use log::info;
use prost::Message;

//use amqprs::channel::Channel;

/// Connects to the RabbitMQ Classic queue system.
/// Expects the following parameters to be stored in the .env file:
/// - `RABBITMQ_ADDRESS`
/// - `RABBITMQ_PORT`
/// - `RABBITMQ_USER`
/// - `RABBITMQ_PASSWORD`
pub async fn connect(queue_name: &str) -> StreamPublisherConnection {
    // Extract necessary information from the .env from the queue
    let address = dotenvy::var("RABBITMQ_ADDRESS")
        .expect("RABBITMQ_ADDRESS should exist in .env file")
        .parse::<String>()
        .unwrap();
    let port = dotenvy::var("RABBITMQ_PORT")
        .expect("RABBITMQ_PORT should exist in .env file")
        .parse::<u16>()
        .unwrap();
    let user = dotenvy::var("RABBITMQ_USER")
        .expect("RABBITMQ_USER should exist in .env file")
        .parse::<String>()
        .unwrap();
    let password = dotenvy::var("RABBITMQ_PASSWORD")
        .expect("RABBITMQ_PASSWORD should exist in .env file")
        .parse::<String>()
        .unwrap();
    let rabbitmq_queue_name = dotenvy::var(queue_name)
        .unwrap_or_else(|_| panic!("{} should exist in .env file", queue_name))
        .parse::<String>()
        .unwrap();

    info!("Creating rabbitmq environment...");
    let connection = amqprs::connection::Connection::open(
        &amqprs::connection::OpenConnectionArguments::new(&address, port, &user, &password),
    )
    .await
    .expect("rabbitmq server has been setup");

    connection
        .register_callback(amqprs::callbacks::DefaultConnectionCallback)
        .await
        .unwrap();

    StreamPublisherConnection {
        client: StreamPublisherConnectionClient::RabbitMQClassic(connection),
        queue_name: rabbitmq_queue_name,
        channel: None,
    }
}

impl StreamPublisherConnectionClient {
    /// Establishes a connection to the RabbitMQ Server
    pub async fn establish_connection(&self, queue_name: &str) -> amqprs::channel::Channel {
        let StreamPublisherConnectionClient::RabbitMQClassic(connection) = self;
        let channel = connection.open_channel(None).await.unwrap();

        channel
            .register_callback(amqprs::callbacks::DefaultChannelCallback)
            .await
            .unwrap();

        let _ = channel
            .queue_declare(amqprs::channel::QueueDeclareArguments::durable_client_named(queue_name))
            .await
            .unwrap()
            .unwrap();
        channel
    }

    /// Disconnects from the RabbitMQ server
    pub async fn disconnect(self) {
        let StreamPublisherConnectionClient::RabbitMQClassic(connection) = self;
        let _ = connection.close().await;
    }
}

impl StreamPublisherConnection {
    /// Returns a new StreamPublisherConnection with a channel.  This instance cannot be moved between
    /// threads safely.
    ///
    /// NOTE: You cannot use this function and send the resulting StreamPublisherConnection
    /// to another thread, as the channel cannot move threads.  Instead, you should
    /// call this function once you are in the thread you intend to use the publisher.
    pub async fn with_channel(self) -> StreamPublisherConnection {
        // Create a channel with the current client
        let channel = Some(self.client.establish_connection(&self.queue_name).await);
        // Create a new StreamPublisherConnection
        StreamPublisherConnection {
            client: self.client,
            queue_name: self.queue_name,
            channel,
        }
    }

    /// Sends the message to the RabbitMQ classic queue.
    ///
    /// NOTE: Will panic if channel is not yet created.  The `RABBITMQ_CLASSIC` feature
    /// creates a connection without a channel to allow the StreamPublisherConnection to move
    /// between threads safely.  Once in the thread you intend to publish in, you can call
    /// `with_channel` to return a StreamPublisherConnection with the same `client` and
    /// `queue_name`, but also with a channel that will only be functional in the current
    /// thread.
    #[inline]
    pub async fn publish<T: Message>(&self, msg: T) {
        let args = amqprs::channel::BasicPublishArguments::new("", &self.queue_name);
        self.channel
            .as_ref()
            .unwrap()
            .basic_publish(
                amqprs::BasicProperties::default(),
                msg.encode_to_vec(),
                args.clone(),
            )
            .await
            .unwrap();
    }

    /// Disconnects the client.  Should be called before terminating the program.
    pub async fn disconnect(self) {
        self.client.disconnect().await;
    }
}
