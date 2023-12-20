/*

transforms the block data into a block record, and a sequence of block reward records.

uses protocol buffers that match the bigquery schemas for these tables.

*/
use crate::solana_config::proto_codegen::etl_block::EtlBlock;

#[cfg(feature = "STRING_TIMESTAMP")]
use {
    crate::solana_config::proto_codegen::records_string_timestamp::{
        BlockRecord, BlockRewardRecord,
    },
    chrono::{NaiveDateTime, TimeZone, Utc},
};

#[cfg(feature = "INT_TIMESTAMP")]
use crate::solana_config::proto_codegen::records_int_timestamp::{BlockRecord, BlockRewardRecord};

pub fn transform_to_block_record(etl_block: &EtlBlock) -> BlockRecord {
    match &etl_block.block {
        None => panic!("Unexpected input: etl_block.block should never be None"),
        Some(block) => {
            #[cfg(feature = "STRING_TIMESTAMP")]
            let block_timestamp = block.block_time.as_ref().map(|bt| {
                Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(bt.timestamp, 0).unwrap())
                    .to_rfc3339()
            });
            #[cfg(feature = "INT_TIMESTAMP")]
            let block_timestamp = block
                .block_time
                .to_owned()
                .map(|bt| bt.timestamp * 1_000_000);

            let height = block.block_height.as_ref().map(|bh| bh.block_height as i64);
            BlockRecord {
                slot: Some(etl_block.slot as i64),
                height,
                block_hash: Some(block.blockhash.to_owned()),
                previous_block_hash: Some(block.previous_blockhash.to_owned()),
                block_timestamp,
                transaction_count: Some(block.transaction_count as i64),
                leader_reward: Some(block.leader_reward as i64),
                leader: Some(block.leader.to_owned()),
            }
        }
    }
}

pub fn transform_to_block_reward_records(etl_block: &EtlBlock) -> Vec<BlockRewardRecord> {
    let table_context = etl_block.table_context.to_owned().unwrap();
    #[cfg(feature = "STRING_TIMESTAMP")]
    let block_timestamp = table_context.block_timestamp.as_ref().map(|bt| {
        Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(bt.timestamp, 0).unwrap())
            .to_rfc3339()
    });
    #[cfg(feature = "INT_TIMESTAMP")]
    let block_timestamp = table_context
        .block_timestamp
        .to_owned()
        .map(|bt| bt.timestamp * 1_000_000);
    etl_block
        .block_rewards
        .iter()
        .map(|reward| BlockRewardRecord {
            block_slot: Some(etl_block.slot as i64),
            block_hash: Some(table_context.block_hash.to_owned()),
            block_timestamp: block_timestamp.to_owned(),
            commission: reward.commission.parse().ok(),
            lamports: Some(reward.lamports as i64),
            post_balance: Some(reward.post_balance),
            pubkey: Some(reward.pubkey.to_owned()),
            reward_type: match reward.reward_type {
                0 => Some(String::from("Unspecified")),
                1 => Some(String::from("Fee")),
                2 => Some(String::from("Rent")),
                3 => Some(String::from("Staking")),
                4 => Some(String::from("Voting")),
                _ => panic!("Unexpected reward type. Terminating..."),
            },
        })
        .collect()
}
