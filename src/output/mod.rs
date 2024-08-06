#![doc = include_str!("README.md")]
pub mod publish;

#[cfg(feature = "SINGLE_PUBLISHER")]
pub mod single_stream_publisher;

#[cfg(feature = "GOOGLE_PUBSUB")]
pub mod google_pubsub;

#[cfg(feature = "RABBITMQ_CLASSIC")]
pub mod rabbitmq_classic;

#[cfg(feature = "RABBITMQ_STREAM")]
pub mod rabbitmq_stream;

pub mod tests;
