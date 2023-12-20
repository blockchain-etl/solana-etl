/// Response for [getSlot](https://docs.solana.com/api/http#getslot).
/// Contains the most recent slot with a confirmed block.
#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
pub struct SlotResponse {
    jsonrpc: String,
    /// The most recent slot with a confirmed block.
    pub result: u64,
    id: i32,
}

impl SlotResponse {
    pub fn get_slot(&self) -> u64 {
        self.result
    }
}
