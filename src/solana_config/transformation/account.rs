use crate::solana_config::proto_codegen::etl_block::EtlBlock;

#[cfg(feature = "STRING_TIMESTAMP")]
use {
    crate::solana_config::proto_codegen::records_string_timestamp::{
        AccountRecord, AuthorizedVoterRecord, CreatorRecord, DataRecord, EpochCreditRecord,
        PriorVoterRecord, TimestampRecord, TokenRecord, VoteRecord,
    },
    chrono::{NaiveDateTime, TimeZone, Utc},
};

#[cfg(feature = "INT_TIMESTAMP")]
use crate::solana_config::proto_codegen::records_int_timestamp::{
    AccountRecord, AuthorizedVoterRecord, CreatorRecord, DataRecord, EpochCreditRecord,
    PriorVoterRecord, TimestampRecord, TokenRecord, VoteRecord,
};

pub fn transform_to_account_and_token_records(
    etl_block: &EtlBlock,
) -> (Vec<AccountRecord>, Vec<TokenRecord>) {
    let table_context = etl_block.table_context.to_owned().unwrap();

    let mut account_records: Vec<AccountRecord> = Vec::with_capacity(etl_block.accounts.len());
    let mut token_records: Vec<TokenRecord> = Vec::with_capacity(
        etl_block
            .accounts
            .iter()
            .map(|account| account.tokens.len())
            .sum(),
    );

    for accounts_and_tokens in &etl_block.accounts {
        let tx_signature = accounts_and_tokens.tx_signature.to_owned();
        let block_slot = Some(etl_block.slot as i64);
        let block_hash = Some(table_context.block_hash.to_owned());

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

        for account_data in &accounts_and_tokens.accounts {
            #[cfg(feature = "STRING_TIMESTAMP")]
            let retrieval_timestamp = account_data.retrieval_timestamp.as_ref().map(|bt| {
                Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(bt.timestamp, 0).unwrap())
                    .to_rfc3339()
            });
            #[cfg(feature = "INT_TIMESTAMP")]
            let retrieval_timestamp = account_data
                .retrieval_timestamp
                .to_owned()
                .map(|bt| bt.timestamp * 1_000_000);

            let account_record = AccountRecord {
                block_slot,
                block_hash: block_hash.to_owned(),
                block_timestamp: block_timestamp.to_owned(),
                tx_signature: tx_signature.to_owned(),
                retrieval_timestamp,
                pubkey: Some(account_data.pubkey.to_owned()),
                executable: Some(account_data.executable),
                lamports: Some(account_data.lamports),
                owner: account_data.owner.to_owned(),
                rent_epoch: Some(account_data.rent_epoch as i64),
                program: account_data.program.to_owned(),
                space: account_data.space,
                account_type: account_data.account_type.to_owned(),
                is_native: account_data.is_native,
                mint: account_data.mint.to_owned(),
                state: account_data.state.to_owned(),
                token_amount: account_data
                    .token_amount
                    .as_ref()
                    .and_then(|d| d.parse::<u64>().ok()),
                token_amount_decimals: account_data.token_amount_decimals,
                program_data: account_data.program_data.to_owned(),
                authorized_voters: account_data
                    .authorized_voters
                    .iter()
                    .map(|av| AuthorizedVoterRecord {
                        authorized_voter: Some(av.authorized_voter.to_owned()),
                        epoch: Some(av.epoch),
                    })
                    .collect(),
                authorized_withdrawer: account_data.authorized_withdrawer.to_owned(),
                prior_voters: account_data
                    .prior_voters
                    .iter()
                    .map(|pv| PriorVoterRecord {
                        authorized_pubkey: Some(pv.authorized_pubkey.to_owned()),
                        epoch_of_last_authorized_switch: Some(pv.epoch_of_last_authorized_switch),
                        target_epoch: Some(pv.target_epoch),
                    })
                    .collect(),
                node_pubkey: account_data.node_pubkey.to_owned(),
                commission: account_data.commission,
                epoch_credits: account_data
                    .epoch_credits
                    .iter()
                    .map(|ec| EpochCreditRecord {
                        credits: Some(ec.credits.to_owned()),
                        epoch: Some(ec.epoch),
                        previous_credits: Some(ec.previous_credits.to_owned()),
                    })
                    .collect(),
                votes: account_data
                    .votes
                    .iter()
                    .map(|v| VoteRecord {
                        slot: Some(v.slot),
                        confirmation_count: Some(v.confirmation_count),
                    })
                    .collect(),
                root_slot: account_data.root_slot,
                last_timestamp: account_data
                    .last_timestamp
                    .iter()
                    .map(|lt| {
                        #[cfg(feature = "STRING_TIMESTAMP")]
                        let timestamp = Utc
                            .from_utc_datetime(
                                &NaiveDateTime::from_timestamp_opt(lt.timestamp, 0).unwrap(),
                            )
                            .to_rfc3339();
                        #[cfg(feature = "INT_TIMESTAMP")]
                        let timestamp = lt.timestamp * 1_000_000;
                        TimestampRecord {
                            slot: Some(lt.slot),
                            timestamp: Some(timestamp),
                        }
                    })
                    .collect(),
                data: account_data
                    .data
                    .iter()
                    .map(|d| DataRecord {
                        raw: Some(d.raw.to_owned()),
                        encoding: Some(d.encoding.to_owned()),
                    })
                    .collect(),
            };
            account_records.push(account_record);
        }

        for token_data in &accounts_and_tokens.tokens {
            #[cfg(feature = "STRING_TIMESTAMP")]
            let retrieval_timestamp = token_data.retrieval_timestamp.as_ref().map(|bt| {
                Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(bt.timestamp, 0).unwrap())
                    .to_rfc3339()
            });
            #[cfg(feature = "INT_TIMESTAMP")]
            let retrieval_timestamp = token_data
                .retrieval_timestamp
                .to_owned()
                .map(|bt| bt.timestamp * 1_000_000);

            let token_record = TokenRecord {
                block_slot,
                block_hash: block_hash.to_owned(),
                block_timestamp: block_timestamp.to_owned(),
                tx_signature: tx_signature.to_owned(),
                retrieval_timestamp,
                is_nft: Some(token_data.is_nft),
                mint: Some(token_data.mint.to_owned()),
                update_authority: Some(token_data.update_authority.to_owned()),
                name: Some(token_data.name.to_owned()),
                symbol: Some(token_data.symbol.to_owned()),
                uri: Some(token_data.uri.to_owned()),
                seller_fee_basis_points: Some(token_data.seller_fee_basis_points),

                creators: token_data
                    .creators
                    .iter()
                    .map(|c| CreatorRecord {
                        address: Some(c.address.to_owned()),
                        verified: Some(c.verified),
                        share: Some(c.share as i64),
                    })
                    .collect(),
                primary_sale_happened: Some(token_data.primary_sale_happened),
                is_mutable: Some(token_data.is_mutable),
            };

            token_records.push(token_record);
        }
    }

    (account_records, token_records)
}
