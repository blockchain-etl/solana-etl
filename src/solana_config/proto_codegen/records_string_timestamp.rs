#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRecord {
    #[prost(int64, optional, tag = "1")]
    pub slot: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub block_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "4")]
    pub height: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "5")]
    pub previous_block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "6")]
    pub transaction_count: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "7")]
    pub leader_reward: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "8")]
    pub leader: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRewardRecord {
    #[prost(int64, optional, tag = "1")]
    pub block_slot: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub block_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "4")]
    pub commission: ::core::option::Option<u64>,
    #[prost(int64, optional, tag = "5")]
    pub lamports: ::core::option::Option<i64>,
    #[prost(uint64, optional, tag = "6")]
    pub post_balance: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "7")]
    pub pubkey: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub reward_type: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionRecord {
    #[prost(int64, optional, tag = "1")]
    pub block_slot: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub block_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub recent_block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "6")]
    pub index: ::core::option::Option<i64>,
    #[prost(uint64, optional, tag = "7")]
    pub fee: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "8")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub err: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "10")]
    pub compute_units_consumed: ::core::option::Option<u64>,
    #[prost(message, repeated, tag = "11")]
    pub accounts: ::prost::alloc::vec::Vec<TransactionAccountRecord>,
    #[prost(string, repeated, tag = "12")]
    pub log_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "13")]
    pub balance_changes: ::prost::alloc::vec::Vec<BalanceChangeRecord>,
    #[prost(message, repeated, tag = "14")]
    pub pre_token_balances: ::prost::alloc::vec::Vec<TokenBalanceRecord>,
    #[prost(message, repeated, tag = "15")]
    pub post_token_balances: ::prost::alloc::vec::Vec<TokenBalanceRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionAccountRecord {
    #[prost(string, optional, tag = "1")]
    pub pubkey: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub signer: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub writable: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceChangeRecord {
    #[prost(string, optional, tag = "1")]
    pub account: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "2")]
    pub before: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub after: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenBalanceRecord {
    #[prost(int64, optional, tag = "1")]
    pub account_index: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub amount: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "5")]
    pub decimals: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionRecord {
    #[prost(int64, optional, tag = "1")]
    pub block_slot: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub block_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub tx_signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "5")]
    pub index: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "6")]
    pub parent_index: ::core::option::Option<i64>,
    #[prost(string, repeated, tag = "7")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub parsed: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub program_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub instruction_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "13")]
    pub params: ::prost::alloc::vec::Vec<ParamsRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsRecord {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRecord {
    #[prost(int64, optional, tag = "1")]
    pub block_slot: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub block_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub tx_signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub retrieval_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub pubkey: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "7")]
    pub executable: ::core::option::Option<bool>,
    #[prost(uint64, optional, tag = "8")]
    pub lamports: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "9")]
    pub owner: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "10")]
    pub rent_epoch: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "11")]
    pub program: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "12")]
    pub space: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "13")]
    pub account_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "14")]
    pub is_native: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "15")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub state: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "17")]
    pub token_amount: ::core::option::Option<u64>,
    #[prost(int64, optional, tag = "18")]
    pub token_amount_decimals: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "19")]
    pub program_data: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "20")]
    pub authorized_voters: ::prost::alloc::vec::Vec<AuthorizedVoterRecord>,
    #[prost(string, optional, tag = "21")]
    pub authorized_withdrawer: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "22")]
    pub prior_voters: ::prost::alloc::vec::Vec<PriorVoterRecord>,
    #[prost(string, optional, tag = "23")]
    pub node_pubkey: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "24")]
    pub commission: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "25")]
    pub epoch_credits: ::prost::alloc::vec::Vec<EpochCreditRecord>,
    #[prost(message, repeated, tag = "26")]
    pub votes: ::prost::alloc::vec::Vec<VoteRecord>,
    #[prost(int64, optional, tag = "27")]
    pub root_slot: ::core::option::Option<i64>,
    #[prost(message, repeated, tag = "28")]
    pub last_timestamp: ::prost::alloc::vec::Vec<TimestampRecord>,
    #[prost(message, repeated, tag = "29")]
    pub data: ::prost::alloc::vec::Vec<DataRecord>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedVoterRecord {
    #[prost(string, optional, tag = "1")]
    pub authorized_voter: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2")]
    pub epoch: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriorVoterRecord {
    #[prost(string, optional, tag = "1")]
    pub authorized_pubkey: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2")]
    pub epoch_of_last_authorized_switch: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "3")]
    pub target_epoch: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochCreditRecord {
    #[prost(string, optional, tag = "1")]
    pub credits: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2")]
    pub epoch: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "3")]
    pub previous_credits: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoteRecord {
    #[prost(int64, optional, tag = "1")]
    pub confirmation_count: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "2")]
    pub slot: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampRecord {
    #[prost(string, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "2")]
    pub slot: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRecord {
    #[prost(string, optional, tag = "1")]
    pub raw: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub encoding: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenRecord {
    #[prost(int64, optional, tag = "1")]
    pub block_slot: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub block_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub tx_signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub retrieval_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub is_nft: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "7")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub symbol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "12")]
    pub seller_fee_basis_points: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "13")]
    pub creators: ::prost::alloc::vec::Vec<CreatorRecord>,
    #[prost(bool, optional, tag = "14")]
    pub primary_sale_happened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "15")]
    pub is_mutable: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatorRecord {
    #[prost(string, optional, tag = "1")]
    pub address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "2")]
    pub verified: ::core::option::Option<bool>,
    #[prost(int64, optional, tag = "3")]
    pub share: ::core::option::Option<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenTransferRecord {
    #[prost(int64, optional, tag = "1")]
    pub block_slot: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "2")]
    pub block_hash: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub block_timestamp: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub tx_signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub destination: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "8")]
    pub value: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "9")]
    pub fee: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "10")]
    pub fee_decimals: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "11")]
    pub memo: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "12")]
    pub decimals: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "13")]
    pub mint: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub mint_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub transfer_type: ::core::option::Option<::prost::alloc::string::String>,
}
