#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountInfo {
    /// TableContext tableContext = 1;
    /// AccountsWithContext accountsWithContext = 2;
    #[prost(string, optional, tag = "1")]
    pub tx_signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
    #[prost(message, repeated, tag = "3")]
    pub tokens: ::prost::alloc::vec::Vec<Token>,
}
/// Protocol Buffer for context, however only contains slot, not the API version
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountContext {
    #[prost(int32, tag = "1")]
    pub slot: i32,
}
/// Protocol Buffer for AccountInfo
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(string, tag = "1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub executable: bool,
    #[prost(uint64, tag = "3")]
    pub lamports: u64,
    #[prost(string, optional, tag = "4")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag = "5")]
    pub rent_epoch: u64,
    #[prost(int64, optional, tag = "6")]
    pub space: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "7")]
    pub program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub program_data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub account_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "10")]
    pub is_native: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "11")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub state: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub token_amount: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "14")]
    pub token_amount_decimals: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "15")]
    pub authorized_voters: ::prost::alloc::vec::Vec<AuthorizedVoter>,
    #[prost(string, optional, tag = "16")]
    pub authorized_withdrawer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "17")]
    pub prior_voters: ::prost::alloc::vec::Vec<PriorVoters>,
    #[prost(string, optional, tag = "18")]
    pub node_pubkey: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "19")]
    pub commission: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "20")]
    pub epoch_credits: ::prost::alloc::vec::Vec<EpochCredit>,
    #[prost(message, repeated, tag = "21")]
    pub votes: ::prost::alloc::vec::Vec<Votes>,
    #[prost(int64, optional, tag = "22")]
    pub root_slot: ::core::option::Option<i64>,
    #[prost(message, optional, tag = "23")]
    pub last_timestamp: ::core::option::Option<LastTimestamp>,
    #[prost(string, optional, tag = "24")]
    pub mint_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "25")]
    pub supply: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "26")]
    pub data: ::core::option::Option<EncodedData>,
    #[prost(message, optional, tag = "27")]
    pub retrieval_timestamp: ::core::option::Option<
        super::confirmed_block::UnixTimestamp,
    >,
}
/// Encoded data stores the raw data as a string along with the encoding utilized.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncodedData {
    #[prost(string, tag = "1")]
    pub raw: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub encoding: ::prost::alloc::string::String,
}
/// Authorized voter contains the pubkey and the epoch
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedVoter {
    #[prost(string, tag = "1")]
    pub authorized_voter: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub epoch: i64,
}
/// Credits earned in the epoch
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochCredit {
    #[prost(string, tag = "1")]
    pub credits: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub epoch: i64,
    #[prost(string, tag = "3")]
    pub previous_credits: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriorVoters {
    #[prost(string, tag = "1")]
    pub authorized_pubkey: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub epoch_of_last_authorized_switch: i64,
    #[prost(int64, tag = "3")]
    pub target_epoch: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastTimestamp {
    #[prost(int64, tag = "1")]
    pub slot: i64,
    #[prost(int64, tag = "2")]
    pub timestamp: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Votes {
    #[prost(int64, tag = "1")]
    pub slot: i64,
    #[prost(int64, tag = "2")]
    pub confirmation_count: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(bool, tag = "1")]
    pub is_nft: bool,
    #[prost(string, tag = "2")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub update_authority: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    #[prost(uint32, tag = "7")]
    pub seller_fee_basis_points: u32,
    #[prost(message, repeated, tag = "8")]
    pub creators: ::prost::alloc::vec::Vec<Creator>,
    #[prost(bool, tag = "9")]
    pub primary_sale_happened: bool,
    #[prost(bool, tag = "10")]
    pub is_mutable: bool,
    #[prost(message, optional, tag = "11")]
    pub retrieval_timestamp: ::core::option::Option<
        super::confirmed_block::UnixTimestamp,
    >,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Creator {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub verified: bool,
    #[prost(uint32, tag = "3")]
    pub share: u32,
}
