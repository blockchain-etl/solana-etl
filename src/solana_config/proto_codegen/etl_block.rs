/// these fields are optional so that we can either send a block, or accounts data, or transactions data - and use a single protobuf and pub/sub topic.
/// accounts data + block data is very large, so splitting them up ensures we don't exceed google pub/sub's 10mb message limit.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtlBlock {
    #[prost(uint64, tag = "1")]
    pub slot: u64,
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<super::confirmed_block::ConfirmedBlock>,
    #[prost(message, repeated, tag = "3")]
    pub block_rewards: ::prost::alloc::vec::Vec<super::confirmed_block::Reward>,
    #[prost(message, repeated, tag = "4")]
    pub transactions: ::prost::alloc::vec::Vec<
        super::confirmed_block::ConfirmedTransaction,
    >,
    #[prost(message, repeated, tag = "5")]
    pub accounts: ::prost::alloc::vec::Vec<super::account_info::AccountInfo>,
    #[prost(message, optional, tag = "6")]
    pub table_context: ::core::option::Option<TableContext>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableContext {
    #[prost(message, optional, tag = "1")]
    pub block_timestamp: ::core::option::Option<super::confirmed_block::UnixTimestamp>,
    #[prost(string, tag = "2")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub previous_block_hash: ::prost::alloc::string::String,
}
