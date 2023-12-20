/// This module consists of feature checks during compilation that will raise compiler errors if the feature
/// selection is invalid.This module will raise a compiler error for commonly known feature selection
/// contradictions (like using `RABBITMQ_QUEUE` and `RABBITMQ_STREAM` and when a key feature selection is
/// missing (i.e. no block chain feature selected like `SOLANA`).
///
/// Feature contradiction / requirements should be added to this module as they are created.

#[cfg(all(feature = "RPC", feature = "REST"))]
compile_error!("Features `RPC` and `REST` are mutually exclusive. Please select only one.");

// Choosing the output publisher

#[cfg(all(feature = "GOOGLE_PUBSUB", feature = "RABBITMQ_STREAM"))]
compile_error!("Features `GOOGLE_PUBSUB` and `RABBITMQ_STREAM` are mutually exclusive. Please select only one.");

#[cfg(all(feature = "GOOGLE_PUBSUB", feature = "RABBITMQ_CLASSIC"))]
compile_error!("Features `GOOGLE_PUBSUB` and `RABBITMQ_CLASSIC` are mutually exclusive. Please select only one.");

#[cfg(all(feature = "RABBITMQ_CLASSIC", feature = "RABBITMQ_STREAM"))]
compile_error!("Features `RABBITMQ_CLASSIC` and `RABBITMQ_STREAM` are mutually exclusive. Please select only one.");

#[cfg(not(any(
    feature = "GOOGLE_PUBSUB",
    feature = "RABBITMQ_STREAM",
    feature = "RABBITMQ_CLASSIC",
    feature = "JSONL",
    feature = "JSON"
)))]
compile_error!("Either `JSONL`, `JSON`, `GOOGLE_PUBSUB`, `RABBITMQ_STREAM`, or `RABBITMQ_CLASSIC` must be enabled.");

// Makes sure we either have one or multiple publishers

#[cfg(all(feature = "SINGLE_PUBLISHER", feature = "SEPARATE_PUBLISHERS"))]
compile_error!("Features `SINGLE_PUBLISHER` and `SEPARATE_PUBLISHERS` are mutually exclusive.  Please select only one.");

#[cfg(not(any(feature = "SINGLE_PUBLISHER", feature = "SEPARATE_PUBLISHERS")))]
compile_error!("Either `SINGLE_PUBLISHER` or `SEPARATE_PUBLISHERS` must be enabled");

// for now, solana is the only supported blockchain.
// in the future, this check should be implemented for all supported blockchains.
#[cfg(not(feature = "SOLANA"))]
compile_error!("No blockchain feature has been enabled. Please select one, such as `SOLANA`.");
