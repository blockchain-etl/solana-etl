use blockchain_etl_indexer as blockchain_generic;
use std::{mem::size_of_val, thread, time};

#[cfg(feature = "SOLANA")]
use {
    blockchain_etl_indexer::solana_config::types::block_response_types,
    solana_transaction_status::UiConfirmedBlock as Block,
};

use blockchain_etl_indexer::source::config::RequestConfig;

/// Determines an estimate of the blockchain's throughput (block size * number of blocks / time).
/// Takes some number of minutes for measurement. Returns the throughput in bytes per second.
/// Block size is measured in bytes with Rust's `std::mem::size_of_val()`, using the first block in the measurement period.
///
/// NOTE: for a closer estimate, we could measure the size of every block in the period. But for now, this is probably close enough.
#[allow(non_snake_case)]
pub async fn get_blockchain_throughput(
    request_builder: reqwest::RequestBuilder,
    time_in_minutes: u32,
) -> u64 {
    // our measurement periods are in minutes, but currently std::time::Duration doesn't have a `from_minutes` function.
    let time_in_seconds: u64 = (time_in_minutes as u64) * 60;
    let period = time::Duration::from_secs(time_in_seconds);

    let request_config = RequestConfig::ReqBldr(request_builder);

    let block_height_start =
        blockchain_generic::call_getBlockHeight(request_config.try_clone().unwrap(), None).await;
    thread::sleep(period);
    let block_height_end =
        blockchain_generic::call_getBlockHeight(request_config.try_clone().unwrap(), None).await;

    // we want the size of a representative block. so if the one we check "was missing or skipped in long term storage", then we will try the next one.
    let example_block: Block = {
        let mut parsed_block: Option<Block> = None;
        let blocks_in_period = block_height_start..block_height_end;
        // Go through all the blocks until we find one that is valid, then break.
        for i in blocks_in_period {
            let block_in_period: block_response_types::BlockResponse =
                blockchain_generic::call_getBlock(request_config.try_clone().unwrap(), i, None)
                    .await;

            if let Some(_parsed_block) = block_in_period.result {
                parsed_block = Some(_parsed_block);
                break;
            }
        }
        // At this point, we either broke the loop because we found a valid block or we went through all the blocks.
        // If we find that we do not have a valid block still, panic.
        match parsed_block {
            Some(b) => b,
            None => panic!("FATAL: all blocks in the measurement period were `None`. Check the deserialization.")
        }
    };

    // Now we estimate the amount of data per second by assuming the valid block we found is representative of an average
    // and multiplying it by the number of blocks viewed, divided by the time in seconds.
    let serialized_block = serde_json::to_string(&example_block).unwrap();
    let block_size: u64 = size_of_val(serialized_block.as_bytes()) as u64;
    let num_blocks = block_height_end - block_height_start;

    block_size * num_blocks / time_in_seconds
}
