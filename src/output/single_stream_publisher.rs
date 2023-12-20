//! This module defines the single StreamPublisher.  Since all outputs are managed through a singular stream,
//! it is not blockchain specific.  Note, StreamPublishers supporting seperate publishers should be implemented within
//! the blockchain.
use super::publish::StreamPublisherConnection;
use log::info;

// Get the appropriate connect
use super::publish::connect;

/// StreamPublisher struct (single-publisher version) that contains a singular Stream for
/// all output.
#[cfg(feature = "SINGLE_PUBLISHER")]
#[derive(Clone)]
pub struct StreamPublisher {
    pub all: StreamPublisherConnection,
}

#[cfg(feature = "SINGLE_PUBLISHER")]
impl StreamPublisher {
    pub async fn new() -> StreamPublisher {
        info!("Connecting to the publisher...");
        StreamPublisher {
            all: connect("QUEUE_NAME").await,
        }
    }

    pub async fn disconnect(self) {
        info!("Disconnecting from publisher...");
        self.all.disconnect().await;
    }
}
