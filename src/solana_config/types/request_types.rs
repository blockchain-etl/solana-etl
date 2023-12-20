//! Contains structs for different requests.
//!
//! ## Solana Request Formatting
//! View the Solana Request Formatting documentation for more information on why the structs
//! are formatted in this way.
//! [Solana documentation](https://docs.solana.com/api/http#request-formatting)
//!
//!
//! ### Request Formatting Notes:
//! - jsonrpc: is not well explained in the Solana documentation, however it is noted that it is `set to "2.0"`
//! - id is an unique client-generated identifying integer.  Most examples have it set to `1`
//! - method is the name of the method being called
//! - params is a json Array of ordered parameter values.
#![allow(non_snake_case)]

/// Stores an error response.
#[derive(serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ResponseError {
    /// The error code returned, varies depending on request and error.
    pub code: i32,
    /// The error message returned, varies depending on request and error.
    message: String,
}

/// Request struct for [getBlockHeight](https://docs.solana.com/api/http#getblockheight)
#[derive(serde::Serialize)]
pub struct BlockHeightRequest {
    /// JSON-RPC version
    jsonrpc: String,
    /// an unique client-generated identifying integer
    id: i32,
    /// a string containing the method to be invoked
    method: String,
}

impl BlockHeightRequest {
    pub fn new() -> Self {
        Self {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from("getBlockHeight"),
        }
    }
}

impl Default for BlockHeightRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Optional parameters for [getBlock](https://docs.solana.com/api/http#getblock)
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct OptionalParams {
    /// Encoding format for each returned Transaction
    encoding: String,
    /// The level of transaction details to return
    transactionDetails: String,
    /// A boolean whther to poulate the `rewards` array.  Default is `true` if not provided.
    rewards: bool,
    /// Default is `confirmed`
    commitment: String,
    /// The max transaction version to return in responses.  If not provided, only legacy transcations are
    /// returned.  Returns an error if the requested block is a higher version.
    maxSupportedTransactionVersion: u32,
}

/// Request struct for [getBlock](https://docs.solana.com/api/http#get)
#[derive(serde::Serialize)]
pub struct BlockRequest {
    jsonrpc: String,
    id: i32,
    method: String,
    /// Stores the slot number and the optional parameters for the request.
    params: (u64, OptionalParams),
}

impl BlockRequest {
    pub fn new(slot: u64) -> Self {
        let opt = OptionalParams {
            encoding: String::from("jsonParsed"),
            transactionDetails: String::from("full"),
            rewards: true,
            commitment: String::from("finalized"),
            maxSupportedTransactionVersion: 0,
        };

        Self {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from("getBlock"),
            params: (slot, opt),
        }
    }
}

/// Request struct for [getBlocks](https://docs.solana.com/api/http#getblocks)
#[derive(serde::Serialize)]
pub struct BlocksRequest {
    jsonrpc: String,
    id: i32,
    method: String,
    /// the start_slot and end_slot (end_slot cannot be more than 500,000 blocks higher)
    params: (u64, u64),
}

impl BlocksRequest {
    pub fn new(start_slot: u64, end_slot: u64) -> Self {
        Self {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from("getBlocks"),
            params: (start_slot, end_slot),
        }
    }
}

/// Optional parameters for [getAccounts](https://docs.solana.com/api/http#getmultipleaccounts)
///
/// Notes:
/// - Did not include dataSlice and minContextSlot despite being optional arguments.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct OptionalAccountParams {
    /// Which [encoding](https://docs.solana.com/api/http#parsed-responses) to return, view api call for specific available
    /// responses.
    encoding: String,
    // / The number of bytes to return and the offset to which start reading.
    // dataSlice: (u32, u32),
    /// Either `"finalized"`, `"confirmed"`, or `"processed"`.  More documentation on the different
    /// [commitment](https://docs.solana.com/terminology#commitment) levels are available in the
    /// [Configuring State Commitment documentation](https://docs.solana.com/api/http#configuring-state-commitment).
    commitment: String,
    // / The minimum slot that the request can be evaluated at
    // minContextSlot: u32,
}

/// The request structure for [getMultipleAccounts](https://docs.solana.com/api/http#getmultipleaccounts)
#[derive(serde::Serialize)]
pub struct AccountsRequest {
    jsonrpc: String,
    id: i32,
    method: String,
    /// A vector of pubkeys (up to 100), along with the optional account parameters.
    params: (Vec<String>, OptionalAccountParams),
}

impl AccountsRequest {
    /// Creates an Accounts Requests given an array of pubkeys (up to 100)
    pub fn new(account: Vec<String>) -> Self {
        let opt = OptionalAccountParams {
            encoding: String::from("jsonParsed"),
            commitment: String::from("finalized"),
        };
        Self {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from("getMultipleAccounts"),
            params: (account, opt),
        }
    }
}

/// A struct to represent the request for the [getSlot](https://docs.solana.com/api/http#getslot) api call
#[derive(serde::Serialize)]
pub struct SlotRequest {
    jsonrpc: String,
    id: i32,
    method: String,
}

impl SlotRequest {
    /// Creates a SlotRequest given a start slot (inclusive) and the number of confirmed blocks
    /// we can return
    ///
    /// NOTE: according to [Solana API Documentation](https://docs.solana.com/api/http#getslot),
    pub fn new() -> Self {
        Self {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from("getSlot"),
        }
    }
}

impl Default for SlotRequest {
    fn default() -> Self {
        Self::new()
    }
}
