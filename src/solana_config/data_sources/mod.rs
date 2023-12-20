//! This module defines functions to access the APIs related to Solana.

pub mod json_rpc;

#[cfg(feature = "SOLANA_BIGTABLE")]
pub mod bigtable;
