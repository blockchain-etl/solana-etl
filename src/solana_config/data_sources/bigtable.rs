//! This module contains the functions required to communicate with the
//! [Solana Foundation's BigTable instance](https://docs.solana.com/implemented-proposals/rpc-transaction-history#table-schema).  
use crate::solana_config::constants;
use solana_transaction_status::{
    BlockEncodingOptions, ConfirmedBlock, TransactionDetails, UiConfirmedBlock,
    UiTransactionEncoding,
};

use log::{error, info, warn};

use solana_storage_bigtable::{CredentialType, LedgerStorage};

/// Result from BigTable, should expect either data or an error.
pub type BigTableResult<T> = std::result::Result<T, solana_storage_bigtable::Error>;

/// Connects to the BigTable.  
///
/// Note: Must have BigTable credentials set up in the .env file, otherwise this will fail.
pub async fn connect_to_bigtable() -> Result<LedgerStorage, solana_storage_bigtable::Error> {
    let bigtable_cred = dotenvy::var("BIGTABLE_CRED")
        .expect("BIGTABLE_CRED should exist in .env file")
        .parse::<String>()
        .unwrap();

    let bt_config = solana_storage_bigtable::LedgerStorageConfig {
        credential_type: CredentialType::Filepath(Some(bigtable_cred)),
        ..Default::default()
    };

    info!("Connecting to the BigTable...");

    solana_storage_bigtable::LedgerStorage::new_with_config(bt_config).await
}

/// Returns the block data at a particular slot if available, otherwise returns missing block error.
#[inline]
pub async fn call_get_confirmed_block(
    bigtable: &solana_storage_bigtable::LedgerStorage,
    confirmed_slot: u64,
) -> BigTableResult<ConfirmedBlock> {
    info!(
        "Requesting a block from the BigTable at slot {}...",
        confirmed_slot
    );

    let mut bt = bigtable.clone();
    match bt.get_confirmed_block(confirmed_slot).await {
        Ok(blocks) => Ok(blocks),
        Err(e) => {
            // blocks are expected to be missing from some slots
            if let solana_storage_bigtable::Error::BlockNotFound(_) = e {
                warn!("{:?}", e);
                return Err(e);
            } else {
                error!(
                    "get_confirmed_block failed for block {}: {:?}",
                    confirmed_slot, e
                );
            }
            loop {
                match bt.get_confirmed_block(confirmed_slot).await {
                    Ok(block) => {
                        info!(
                            "get_confirmed_block succeeded after reattempt for slot: {}",
                            confirmed_slot
                        );
                        return Ok(block);
                    }
                    Err(e_reattempted) => {
                        if let solana_storage_bigtable::Error::BlockNotFound(_) = e_reattempted {
                            return Err(e_reattempted);
                        }
                        error!(
                            "get_confirmed_block failed on reattempt for slot {}: {}",
                            confirmed_slot, e_reattempted
                        );
                        // attempt to re-connect to the bigtable if there's a connection error
                        let attempted_reconnection = connect_to_bigtable().await;
                        match attempted_reconnection {
                            Ok(connection) => bt = connection,
                            Err(e_connection) => {
                                error!("Failed to re-connect to bigtable: {:?}", e_connection)
                            }
                        }
                        continue;
                    }
                }
            }
        }
    }
}

/// parses the block data that is returned from the bigtable
#[inline]
pub fn parse_block(unparsed_block: ConfirmedBlock) -> UiConfirmedBlock {
    let encoding_options = BlockEncodingOptions {
        transaction_details: TransactionDetails::Full,
        show_rewards: true,
        max_supported_transaction_version: Some(constants::TRANSACTION_VERSION),
    };
    unparsed_block
        .encode_with_options(UiTransactionEncoding::JsonParsed, encoding_options)
        .unwrap()
}
