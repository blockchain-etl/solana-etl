//! This file stores output output struct that compile different functions
//! depending on the compilation features enabled.  The objective is to allow
//! the structs and functions provided in this module regardless of the comilation
//! features enabled.

// Import the StreamPublisher.  Should either be the single_stream_publisher or a blockchain specific one
#[cfg(feature = "SINGLE_PUBLISHER")]
pub use super::single_stream_publisher::StreamPublisher;
#[cfg(all(feature = "SEPARATE_PUBLISHERS", feature = "SOLANA"))]
pub use crate::solana_config::streampublisher::StreamPublisher;

// Get the appropriate connect
#[cfg(feature = "GOOGLE_PUBSUB")]
pub use super::google_pubsub::connect;
#[cfg(feature = "JSON")]
pub use super::json::connect;
#[cfg(feature = "JSONL")]
pub use super::jsonl::connect;
#[cfg(feature = "RABBITMQ_CLASSIC")]
pub use super::rabbitmq_classic::connect;
#[cfg(feature = "RABBITMQ_STREAM")]
pub use super::rabbitmq_stream::connect;

/// An enum that represents a connection to an output.  Will only contain one item
/// dependent on the enabled features.
#[derive(Clone)]
pub enum StreamPublisherConnectionClient {
    #[cfg(feature = "GOOGLE_PUBSUB")]
    GcpPubSub(google_cloud_pubsub::publisher::Publisher),
    #[cfg(feature = "RABBITMQ_CLASSIC")]
    RabbitMQClassic(amqprs::connection::Connection),
    #[cfg(feature = "RABBITMQ_STREAM")]
    RabbitMQStream(rabbitmq_stream_client::Producer<rabbitmq_stream_client::NoDedup>),
    #[cfg(feature = "JSONL")]
    JsonL(std::path::PathBuf),
    #[cfg(feature = "JSON")]
    Json(std::path::PathBuf),
}

/// A struct that contains the client used to connect to the publisher and the queue_name
#[derive(Clone)]
pub struct StreamPublisherConnection {
    /// The `client` is an Enum with a singular item depending on the enabled features that
    /// contain the functionality of publishing
    pub client: StreamPublisherConnectionClient,
    /// The `queue_name` is a string to represent the output stream.  This would be things like
    /// the google pubsub topic, the rabbitmq queue or stream name, etc.
    pub queue_name: String,
    /// Channel is only compiled when `RABBITMQ_CLASSIC` feature is enabled.  It is Optional as
    /// you cannot create a Channel and utilize it in a different thread.  You should create a
    /// a Channel for this publisher once you are in the thread you intend to use the publisher in.
    #[cfg(feature = "RABBITMQ_CLASSIC")]
    pub channel: Option<amqprs::channel::Channel>,
}
