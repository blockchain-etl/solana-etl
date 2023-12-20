/// this is used for parsing the json response from calling the rpc node's
/// [getBlockHeight()](https://docs.solana.com/api/http#getblockheight) function.
#[derive(serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct BlockHeightResponse {
    jsonrpc: String,
    /// The current block height
    pub result: u64,
    id: i32,
}

impl BlockHeightResponse {
    pub fn get_block_height(&self) -> u64 {
        self.result
    }
}
