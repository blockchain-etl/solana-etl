//! # Account data helper functions
//! This module contains various helper functions for interacting with account data and
//! nested RPC response timestamps.
//!
//! ## Nested RPC response timeStamp
//! There are also nested structures for keeping the RPC response timestamp and account
//! pubkey with the account data that was retrieved from the RPC node.  The nested
//! structure of the timestamp pubkey data looks like this:
//! ```
//! KeyedTimestampedAccounts {
//!     timestamp,
//!     KeyedAccountValueResponse {
//!         KeyedAccountInfoResponse {
//!             account_data,
//!             pubkey
//!         }
//!     }
//! }
//! ```

use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(debug_assertions)]
use {std::time, tokio::time::sleep};

use log::error;

use super::{
    proto_conversions::account::PackagedAccount,
    types::account_response_types::{
        AccountDataEnumResponse, AccountInfoInfoEnumResponse, AccountInfoResponse,
        AccountValueResponse, ContextInfoResponse,
    },
};
use crate::{
    self as blockchain_generic,
    solana_config::{
        data_sources::json_rpc::get_multiple_accounts_post_body,
        types::account_response_types::AccountResponse,
    },
};

use blockchain_generic::{metrics::Metrics, source::config::RequestConfig};

use log::{info, warn};

/// Account Value Response with the timestamp of when the data was retrieved.
#[derive(Debug)]
pub struct TimestampedAccounts {
    /// A timestamp of when the [account](https://docs.solana.com/terminology#account) data was
    /// retrieved from the RPC node.
    pub timestamp: u64,
    /// The accounts' data retrieved
    pub accounts: AccountValueResponse,
}

/// KeyedAccount is the [account](https://docs.solana.com/terminology#account) data of a singular
/// account along with the [public key](https://docs.solana.com/terminology#public-key-pubkey)
/// of the account.  This is because the AccountInfoResponse does not contain the Pubkey, however it
/// is critical to keep the pubkey with the data so we can store it with the return data.
#[derive(Clone)]
pub struct KeyedAccountInfoResponse {
    /// The [public key](https://docs.solana.com/terminology#public-key-pubkey) associated with the
    /// [account](https://docs.solana.com/terminology#account).
    pub pubkey: String,
    /// The [account](https://docs.solana.com/terminology#account) data responded.
    pub accounts_data: AccountInfoResponse,
}

/// KeyedAccountValueResponse contain the context in which the call for the account(s)
/// was made, with the KeyedAccountInfoResponse containing both the timestamps when
/// the RPC node responded with the data and the public key.
#[derive(Clone)]
pub struct KeyedAccountValueResponse {
    /// Context relating to the [account](https://docs.solana.com/terminology#account) value
    /// response. (slot data & api version)
    pub context: ContextInfoResponse,
    /// A vector of the [account](https://docs.solana.com/terminology#account) data responded
    /// with the [public keys](https://docs.solana.com/terminology#public-key-pubkey)
    pub value: Vec<KeyedAccountInfoResponse>,
}

/// KeyedTimestampedAccounts contains account data and timestamps.
#[derive(Clone)]
pub struct KeyedTimestampedAccounts {
    /// Timestamp of when the [account](https://docs.solana.com/terminology#account) value response
    /// was retrieved.
    pub timestamp: u64,
    /// The [accounts](https://docs.solana.com/terminology#account) value response with retrieval
    /// timestamps, public key, and context
    pub accounts: KeyedAccountValueResponse,
}

impl KeyedTimestampedAccounts {
    /// constructor function
    pub fn from_keys_and_accounts(
        pubkeys: Vec<String>,
        timestamped_accounts: TimestampedAccounts,
    ) -> KeyedTimestampedAccounts {
        KeyedTimestampedAccounts {
            timestamp: timestamped_accounts.timestamp,
            accounts: KeyedAccountValueResponse {
                context: timestamped_accounts.accounts.context,
                value: timestamped_accounts
                    .accounts
                    .value
                    .into_iter()
                    .zip(pubkeys)
                    .filter_map(|(account, pubkey)| {
                        account.map(|accounts_data| KeyedAccountInfoResponse {
                            pubkey,
                            accounts_data,
                        })
                    })
                    .collect(),
            },
        }
    }
}

/// helper function for conversion from the raw data received from the RPC node to the protocol buffers specification.
/// each element of the returned vector is a separate account.
pub fn package_accounts(keyed_accounts: KeyedTimestampedAccounts) -> Vec<PackagedAccount> {
    let timestamp = keyed_accounts.timestamp;
    let mut accounts_packaged = Vec::new();

    for KeyedAccountInfoResponse {
        pubkey,
        accounts_data,
    } in keyed_accounts.accounts.value.into_iter()
    {
        let executable = accounts_data.executable;
        let lamports = accounts_data.lamports;
        let owner = accounts_data.owner;
        let rent_epoch = accounts_data.rentEpoch;

        match accounts_data.data {
            AccountDataEnumResponse::AccountObject(p) => {
                let space = p.space;
                let program = p.program;

                let account_type = p.parsed.r#type;
                match p.parsed.info {
                    Some(AccountInfoInfoEnumResponse::Structure(s)) => {
                        if account_type == "account" {
                            let is_native = s.isNative;
                            let owner = s.owner;
                            let mint = s.mint;
                            let state = s.state;
                            let (token_amount, token_amount_decimals) = match s.tokenAmount {
                                Some(inner) => (Some(inner.amount), Some(inner.decimals)),
                                None => (None, None),
                            };
                            let a = PackagedAccount::new_account_type(
                                timestamp,
                                pubkey,
                                executable,
                                lamports,
                                owner,
                                rent_epoch,
                                space,
                                program,
                                mint,
                                token_amount,
                                token_amount_decimals,
                                is_native,
                                state,
                            );
                            accounts_packaged.push(a);
                        } else if account_type == "mint" {
                            let token_amount_decimals = s.decimals;
                            let mint_authority = s.mintAuthority;
                            let supply = s.supply;
                            let a = PackagedAccount::new_mint_type(
                                timestamp,
                                pubkey,
                                executable,
                                lamports,
                                Some(owner),
                                rent_epoch,
                                space,
                                program,
                                token_amount_decimals,
                                mint_authority,
                                supply,
                            );
                            accounts_packaged.push(a);
                        } else if account_type == "program" {
                            let program_data = s.programData;
                            let a = PackagedAccount::new_program_type(
                                timestamp,
                                pubkey,
                                executable,
                                lamports,
                                Some(owner),
                                rent_epoch,
                                space,
                                program,
                                program_data,
                            );
                            accounts_packaged.push(a);
                        } else if account_type == "vote" {
                            let authorized_voters = s.authorizedVoters;
                            let authorized_withdrawer = s.authorizedWithdrawer;
                            let prior_voters = s.priorVoters;
                            let node_pubkey = s.nodePubkey;
                            let commission = s.commission;
                            let epoch_credits = s.epochCredits;
                            let votes = s.votes;
                            let root_slot = s.rootSlot;
                            let last_timestamp = s.lastTimestamp;

                            let a = PackagedAccount::new_vote_type(
                                timestamp,
                                pubkey,
                                executable,
                                lamports,
                                Some(owner),
                                rent_epoch,
                                space,
                                program,
                                authorized_voters,
                                authorized_withdrawer,
                                prior_voters,
                                node_pubkey,
                                commission,
                                epoch_credits,
                                votes,
                                root_slot,
                                last_timestamp,
                            );
                            accounts_packaged.push(a);
                        } else if account_type == "delegated" {
                            let a = PackagedAccount::new_unused_type(
                                timestamp,
                                pubkey,
                                executable,
                                lamports,
                                owner,
                                rent_epoch,
                                space,
                                program,
                                account_type,
                            );
                            accounts_packaged.push(a);
                        } else {
                            // store the raw account data as a json string if the account type is unknown

                            let data = serde_json::to_string(&s).unwrap_or_default();

                            let a = PackagedAccount::new_other_type(
                                timestamp,
                                pubkey,
                                executable,
                                lamports,
                                owner,
                                rent_epoch,
                                space,
                                program,
                                account_type,
                                data,
                            );

                            accounts_packaged.push(a);
                        }
                    }
                    Some(AccountInfoInfoEnumResponse::List0(_l)) => {
                        warn!("Encountered a hash-slot pair list. Unused for now...");
                        /*let _hash_slot_pairs: Vec<(String, i64)> = l
                        .into_iter()
                        .map(|pair| (pair.hash, pair.slot))
                        .collect();*/
                        let a = PackagedAccount::new_unused_type(
                            timestamp,
                            pubkey,
                            executable,
                            lamports,
                            owner,
                            rent_epoch,
                            space,
                            program,
                            account_type,
                        );
                        accounts_packaged.push(a);
                    }
                    Some(AccountInfoInfoEnumResponse::List1(_l)) => {
                        warn!("Encountered a feeCalculator list. Unused for now...");
                        let a = PackagedAccount::new_unused_type(
                            timestamp,
                            pubkey,
                            executable,
                            lamports,
                            owner,
                            rent_epoch,
                            space,
                            program,
                            account_type,
                        );
                        accounts_packaged.push(a);
                    }
                    Some(AccountInfoInfoEnumResponse::List2(_l)) => {
                        warn!("Encountered a stakeHistory list. Unused for now...");
                        let a = PackagedAccount::new_unused_type(
                            timestamp,
                            pubkey,
                            executable,
                            lamports,
                            owner,
                            rent_epoch,
                            space,
                            program,
                            account_type,
                        );
                        accounts_packaged.push(a);
                    }
                    None => {
                        let a = PackagedAccount::new_unused_type(
                            timestamp,
                            pubkey,
                            executable,
                            lamports,
                            owner,
                            rent_epoch,
                            space,
                            program,
                            account_type,
                        );
                        accounts_packaged.push(a);
                    }
                }
            }
            AccountDataEnumResponse::Array(strings) => {
                let string_slice: [String; 2] =
                    strings.try_into().expect("accounts.data is 2 strings");
                let [data, encoding] = string_slice;

                let a = PackagedAccount::new_encoded_type(
                    timestamp, pubkey, executable, lamports, owner, rent_epoch, data, encoding,
                );
                accounts_packaged.push(a);

                /*use base64::{engine, alphabet, Engine as _};
                if executable == false {
                    info!("excecutable=false so decoding the base64 token `data`...");
                    let decoded = base64::decode(data).unwrap();
                    //let e = engine::GeneralPurpose::new(&alphabet::URL_SAFE, engine::general_purpose::NO_PAD);
                    dbg!(decoded);
                }*/
            }
        }
    }

    accounts_packaged
}

/// makes a request for the account data associated with each of the pubkeys.
#[allow(non_snake_case)]
pub async fn call_getMultipleAccounts(
    request_config: RequestConfig,
    account_keys: Vec<String>,
    metrics: Option<Metrics>,
) -> Option<TimestampedAccounts> {
    // can only call getMultipleAccounts with up to 100 account pubkeys at once.
    // so we break up the vector by chunks of 100, and join each of the responses.
    let mut context: Option<ContextInfoResponse> = None;
    let mut AccountInfoResponses: Vec<Option<AccountInfoResponse>> = Vec::new();

    for chunk in account_keys.chunks(100) {
        info!("Making a call to getMultipleAccounts...");
        let deserialized_accounts = loop {
            let cur_builder = request_config.try_clone().unwrap();
            let response: reqwest::Response = blockchain_generic::call_rpc_method(
                cur_builder,
                get_multiple_accounts_post_body(chunk.to_vec()),
                metrics.clone(),
            )
            .await;
            #[cfg(debug_assertions)]
            {
                let r = response
                    .text()
                    .await
                    .expect("FATAL: could not parse the response as a string");

                if r.contains("Too many requests for a specific RPC call, contact your app developer or support@rpcpool.com.") {
                error!("throttled by the public node.");
                let seconds = time::Duration::from_secs(5);
                sleep(seconds).await;
                continue;
             }

                let deserialized_response: AccountResponse = match serde_json::from_str(&r) {
                    Ok(deserialized_response) => deserialized_response,
                    Err(e) => {
                        dbg!(r.clone());
                        warn!("Failed to parse the getMultipleAccounts() response for pubkeys {:?}: {:?}", chunk, e);
                        continue;
                    }
                };
                break deserialized_response;
            }

            #[cfg(not(debug_assertions))]
            {
                // the `serde_json` deserializer produces more information for debugging (such as the column in the string, and the value)
                //  but it is slower than `simd_json`.
                // NOTE: if something is broken in the simd_json crate, then we will not notice it if we compile in debug mode.
                let deserialized_response: Result<AccountResponse, simd_json::Error> = {
                    let response_bytes = match response.bytes().await {
                        Err(e) => {
                            error!("could not read response: {:?}", e);
                            continue;
                        }
                        Ok(val) => val,
                    };
                    let mut byte_vec = response_bytes.to_vec();
                    let byte_slice = byte_vec.as_mut_slice();
                    simd_json::from_slice(byte_slice)
                };
                match deserialized_response {
                    Ok(deserialized) => break deserialized,
                    Err(e) => {
                        warn!("Failed to parse the getMultipleAccounts() response for pubkeys {:?}: {:?}", chunk, e);
                        continue;
                    }
                }
            }
        };

        info!("Successfully deserialized accounts data");

        match context {
            // if it's the first/only call to getMultipleAccounts, then create the result vector (AccountInfoResponses)
            None => {
                let res_opt = deserialized_accounts.result;
                if let Some(res) = res_opt {
                    context = Some(res.context);
                    AccountInfoResponses = res.value.clone();
                }
            }
            // if this is a subsequent call to getMultipleAccounts, append to the result vector (AccountInfoResponses)
            Some(_) => {
                AccountInfoResponses.append(&mut deserialized_accounts.result.unwrap().value)
            }
        }
    }

    context.map(|c| {
        let accounts = AccountValueResponse {
            context: c,
            value: AccountInfoResponses,
        };

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time only moves forward")
            .as_secs();

        TimestampedAccounts {
            timestamp,
            accounts,
        }
    })
}
