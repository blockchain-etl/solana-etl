//! This module contains helper functions for making JSON RPC requests to a Solana node.
//!
//! Note: This may be moved in the future to consolidate the requests code.
use crate::solana_config::types::request_types;

#[cfg(not(feature = "SOLANA_BIGTABLE"))]
use crate as blockchain_generic;

#[cfg(not(feature = "SOLANA_BIGTABLE"))]
use {
    blockchain_generic::{
        metrics::Metrics,
        solana_config::{constants, data_sources::json_rpc},
        source::config::RequestConfig,
    },
    log::warn,
};

/// creates a post request body (as a `String`) to make an RPC call for [getBlockHeight()](https://docs.solana.com/api/http#getblockheight) for the given range.
pub fn get_block_height_post_body() -> String {
    String::from("{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"getBlockHeight\"}")
}

/// creates a post request body (as a `String`) to make an RPC call for [getBlock()](https://docs.solana.com/api/http#getblock) at the given index.
///     block_index: [Slot](https://docs.solana.com/terminology#slot) number, as u64 integer
pub fn get_block_post_body(block_index: u64) -> String {
    let post_body_struct = request_types::BlockRequest::new(block_index);
    serde_json::to_string(&post_body_struct).unwrap()
}

/// creates a post request body (as a `String`) to make an RPC call for [getBlocks()](https://docs.solana.com/api/http#getblocks) for the given range.
///     block_start: the start [Slot](https://docs.solana.com/terminology#slot) of the desired range
///     block_end: the end [Slot](https://docs.solana.com/terminology#slot) of the desired range
///         NOTE: Cannot be higher than 500,000 blocks higher than `block_start`
pub fn get_blocks_post_body(block_start: u64, block_end: u64) -> String {
    let post_body_struct = request_types::BlocksRequest::new(block_start, block_end);
    serde_json::to_string(&post_body_struct).unwrap()
}

pub fn get_slot_post_body() -> String {
    let post_body_struct = request_types::SlotRequest::new();
    serde_json::to_string(&post_body_struct).unwrap()
}

/// creates a post request body (as a `String`) to make an RPC call for getMultipleAccounts() for the given PubKeys.
///     account - A vector of up to 100 [pubkeys](https://docs.solana.com/terminology#public-key-pubkey)
pub fn get_multiple_accounts_post_body(account: Vec<String>) -> String {
    let post_body_struct = request_types::AccountsRequest::new(account);
    serde_json::to_string(&post_body_struct).unwrap()
}

/// Returns a recent block (close to the tip of the chain) using the rpc node.
/// Continuously calls the API until it receives a response.
/// If the block is older than what the RPC node(s) stores, then panics.
/// This should not occur if a proper fallback node is provided.
#[cfg(not(feature = "SOLANA_BIGTABLE"))]
pub async fn get_recent_block(
    request_config: RequestConfig,
    slot: u64,
    metrics: Option<Metrics>,
) -> Option<solana_transaction_status::UiConfirmedBlock> {
    let mut _request_config = request_config;
    let mut tried_fallback = false;
    loop {
        // Calls the getBlock() API. Will attempt primary node, then fall-back node.
        let block_response = blockchain_generic::call_getBlock(
            _request_config.try_clone().unwrap(),
            slot,
            metrics.clone(),
        )
        .await;

        let confirmed_block = block_response.result;
        match confirmed_block {
            None => {
                // In event that no block is returned in the response, we expect an error.
                match block_response.error {
                    None => {
                        warn!("Data source does not provide block at slot: {}", slot);
                        if tried_fallback {
                            return None;
                        }
                        if let Ok(fb) = dotenvy::var("FALLBACK_ENDPOINT") {
                            warn!("Trying again with the fallback endpoint...");
                            let request_body = json_rpc::get_block_post_body(slot);
                            let cur_request_builder =
                                blockchain_generic::use_fallback_endpoint(request_body.clone(), fb)
                                    .await;
                            _request_config = RequestConfig::ReqBldr(cur_request_builder);
                            tried_fallback = true;
                        } else {
                            return None;
                        }
                    }
                    Some(err) => {
                        // Solana's official documentation does not provide information on these error codes,
                        // but quicknode does: https://support.quicknode.com/hc/en-us/articles/16459608696721-Solana-RPC-Error-Code-Reference
                        match err.code {
                            constants::LEDGER_JUMP_ERROR_CODE | constants::INTERNAL_ERROR_CODE => {
                                // The data source does not provide this block (and it never will).
                                // Switches to the fallback endpoint.
                                warn!("Data source does not provide block at slot: {}", slot);
                                if tried_fallback {
                                    return None;
                                }
                                if let Ok(fb) = dotenvy::var("FALLBACK_ENDPOINT") {
                                    warn!("Trying again with the fallback endpoint...");
                                    let request_body = json_rpc::get_block_post_body(slot);
                                    let cur_request_builder =
                                        blockchain_generic::use_fallback_endpoint(
                                            request_body.clone(),
                                            fb,
                                        )
                                        .await;
                                    _request_config = RequestConfig::ReqBldr(cur_request_builder);
                                    tried_fallback = true;
                                } else {
                                    return None;
                                }
                            }
                            constants::SKIPPED_SLOT_ERROR_CODE | constants::NO_TX_HISTORY => {
                                // The slot doesn't have any block (and it never will).
                                // Safe to ignore.
                                warn!("Slot {:?} does not contain a block (skipped slot)", slot);
                                return None;
                            }
                            constants::OLD_BLOCK_SLOT_ERROR_CODE => {
                                // Too far behind the tip of the chain
                                warn!(
                                    "The data source does not provide data for blocks this old: {}", slot
                                );
continue;
                            }
                            constants::UNCONFIRMED_BLOCK_SLOT_ERROR_CODE | constants::NO_STATUS => {
                                // The slot currently doesn't have a block, but it might in the future.
                                warn!(
                                    "Attempted to access slot {:?}, but it is not yet confirmed.",
                                    slot
                                );
                                continue;
                            }
                            _ => {
                                panic!("FATAL: unexpected error: {:?}, for slot {}", err, slot);
                            }
                        }
                    }
                }
            }
            Some(block) => return Some(block),
        }
    }
}
