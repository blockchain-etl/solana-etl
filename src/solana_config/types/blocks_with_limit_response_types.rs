use crate::solana_config::types::request_types::ResponseError;


/// Response for [getBlocksWithLimit](https://docs.solana.com/api/http#getblockswithlimit).  Will either contain an error
/// or a result.
#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct BlocksWithLimitResponse {
    jsonrpc: String,
    pub error: Option<ResponseError>,
    /// A list of confirmed slots starting from the requested start slot, where the length is limited by either 500,000
    /// or the number provided by the request.
    pub result: Option<Vec<u64>>,
    id: i32,
}


