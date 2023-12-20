//! this file contains various helper functions for interacting with token data.
//!
use super::proto_codegen::confirmed_block::UnixTimestamp;
use super::{
    accounts::{KeyedAccountInfoResponse, KeyedTimestampedAccounts},
    proto_codegen::account_info as solana_account_protobuf,
    types::account_response_types::{AccountDataEnumResponse, AccountInfoInfoEnumResponse},
};
use crate::solana_config::constants;
use borsh::BorshDeserialize;
use solana_sdk::borsh0_10::try_from_slice_unchecked;
use std::str::FromStr;

#[derive(Clone, BorshDeserialize, Debug, PartialEq)]
pub struct Metadata {
    key: mpl_token_metadata::types::Key,
    pub update_authority: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<mpl_token_metadata::types::Creator>>,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
}

/// Used to facilitate token data retrieval from the RPC Node, the struct contains
/// mint data for tokens and whether it is a NFT
#[derive(Clone)]
pub struct PackedTokenData {
    /// The mint associated with this token.
    pub mint: String,
    /// Whether the token is a non-fungible token.
    pub is_nft: bool,
}

/// retrieves the tokens associated with the mint accounts.
/// if there are no tokens, then an empty vector is returned.
pub fn get_tokens_from_mint_accounts(
    keyed_accounts: KeyedTimestampedAccounts,
) -> Vec<PackedTokenData> {
    let mut packed_token_data = Vec::new();

    for KeyedAccountInfoResponse {
        pubkey,
        accounts_data,
    } in keyed_accounts.accounts.value.into_iter()
    {
        if let AccountDataEnumResponse::AccountObject(p) = accounts_data.data {
            let account_type = p.parsed.r#type;
            if let Some(AccountInfoInfoEnumResponse::Structure(s)) = p.parsed.info {
                if account_type == "mint" {
                    let token_amount_decimals = s.decimals;
                    if token_amount_decimals.is_some() {
                        // TODO: when Rust stabilizes the LazyCell feature, we can initialize this once at startup.
                        let metadata_program_id = solana_sdk::pubkey::Pubkey::from_str(
                            constants::METADATA_PROGRAM_ID_STR,
                        )
                        .expect("METADATA_PROGRAM_ID is parseable into a key");
                        let mint_key = solana_sdk::pubkey::Pubkey::from_str(&pubkey)
                            .expect("account pubkey is parseable into a key");
                        let (metadata_account, _) =
                            solana_sdk::pubkey::Pubkey::find_program_address(
                                &[
                                    constants::METADATA_BYTES,
                                    &metadata_program_id.to_bytes(),
                                    &mint_key.to_bytes(),
                                ],
                                &metadata_program_id,
                            );
                        let is_nft = token_amount_decimals.map(|decimals| decimals == 0).unwrap();
                        packed_token_data.push(PackedTokenData {
                            mint: metadata_account.to_string(),
                            is_nft,
                        });
                    };
                }
            }
        }
    }

    packed_token_data
}

/// decodes the base64-encoded token mint account data, and returns the token data associated with it
pub fn unpack_token_account(
    token_data: AccountDataEnumResponse,
    is_nft: bool,
    token_retrieval_timestamp: u64,
) -> solana_account_protobuf::Token {
    match token_data {
        AccountDataEnumResponse::Array(strings) => {
            let string_slice: [String; 2] = strings.try_into().expect("accounts.data is 2 strings");
            let [encoded_data, _encoding] = string_slice;
            let base64_decoded_data =
                base64::Engine::decode(&base64::prelude::BASE64_STANDARD, encoded_data).unwrap();
            let metadata = try_from_slice_unchecked::<Metadata>(&base64_decoded_data).unwrap();

            solana_account_protobuf::Token {
                retrieval_timestamp: Some(UnixTimestamp {
                    timestamp: token_retrieval_timestamp as i64,
                }),
                is_nft,
                mint: metadata.mint.to_string(),
                update_authority: metadata.update_authority.to_string(),
                name: metadata.name.to_string().replace('\0', ""),
                symbol: metadata.symbol.to_string().replace('\0', ""),
                uri: metadata.uri.to_string().replace('\0', ""),
                seller_fee_basis_points: metadata.seller_fee_basis_points as u32,
                creators: match metadata.creators {
                    Some(creators) => creators
                        .into_iter()
                        .map(|creator| solana_account_protobuf::Creator {
                            address: creator.address.to_string(),
                            verified: creator.verified,
                            share: creator.share as u32,
                        })
                        .collect(),
                    None => Vec::new(),
                },
                primary_sale_happened: metadata.primary_sale_happened,
                is_mutable: metadata.is_mutable,
            }
        }
        _ => panic!("token data should be a list of strings"),
    }
}
