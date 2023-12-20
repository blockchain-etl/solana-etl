#![allow(non_snake_case)]

use crate::solana_config::types::request_types::ResponseError;

// See Solana's official documentation on the getAccountInfo and getMultipleAccounts APIs:
// https://docs.solana.com/api/http#getaccountinfo
// https://docs.solana.com/api/http#getmultipleaccounts

/// Response to the `getAccountInfo` request.  Will either have a valid error value or
/// result value.
#[derive(serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct AccountResponse {
    /// The JSON-RPC ([Solana documentation](https://docs.solana.com/api/) says should be set to "2.0")
    jsonrpc: String,
    /// The error, if applicable, from the trying to retrieve [account](https://docs.solana.com/terminology#account) information.
    pub error: Option<ResponseError>,
    /// The result, if applicable, from trying to retrieve the [account](https://docs.solana.com/terminology#account) information.
    pub result: Option<AccountValueResponse>,
    /// An unique client-generated identifying integer [Solana documentation](https://docs.solana.com/api/http#request-formatting).
    id: i32,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct AccountValueResponse {
    pub context: ContextInfoResponse,
    pub value: Vec<Option<AccountInfoResponse>>,
}

#[derive(serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ContextInfoResponse {
    pub slot: i32,
    /// The API Version used (can be `"legacy"`, a number, or undefined)
    apiVersion: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoTokenAmountResponse {
    pub amount: String,
    pub decimals: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoAuthorizedVotersResponse {
    pub authorizedVoter: String,
    pub epoch: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoAuthorizedEpochCreditsResponse {
    pub credits: String,
    pub epoch: i64,
    pub previousCredits: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoLastTimestampResponse {
    pub slot: i64,
    pub timestamp: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoAuthorizedPriorVotersResponse {
    pub authorizedPubkey: String,
    pub epochOfLastAuthorizedSwitch: i64,
    pub targetEpoch: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoVotesResponse {
    pub confirmationCount: i64,
    pub slot: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct InfoFeeCalculatorResponse {
    pub authority: Option<String>,
    pub feeCalculator: FeeCalculatorResponse,
    pub blockhash: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FeeCalculatorResponse {
    pub lamportsPerSignature: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoInfoStructureResponse {
    pub programData: Option<String>,
    pub isNative: Option<bool>,
    pub mint: Option<String>,
    pub mintAuthority: Option<String>,
    pub freezeAuthority: Option<String>,
    pub isInitialized: Option<bool>,
    pub supply: Option<String>,
    pub decimals: Option<i64>,
    pub owner: Option<String>,
    pub state: Option<String>,
    pub tokenAmount: Option<AccountInfoTokenAmountResponse>,
    pub authorizedVoters: Option<Vec<AccountInfoAuthorizedVotersResponse>>,
    pub authorizedWithdrawer: Option<String>,
    pub commission: Option<i64>,
    pub epochCredits: Option<Vec<AccountInfoAuthorizedEpochCreditsResponse>>,
    pub lastTimestamp: Option<AccountInfoLastTimestampResponse>,
    pub nodePubkey: Option<String>,
    pub priorVoters: Option<Vec<AccountInfoAuthorizedPriorVotersResponse>>,
    pub rootSlot: Option<i64>,
    pub votes: Option<Vec<AccountInfoVotesResponse>>,
    pub epoch: Option<i64>,
    pub epochStartTimestamp: Option<i64>,
    pub leaderScheduleEpoch: Option<i64>,
    pub slot: Option<i64>, // NOTE: this is separate from rootSlot.
    pub unixTimestamp: Option<i64>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoInfoListResponse {
    pub hash: String,
    pub slot: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct StakeHistoryResponse {
    activating: i64,
    deactivating: i64,
    effective: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoStakeHistoryResponse {
    epoch: i64,
    stakeHistory: StakeHistoryResponse,
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AccountInfoInfoEnumResponse {
    List0(Vec<AccountInfoInfoListResponse>), // 0 = data, 1 = "base64"
    List1(Vec<InfoFeeCalculatorResponse>),
    List2(Vec<AccountInfoStakeHistoryResponse>),
    Structure(AccountInfoInfoStructureResponse),
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TokenOuterResponse {
    mint: String,
    update_authority: String,
    primary_sale_happened: bool,
    is_mutable: bool,
    data: AccountDataEnumResponse,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoDataParsedResponse {
    pub info: Option<AccountInfoInfoEnumResponse>,
    #[serde(rename = "type")]
    pub r#type: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoDataDataResponse {
    pub parsed: AccountInfoDataParsedResponse,
    pub program: String,
    pub space: i64,
}

#[allow(clippy::large_enum_variant)]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AccountDataEnumResponse {
    Array(Vec<String>), // 0 = data, 1 = "base64"
    AccountObject(AccountInfoDataDataResponse),
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoResponse {
    pub data: AccountDataEnumResponse,
    pub executable: bool,
    pub lamports: u64,
    pub owner: String,
    pub rentEpoch: u64,
    pub space: i64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct BalanceChange {
    pub account: String,
    pub before: u64,
    pub after: u64,
}
