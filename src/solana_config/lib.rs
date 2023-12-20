//! This library provides all the functionality required to index
//! the Solana network.
use std::{
    error::Error,
    fs::{remove_file, File},
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time,
};

use futures::future::join_all;
use log::{debug, error, info};
use tokio::time::sleep;

use super::accounts::{self, call_getMultipleAccounts, KeyedTimestampedAccounts};
use super::proto_codegen::{
    account_info as solana_account_protobuf, account_info::Token, confirmed_block::UnixTimestamp,
    etl_block,
};
use super::proto_conversions::{account::PackagedAccount, block::parsed_block_to_proto};
use super::tokens;
use super::transactions::{self, TransactionAccounts};
use super::types::account_response_types::AccountDataEnumResponse;

use crate as blockchain_generic;
use blockchain_generic::{
    constants::RANGE_SIZE, metrics::Metrics, request, source::config::RequestConfig,
};

#[cfg(feature = "SOLANA_BIGTABLE")]
use super::data_sources::bigtable;

#[cfg(not(feature = "SOLANA_BIGTABLE"))]
use super::data_sources::json_rpc::get_recent_block;

#[cfg(feature = "SEPARATE_PUBLISHERS")]
use blockchain_generic::solana_config::transformation;

/// Given KeyedTimestampedAccounts and a request builder, ,
async fn get_accounts_and_tokens_from_pubkeys(
    keyed_accounts: KeyedTimestampedAccounts,
    request_config: RequestConfig,
    metrics: Option<Metrics>,
) -> (Vec<PackagedAccount>, Vec<Token>) {
    // each value is a (pubkey, is_nft)
    let token_accounts = tokens::get_tokens_from_mint_accounts(keyed_accounts.clone());
    let accounts_packaged = accounts::package_accounts(keyed_accounts);

    let mut tokens_packaged = Vec::new();
    if !token_accounts.is_empty() {
        info!("Requesting token data");
        // Extract pubkeys from token account vector
        let token_pubkeys: Vec<String> = token_accounts
            .clone()
            .into_iter()
            .map(|token| token.mint)
            .collect();

        // Extract the request builder
        let request_builder = request_config.try_clone().unwrap();

        // Get account data from pubkeys
        let token_accounts_opt =
            call_getMultipleAccounts(request_builder, token_pubkeys, metrics).await;

        // If there are tokens, start extracting the values and placing them in the tokens_packaged vector
        if let Some(token_accounts_response) = token_accounts_opt {
            // Extract data and add it to the token_data vector
            let token_retrieval_timestamp = token_accounts_response.timestamp;
            for (i, account_info) in token_accounts_response
                .accounts
                .value
                .into_iter()
                .enumerate()
            {
                // Place the token data into the tokens_packaged vector
                if let Some(a) = account_info {
                    match &a.data {
                        AccountDataEnumResponse::Array(strings) => {
                            if strings[0] == "AA==" {
                                info!("Empty token data. Skipping...");
                                continue;
                            }
                        }
                        _ => panic!("Unexpected token data shape"),
                    }
                    let is_nft = token_accounts[i].is_nft;

                    let token_data =
                        tokens::unpack_token_account(a.data, is_nft, token_retrieval_timestamp);
                    tokens_packaged.push(token_data);
                }
            }
        };
    } else {
        info!("No tokens to request.");
    }

    (accounts_packaged, tokens_packaged)
}

/// this function is expected to be run by multiple threads.
/// each instance pulls a slot from the concurrent queue, makes the block request, processes the response, and sends it to the pub/sub middleware for transformation and insertion.
async fn process_block_queue_stream(
    _bigtable: Option<solana_storage_bigtable::LedgerStorage>,
    publisher: blockchain_generic::output::publish::StreamPublisher,
    thread_queue: async_channel::Receiver<u64>,
    metrics: Option<Metrics>,
) -> Result<(), Box<dyn Error>> {
    // NOTE: reqwest clients are not thread-safe, so we create a new one here
    let request_builder = {
        let endpoint = dotenvy::var("ENDPOINT")
            .expect("ENDPOINT should exist in .env file")
            .parse::<String>()
            .unwrap();
        let connection_timeout =
            std::time::Duration::from_secs(blockchain_generic::constants::CONNECTION_TIMEOUT);
        let client_builder = reqwest::Client::builder().connect_timeout(connection_timeout);

        let headers = request::get_headers();
        let client = client_builder.build().unwrap();
        client.post(endpoint).headers(headers)
    };
    // Creates the request config
    let request_config: RequestConfig = RequestConfig::ReqBldr(request_builder);

    let indexed_blocks_dir = Path::new("./indexed_blocks/");

    debug!("starting thread...");
    // Extract the publisher(s)
    #[cfg(feature = "SINGLE_PUBLISHER")]
    let all_publisher = publisher.all;
    #[cfg(feature = "SEPARATE_PUBLISHERS")]
    let (
        blocks_publisher,
        block_rewards_publisher,
        transactions_publisher,
        instructions_publisher,
        token_transfers_publisher,
        tokens_publisher,
        accounts_publisher,
    ) = (
        publisher.blocks,
        publisher.block_rewards,
        publisher.transactions,
        publisher.instructions,
        publisher.token_transfers,
        publisher.tokens,
        publisher.accounts,
    );

    // Creates the channel(s) for this thread (Rabbitmq-Classic specific)
    // RabbitMQ Classic requires that we construct "channel" instances in the thread
    // we intend to publish in.  Because of this, we need to recreate with the channel
    // here.
    #[cfg(feature = "SINGLE_PUBLISHER")]
    #[cfg(feature = "RABBITMQ_CLASSIC")]
    let all_publisher = all_publisher.with_channel().await;
    #[cfg(feature = "SEPARATE_PUBLISHERS")]
    #[cfg(feature = "RABBITMQ_CLASSIC")]
    let (
        blocks_publisher,
        block_rewards_publisher,
        transactions_publisher,
        instructions_publisher,
        token_transfers_publisher,
        tokens_publisher,
        accounts_publisher,
    ) = (
        blocks_publisher.with_channel().await,
        block_rewards_publisher.with_channel().await,
        transactions_publisher.with_channel().await,
        instructions_publisher.with_channel().await,
        token_transfers_publisher.with_channel().await,
        tokens_publisher.with_channel().await,
        accounts_publisher.with_channel().await,
    );

    #[cfg(feature = "SOLANA_BIGTABLE")]
    let bigtable = _bigtable.unwrap();

    /*
        this loop contains much of core indexing logic:
            each slot is used to request a block,
            then the account pubkeys are used to request accounts data.
            account data is also requested for any token mints.
            all of the above is packaged into a pub/sub message for easy transformation and insertion.
    */
    let mut prev_slot = None;
    loop {
        let slot: u64 = match thread_queue.recv().await {
            Ok(rec_i) => rec_i,
            Err(_) => {
                info!("Task queue closed. Ending thread...");
                return Ok(());
            }
        };

        // Delete the previous indexing log, and create a new one for the current slot.
        // NOTE: we do this before actually processing the block, in case the slot doesn't have a block and this iteration gets skipped.
        {
            if let Some(prev) = prev_slot {
                let file_path = indexed_blocks_dir.join(format!("{}", prev));
                match remove_file(file_path) {
                    Ok(_) => info!("Successfully deleted the file"),
                    Err(e) => panic!("FATAL: failed to delete the file: {:?}", e),
                }
            }

            let file_path = indexed_blocks_dir.join(format!("{}", slot));

            match File::create(file_path) {
                Ok(_) => info!("Successfully created the file"),
                Err(e) => panic!("FATAL: failed to create the file: {:?}", e),
            }
            prev_slot = Some(slot);
        }

        info!("received block task: {}", slot);

        #[cfg(feature = "SOLANA_BIGTABLE")]
        let parsed_block = {
            let confirmed_block = bigtable::call_get_confirmed_block(&bigtable, slot).await;
            match confirmed_block {
                Err(_) => continue, // only happens when there is no block at the slot
                Ok(block) => bigtable::parse_block(block),
            }
        };

        #[cfg(not(feature = "SOLANA_BIGTABLE"))]
        let parsed_block = {
            let result =
                get_recent_block(request_config.try_clone().unwrap(), slot, metrics.clone()).await;
            match result {
                None => continue, // only happens for skipped slots. safe to move past.
                Some(b) => b,
            }
        };

        let block_hash = parsed_block.blockhash.clone();
        let previous_block_hash = parsed_block.previous_blockhash.clone();
        let block_timestamp = parsed_block.block_time;

        // get the account public keys so that we can call the RPC method getMultipleAccounts()
        let transactions = transactions::get_transactions_from_block(&parsed_block);
        let all_account_pubkeys = transactions::get_pubkeys_from_transactions(transactions);
        let mut packed_accounts = Vec::new();
        for TransactionAccounts {
            tx_signature,
            accounts,
        } in all_account_pubkeys.into_iter()
        {
            let timestamped_accounts_data = call_getMultipleAccounts(
                request_config.try_clone().unwrap(),
                accounts.clone(),
                metrics.clone(),
            )
            .await;

            match timestamped_accounts_data {
                Some(accounts_inner) => {
                    let keyed_accounts =
                        KeyedTimestampedAccounts::from_keys_and_accounts(accounts, accounts_inner);

                    // this is the full account data and full token data associated with the current transaction
                    let (accounts, tokens) = get_accounts_and_tokens_from_pubkeys(
                        keyed_accounts,
                        request_config.try_clone().unwrap(),
                        metrics.clone(),
                    )
                    .await;

                    packed_accounts.push((tx_signature, accounts, tokens)); // the timestamps are stored within the accounts
                }
                None => info!("No accounts found"),
            }
        }

        let all_accounts_and_tokens = packed_accounts
            .into_iter()
            .map(|(tx_signature, accounts, tokens)| {
                solana_account_protobuf::AccountInfo::new(tx_signature, accounts, tokens)
            })
            .collect();

        let (block_metadata, block_rewards, transactions) = parsed_block_to_proto(parsed_block);
        let table_context = etl_block::TableContext {
            block_hash,
            previous_block_hash,
            block_timestamp: block_timestamp.map(|ts| UnixTimestamp { timestamp: ts }),
        };

        let packed_block: etl_block::EtlBlock = etl_block::EtlBlock {
            slot,
            block: Some(block_metadata),
            block_rewards,
            transactions,
            accounts: all_accounts_and_tokens,
            table_context: Some(table_context),
        };

        #[cfg(feature = "SINGLE_PUBLISHER")]
        {
            let serialized_block: Vec<u8> = packed_block.encode_to_vec();
            all_publisher.publish(serialized_block).await;
            info!("PUBLISHED BLOCK: {}", slot);
        }

        #[cfg(feature = "SEPARATE_PUBLISHERS")]
        {
            /// Publishes the records
            #[allow(unused_variables)]
            async fn publish_records<T>(
                publisher: &blockchain_generic::output::publish::StreamPublisherConnection,
                records: Vec<T>,
                name: Option<&str>,
            ) where
                T: prost::Message,
                T: serde::Serialize,
            {
                #[cfg(feature = "PUBLISH_WITH_NAME")]
                if let Some(output_name) = name {
                    #[cfg(feature = "JSON")]
                    for (i, record) in records.into_iter().enumerate() {
                        publisher
                            .publish(&format!("{}_{}", output_name, i), record)
                            .await;
                    }
                    #[cfg(feature = "JSONL")]
                    publisher.publish_batch(output_name, records).await;
                }

                #[cfg(not(feature = "PUBLISH_WITH_NAME"))]
                {
                    #[cfg(feature = "GOOGLE_PUBSUB")]
                    publisher.publish_batch(records).await;

                    #[cfg(not(feature = "GOOGLE_PUBSUB"))]
                    for record in records {
                        publisher.publish(record).await;
                    }
                }
            }

            // Here we unpack the data from the `packed_block` through the transform functions then
            // serialize the data to be sent to the respective publisher.

            // Block record
            let block_record = transformation::block::transform_to_block_record(&packed_block);

            // Block Rewards records
            let block_reward_records =
                transformation::block::transform_to_block_reward_records(&packed_block);

            // Transformation for Transactions, Instructions & token transfers.
            let (transaction_records, instruction_records, token_transfer_records) =
                transformation::transaction::transform_to_transaction_records(&packed_block);

            let (account_records, token_records) =
                transformation::account::transform_to_account_and_token_records(&packed_block);

            // Publish
            #[cfg(feature = "PUBLISH_WITH_NAME")]
            {
                let records_name = slot.to_string();

                // Block
                blocks_publisher.publish(&records_name, block_record).await;

                // Block Rewards
                publish_records(
                    &block_rewards_publisher,
                    block_reward_records,
                    Some(&records_name),
                )
                .await;

                // Transactions
                publish_records(
                    &transactions_publisher,
                    transaction_records,
                    Some(&records_name),
                )
                .await;

                // Instructions
                publish_records(
                    &instructions_publisher,
                    instruction_records,
                    Some(&records_name),
                )
                .await;

                // Token Transfers
                publish_records(
                    &token_transfers_publisher,
                    token_transfer_records,
                    Some(&records_name),
                )
                .await;

                // Tokens
                publish_records(&tokens_publisher, token_records, Some(&records_name)).await;

                // Accounts
                publish_records(&accounts_publisher, account_records, Some(&records_name)).await;
            }
            #[cfg(not(feature = "PUBLISH_WITH_NAME"))]
            {
                // Block
                blocks_publisher.publish(block_record).await;

                // Block Rewards
                publish_records(&block_rewards_publisher, block_reward_records, None).await;

                // Transactions
                publish_records(&transactions_publisher, transaction_records, None).await;

                // Instructions
                publish_records(&instructions_publisher, instruction_records, None).await;

                // Token Transfers
                publish_records(&token_transfers_publisher, token_transfer_records, None).await;

                // Tokens
                publish_records(&tokens_publisher, token_records, None).await;

                // Accounts
                publish_records(&accounts_publisher, account_records, None).await;
            }
        }
        info!("Sent block {} to stream queue", slot);
    }
}

/// this function is run by the main program thread, and is part of the core logic.
/// slot values are sent to a concurrent queue for processing by multiple worker threads.
///     - the worker threads are spawned from here.
///        - processed slots are logged here, and used for picking up from a shutdown or system crash.
#[allow(non_snake_case, clippy::too_many_arguments)]
pub async fn extract<I>(
    range: I,
    request_builder: reqwest::RequestBuilder,
    bigtable: Option<solana_storage_bigtable::LedgerStorage>,
    thread_count: usize,
    publisher: blockchain_generic::output::publish::StreamPublisher,
    metrics: Option<Metrics>,
) -> Result<(), Box<dyn Error>>
where
    I: Iterator<Item = u64>,
{
    // Save the request builder as a request config enum
    let request_config = RequestConfig::ReqBldr(request_builder);
    info!("Starting the indexer...");

    let interrupter = Arc::new(AtomicBool::new(true));
    let (block_sender, block_receiver) = async_channel::unbounded::<u64>();

    let threads = {
        let mut threads: Vec<tokio::task::JoinHandle<()>> = Vec::with_capacity(thread_count);
        for _t in 0..thread_count {
            let cur_bigtable = bigtable.clone();
            let cur_publisher = publisher.clone();
            let cur_block_receiver = block_receiver.clone();
            let cur_metrics = metrics.clone();
            threads.push(tokio::task::spawn(async move {
                process_block_queue_stream(
                    cur_bigtable,
                    cur_publisher,
                    cur_block_receiver,
                    cur_metrics,
                )
                .await
                .unwrap()
            }));
        }
        threads
    };

    info!("Press 'CRTL-C' to terminate...");
    let r = interrupter.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    let mut range = range;
    while interrupter.load(Ordering::SeqCst) {
        // Iterates through all confirmed blocks returned by the function
        let mut latest_slot =
            blockchain_generic::call_getSlot(request_config.try_clone().unwrap(), metrics.clone())
                .await;

        let subrange = range.by_ref().take(RANGE_SIZE as usize);

        // check that the iterator is not empty
        let mut peekable_subrange = subrange.peekable();
        if peekable_subrange.peek().is_none() {
            break;
        }

        // send each of the slot values to the concurrent task queue, ensuring that they don't exceed the node's maximum slot
        for i in peekable_subrange {
            while i > latest_slot {
                latest_slot = blockchain_generic::call_getSlot(
                    request_config.try_clone().unwrap(),
                    metrics.clone(),
                )
                .await;
            }

            info!("sending block task: {}", i);
            block_sender
                .send(i)
                .await
                .expect("block queue has not been disconnected");
        }
        let seconds = time::Duration::from_secs(1);
        sleep(seconds).await;
    }
    info!("Shutting down...");

    block_sender.close();

    let worker_results = join_all(threads).await;

    for result in worker_results {
        if let Err(e) = result {
            error!("Task failed to terminate: {:?}", e);
        }
    }

    Ok(())
}
