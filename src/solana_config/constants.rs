//! This module contains constants used by the Solana-Config.

/// The name of the stream.
/// Message-Passing queues require that you name them since you can have multiple on the same
/// system, we use STREAM_NAME to differentiate them.
pub const STREAM_NAME: &str = "solana-etl";

/// The maximum transaction version used
/// Used to define the max_supported_transaction_version for the Block Encoding Struct
pub const TRANSACTION_VERSION: u8 = 0;

/// used for retrieving token data, useful in [find_program_address](solana_sdk::pubkey::Pubkey::find_program_address)
pub const METADATA_BYTES: &[u8] = b"metadata";

/// used for retrieving token data, useful in [find_program_address](solana_sdk::pubkey::Pubkey::find_program_address)
pub const METADATA_PROGRAM_ID_STR: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";

/// Error code returned when calling for a block given a skipped slot
pub const SKIPPED_SLOT_ERROR_CODE: i32 = -32009;

/// Error code returned when calling for a block given a skipped slot
pub const LEDGER_JUMP_ERROR_CODE: i32 = -32007;

/// Error code returned when calling for a block that is to far behind the tip of the chain (backfill needed)
pub const OLD_BLOCK_SLOT_ERROR_CODE: i32 = -32001;

/// Error code returned when calling for a block that has yet to be confirmed
pub const UNCONFIRMED_BLOCK_SLOT_ERROR_CODE: i32 = -32004;
