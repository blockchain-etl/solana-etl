//! This module has structs and implementations that assist in the process of
//! converting the raw accounts data into a structure compliant with
//! the protocol buffers interface.
#![allow(clippy::too_many_arguments)]

use crate::solana_config::{
    proto_codegen::{self, account_info as solana_account_protobuf},
    types::account_response_types::{
        AccountInfoAuthorizedEpochCreditsResponse, AccountInfoAuthorizedPriorVotersResponse,
        AccountInfoAuthorizedVotersResponse, AccountInfoLastTimestampResponse,
        AccountInfoVotesResponse,
    },
};

impl solana_account_protobuf::AccountInfo {
    pub fn new(
        tx_signature: String,
        accounts_packaged: Vec<PackagedAccount>,
        tokens_packaged: Vec<solana_account_protobuf::Token>,
    ) -> solana_account_protobuf::AccountInfo {
        let accounts_proto = accounts_packaged
            .into_iter()
            .map(solana_account_protobuf::Account::from)
            .collect();

        solana_account_protobuf::AccountInfo {
            tx_signature: Some(tx_signature),
            accounts: accounts_proto,
            tokens: tokens_packaged,
        }
    }
}

/// Packaged Account is a struct that contains all account data
/// in a singular struct, with less nested structures than many of the
/// alternative structs.
#[derive(Debug)]
pub struct PackagedAccount {
    timestamp: u64,
    pubkey: String,
    data: Option<EncodedData>,
    executable: bool,
    lamports: u64,
    owner: Option<String>,
    rent_epoch: u64,
    space: Option<i64>,
    program: Option<String>,
    program_data: Option<String>,
    account_type: Option<String>,
    is_native: Option<bool>,
    mint: Option<String>,
    state: Option<String>,
    token_amount: Option<String>,
    token_amount_decimals: Option<i64>,
    authorized_voters: Option<Vec<AccountInfoAuthorizedVotersResponse>>,
    authorized_withdrawer: Option<String>,
    prior_voters: Option<Vec<AccountInfoAuthorizedPriorVotersResponse>>,
    node_pubkey: Option<String>,
    commission: Option<i64>,
    epoch_credits: Option<Vec<AccountInfoAuthorizedEpochCreditsResponse>>,
    votes: Option<Vec<AccountInfoVotesResponse>>,
    root_slot: Option<i64>,
    last_timestamp: Option<AccountInfoLastTimestampResponse>,
    mint_authority: Option<String>,
    supply: Option<String>,
}

impl From<PackagedAccount> for solana_account_protobuf::Account {
    fn from(account_packaged: PackagedAccount) -> Self {
        solana_account_protobuf::Account {
            retrieval_timestamp: Some(proto_codegen::confirmed_block::UnixTimestamp {
                timestamp: account_packaged.timestamp as i64,
            }),
            pubkey: account_packaged.pubkey,
            executable: account_packaged.executable,
            lamports: account_packaged.lamports,
            owner: account_packaged.owner,
            rent_epoch: account_packaged.rent_epoch,
            space: account_packaged.space,
            program: account_packaged.program,
            program_data: account_packaged.program_data,
            account_type: account_packaged.account_type,
            is_native: account_packaged.is_native,
            mint: account_packaged.mint,
            state: account_packaged.state,
            token_amount: account_packaged.token_amount,
            token_amount_decimals: account_packaged.token_amount_decimals,
            authorized_voters: match account_packaged.authorized_voters {
                None => Vec::new(),
                Some(v) => v
                    .into_iter()
                    .map(|av| solana_account_protobuf::AuthorizedVoter {
                        authorized_voter: av.authorizedVoter,
                        epoch: av.epoch,
                    })
                    .collect(),
            },
            authorized_withdrawer: account_packaged.authorized_withdrawer,
            prior_voters: match account_packaged.prior_voters {
                None => Vec::new(),
                Some(v) => v
                    .into_iter()
                    .map(|apv| solana_account_protobuf::PriorVoters {
                        authorized_pubkey: apv.authorizedPubkey,
                        epoch_of_last_authorized_switch: apv.epochOfLastAuthorizedSwitch,
                        target_epoch: apv.targetEpoch,
                    })
                    .collect(),
            },
            node_pubkey: account_packaged.node_pubkey,
            commission: account_packaged.commission,
            epoch_credits: match account_packaged.epoch_credits {
                None => Vec::new(),
                Some(v) => v
                    .into_iter()
                    .map(|apc| solana_account_protobuf::EpochCredit {
                        credits: apc.credits,
                        epoch: apc.epoch,
                        previous_credits: apc.previousCredits,
                    })
                    .collect(),
            },
            votes: match account_packaged.votes {
                None => Vec::new(),
                Some(v) => v
                    .into_iter()
                    .map(|vv| solana_account_protobuf::Votes {
                        slot: vv.slot,
                        confirmation_count: vv.confirmationCount,
                    })
                    .collect(),
            },
            root_slot: account_packaged.root_slot,
            last_timestamp: account_packaged.last_timestamp.map(|lt| {
                solana_account_protobuf::LastTimestamp {
                    slot: lt.slot,
                    timestamp: lt.timestamp,
                }
            }),
            mint_authority: account_packaged.mint_authority,
            supply: account_packaged.supply,
            data: account_packaged
                .data
                .map(|data| solana_account_protobuf::EncodedData {
                    raw: data.raw,
                    encoding: data.encoding,
                }),
        }
    }
}

impl PackagedAccount {
    pub fn new_account_type(
        timestamp: u64,
        pubkey: String,
        executable: bool,
        lamports: u64,
        owner: Option<String>,
        rent_epoch: u64,
        space: i64,
        program: String,
        mint: Option<String>,
        token_amount: Option<String>,
        token_amount_decimals: Option<i64>,
        is_native: Option<bool>,
        state: Option<String>,
    ) -> PackagedAccount {
        PackagedAccount {
            timestamp,
            pubkey,
            executable,
            lamports,
            owner,
            rent_epoch,
            space: Some(space),
            program: Some(program),
            account_type: Some(String::from("account")),
            authorized_voters: None,
            authorized_withdrawer: None,
            prior_voters: None,
            node_pubkey: None,
            commission: None,
            epoch_credits: None,
            votes: None,
            root_slot: None,
            last_timestamp: None,
            mint,
            program_data: None,
            token_amount,
            token_amount_decimals,
            is_native,
            data: None,
            state,
            mint_authority: None,
            supply: None,
        }
    }

    pub fn new_mint_type(
        timestamp: u64,
        pubkey: String,
        executable: bool,
        lamports: u64,
        owner: Option<String>,
        rent_epoch: u64,
        space: i64,
        program: String,
        token_amount_decimals: Option<i64>,
        mint_authority: Option<String>,
        supply: Option<String>,
    ) -> PackagedAccount {
        PackagedAccount {
            timestamp,
            pubkey,
            executable,
            lamports,
            owner,
            rent_epoch,
            space: Some(space),
            program: Some(program),
            account_type: Some(String::from("mint")),
            authorized_voters: None,
            authorized_withdrawer: None,
            prior_voters: None,
            node_pubkey: None,
            commission: None,
            epoch_credits: None,
            votes: None,
            root_slot: None,
            last_timestamp: None,
            mint: None,
            program_data: None,
            token_amount: None,
            token_amount_decimals,
            is_native: None,
            data: None,
            state: None,
            mint_authority,
            supply,
        }
    }

    pub fn new_program_type(
        timestamp: u64,
        pubkey: String,
        executable: bool,
        lamports: u64,
        owner: Option<String>,
        rent_epoch: u64,
        space: i64,
        program: String,
        program_data: Option<String>,
    ) -> PackagedAccount {
        PackagedAccount {
            timestamp,
            pubkey,
            executable,
            lamports,
            owner,
            rent_epoch,
            space: Some(space),
            program: Some(program),
            account_type: Some(String::from("program")),
            authorized_voters: None,
            authorized_withdrawer: None,
            prior_voters: None,
            node_pubkey: None,
            commission: None,
            epoch_credits: None,
            votes: None,
            root_slot: None,
            last_timestamp: None,
            mint: None,
            program_data,
            token_amount: None,
            token_amount_decimals: None,
            is_native: None,
            data: None,
            state: None,
            mint_authority: None,
            supply: None,
        }
    }

    pub fn new_vote_type(
        timestamp: u64,
        pubkey: String,
        executable: bool,
        lamports: u64,
        owner: Option<String>,
        rent_epoch: u64,
        space: i64,
        program: String,
        authorized_voters: Option<Vec<AccountInfoAuthorizedVotersResponse>>,
        authorized_withdrawer: Option<String>,
        prior_voters: Option<Vec<AccountInfoAuthorizedPriorVotersResponse>>,
        node_pubkey: Option<String>,
        commission: Option<i64>,
        epoch_credits: Option<Vec<AccountInfoAuthorizedEpochCreditsResponse>>,
        votes: Option<Vec<AccountInfoVotesResponse>>,
        root_slot: Option<i64>,
        last_timestamp: Option<AccountInfoLastTimestampResponse>,
    ) -> PackagedAccount {
        PackagedAccount {
            timestamp,
            pubkey,
            executable,
            lamports,
            owner,
            rent_epoch,
            space: Some(space),
            program: Some(program),
            account_type: Some(String::from("vote")),
            authorized_voters,
            authorized_withdrawer,
            prior_voters,
            node_pubkey,
            commission,
            epoch_credits,
            votes,
            root_slot,
            last_timestamp,
            mint: None,
            program_data: None,
            token_amount: None,
            token_amount_decimals: None,
            is_native: None,
            data: None,
            state: None,
            mint_authority: None,
            supply: None,
        }
    }

    pub fn new_other_type(
        timestamp: u64,
        pubkey: String,
        executable: bool,
        lamports: u64,
        owner: String,
        rent_epoch: u64,
        space: i64,
        program: String,
        account_type: String,
        data: String,
    ) -> PackagedAccount {
        PackagedAccount {
            timestamp,
            pubkey,
            executable,
            lamports,
            owner: Some(owner),
            rent_epoch,
            space: Some(space),
            program: Some(program),
            account_type: Some(account_type),
            data: Some(EncodedData {
                raw: data,
                encoding: String::from("json"),
            }),
            authorized_voters: None,
            authorized_withdrawer: None,
            prior_voters: None,
            node_pubkey: None,
            commission: None,
            epoch_credits: None,
            votes: None,
            root_slot: None,
            last_timestamp: None,
            mint: None,
            program_data: None,
            token_amount: None,
            token_amount_decimals: None,
            is_native: None,
            state: None,
            mint_authority: None,
            supply: None,
        }
    }

    pub fn new_unused_type(
        timestamp: u64,
        pubkey: String,
        executable: bool,
        lamports: u64,
        owner: String,
        rent_epoch: u64,
        space: i64,
        program: String,
        account_type: String,
    ) -> PackagedAccount {
        PackagedAccount {
            timestamp,
            pubkey,
            executable,
            lamports,
            owner: Some(owner),
            rent_epoch,
            space: Some(space),
            program: Some(program),
            account_type: Some(account_type),
            data: None,
            authorized_voters: None,
            authorized_withdrawer: None,
            prior_voters: None,
            node_pubkey: None,
            commission: None,
            epoch_credits: None,
            votes: None,
            root_slot: None,
            last_timestamp: None,
            mint: None,
            program_data: None,
            token_amount: None,
            token_amount_decimals: None,
            is_native: None,
            state: None,
            mint_authority: None,
            supply: None,
        }
    }

    pub fn new_encoded_type(
        timestamp: u64,
        pubkey: String,
        executable: bool,
        lamports: u64,
        owner: String,
        rent_epoch: u64,
        data: String,
        encoding: String,
    ) -> PackagedAccount {
        PackagedAccount {
            timestamp,
            pubkey,
            executable,
            lamports,
            owner: Some(owner),
            rent_epoch,
            space: None,
            program: None,
            account_type: None,
            data: Some(EncodedData {
                raw: data,
                encoding,
            }),
            authorized_voters: None,
            authorized_withdrawer: None,
            prior_voters: None,
            node_pubkey: None,
            commission: None,
            epoch_credits: None,
            votes: None,
            root_slot: None,
            last_timestamp: None,
            mint: None,
            program_data: None,
            token_amount: None,
            token_amount_decimals: None,
            is_native: None,
            state: None,
            mint_authority: None,
            supply: None,
        }
    }
}

#[derive(Debug)]
struct EncodedData {
    raw: String,
    encoding: String,
}
