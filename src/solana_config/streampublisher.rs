//! This file contains the streampublisher, a struct containing all the StreamPublisherConnections.
//! This is specific to blockchains since we have different outputs per blockchain.

// Conditional imports
use crate as blockchain_generic;

use blockchain_generic::output::publish::StreamPublisherConnection;
use log::info;

#[cfg(feature = "GOOGLE_PUBSUB")]
use blockchain_generic::output::google_pubsub::connect;
#[cfg(feature = "JSON")]
use blockchain_generic::output::json::connect;
#[cfg(feature = "JSONL")]
use blockchain_generic::output::jsonl::connect;
#[cfg(feature = "RABBITMQ_CLASSIC")]
use blockchain_generic::output::rabbitmq_classic::connect;
#[cfg(feature = "RABBITMQ_STREAM")]
use blockchain_generic::output::rabbitmq_stream::connect;

/// StreamPublisher struct (seperate-publisher version) that contains various output
/// streams for different content.
#[cfg(feature = "SEPARATE_PUBLISHERS")]
#[derive(Clone)]
pub struct StreamPublisher {
    pub blocks: StreamPublisherConnection,
    pub block_rewards: StreamPublisherConnection,
    pub transactions: StreamPublisherConnection,
    pub instructions: StreamPublisherConnection,
    pub tokens: StreamPublisherConnection,
    pub token_transfers: StreamPublisherConnection,
    pub accounts: StreamPublisherConnection,
}

#[cfg(feature = "SEPARATE_PUBLISHERS")]
impl StreamPublisher {
    pub async fn new() -> StreamPublisher {
        info!("Connecting to the publishers...");
        StreamPublisher {
            blocks: connect("QUEUE_NAME_BLOCKS").await,
            block_rewards: connect("QUEUE_NAME_BLOCK_REWARDS").await,
            transactions: connect("QUEUE_NAME_TRANSACTIONS").await,
            instructions: connect("QUEUE_NAME_INSTRUCTIONS").await,
            token_transfers: connect("QUEUE_NAME_TOKEN_TRANSFERS").await,
            tokens: connect("QUEUE_NAME_TOKENS").await,
            accounts: connect("QUEUE_NAME_ACCOUNTS").await,
        }
    }

    #[cfg(not(feature = "PUBLISH_WITH_NAME"))]
    pub async fn disconnect(self) {
        info!("Disconnecting from publishers...");
        self.blocks.disconnect().await;
        self.block_rewards.disconnect().await;
        self.transactions.disconnect().await;
        self.instructions.disconnect().await;
        self.token_transfers.disconnect().await;
        self.tokens.disconnect().await;
        self.accounts.disconnect().await;
    }
}
