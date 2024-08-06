#![allow(non_snake_case)]

use crate::solana_config::types::request_types::ResponseError;

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

/// The AccountValueResponse in the event that the solana API call was successful.
/// May contain multiple [accounts](https://docs.solana.com/terminology#account).
#[derive(serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct AccountValueResponse {
    /// Context is an RPC-ResponseContext JSON structure including a slot field at
    /// which the operation was evaluated
    pub context: ContextInfoResponse,
    /// The accounts' data returned from the api call.
    pub value: Vec<Option<AccountInfoResponse>>,
}

/// Structure used to provide "context" to a solana API call, particularly track a slot and the apiVersion.  This is
/// useful as many of the Solana API calls do not return the slot in the return data.
#[derive(serde::Deserialize, Clone, Debug)]
#[allow(dead_code)]
pub struct ContextInfoResponse {
    pub slot: i32,
    /// The API Version used (can be `"legacy"`, a number, or undefined)
    apiVersion: String,
}

/// Struct for token amounts that are returned. Contains unprocessed amount as string w/o proper decimal point.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoTokenAmountResponse {
    /// The raw balance without decimals (string representation of u64)
    pub amount: String, // TODO: not sure about this being a string
    /// Number of base 10 decimals to the right of the decimal point
    pub decimals: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoAuthorizedVotersResponse {
    pub authorizedVoter: String,
    pub epoch: i64,
}

/// Represents Earned Credits
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoAuthorizedEpochCreditsResponse {
    /// The amount of credits earned at that epoch
    pub credits: String,
    /// The epoch at which the credits were earned
    pub epoch: i64,
    /// The credits owned prior to this epoch
    pub previousCredits: String,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoLastTimestampResponse {
    pub slot: i64,
    /// Timestamp of when the account data was retrieved from the RPC Node.
    pub timestamp: i64,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoAuthorizedPriorVotersResponse {
    /// The public key of a Ed25519 key-pair related to the account
    pub authorizedPubkey: String,
    pub epochOfLastAuthorizedSwitch: i64,
    pub targetEpoch: i64,
}

/// Response given from AccountInfoVotes
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoVotesResponse {
    /// The number of confirmation votes for this account.
    pub confirmationCount: i64,
    pub slot: i64,
}

/// info: maps to a list of these
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct InfoFeeCalculatorResponse {
    pub authority: Option<String>,
    /// FeeCalculator object, the fee schedule for this block hash
    pub feeCalculator: FeeCalculatorResponse,
    /// a Hash as base-58 encoded string
    pub blockhash: String,
}

/// FeeCalculator object describing the cluster fee rate at the queried blockhash.  Deprecation Notice: <https://docs.solana.com/api/http#getfeecalculatorforblockhash>
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FeeCalculatorResponse {
    /// Number of [lamports](https://docs.solana.com/terminology#lamport) per [signature](https://docs.solana.com/terminology#signature)
    pub lamportsPerSignature: String,
}

/// The response from Solana Account information.  
///
/// Notes:
/// - Documentation on the Solana account info returned is limited.  Documentation may include
/// inferences, and when possible will link to Solana documentation where the inference was made.
/// structure
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoInfoStructureResponse {
    pub programData: Option<String>,
    /// True if the account is native to the Solana ecosystem.  View [native token](https://docs.solana.com/terminology#native-token) account
    /// ([more info](https://solana-labs.github.io/solana-program-library/token/js/interfaces/Account.html#mint))
    pub isNative: Option<bool>,
    /// [Mint](https://solana-labs.github.io/solana-program-library/token/js/interfaces/Account.html#mint) associated with the account.
    pub mint: Option<String>,
    /// Optional authority used to mint new tokens. The mint authority may only be provided during mint creation. If no mint authority is present then
    /// the mint has a fixed supply and no further tokens may be minted. ([more info](https://solana-labs.github.io/solana-program-library/token/js/interfaces/Mint.html#mintAuthority))
    pub mintAuthority: Option<String>,
    /// Optional authority to freeze token accounts ([more info](https://solana-labs.github.io/solana-program-library/token/js/interfaces/Mint.html#freezeAuthority))
    pub freezeAuthority: Option<String>,
    /// Optional information on whether the mint is initialized ()
    pub isInitialized: Option<bool>,
    /// Total supply of tokens (likely relating to the Mint)
    pub supply: Option<String>,
    /// Number of base 10 digits to the right of the decimal point.
    /// Notes:
    ///     - Context is not entirely clear in [Solana documentation](https://solana-labs.github.io/solana-program-library/token/js/interfaces/Mint.html#decimals)
    pub decimals: Option<i64>,
    /// The pubkey of the program this account has been assigned to
    pub owner: Option<String>,
    pub state: Option<String>,
    pub tokenAmount: Option<AccountInfoTokenAmountResponse>,
    pub authorizedVoters: Option<Vec<AccountInfoAuthorizedVotersResponse>>,
    pub authorizedWithdrawer: Option<String>,
    pub commission: Option<i64>,
    pub epochCredits: Option<Vec<AccountInfoAuthorizedEpochCreditsResponse>>,
    pub lastTimestamp: Option<AccountInfoLastTimestampResponse>,
    pub nodePubkey: Option<String>,
    pub priorVoters: Option<Vec<AccountInfoAuthorizedPriorVotersResponse>>, // TODO: this was a Option<Vec<AccountInfoAuthorizedPriorVotersResponse>>. check that this is valid.
    /// A [root](https://docs.solana.com/terminology#root) slot is a slot that has reached the maximum [lockout](https://docs.solana.com/terminology#lockout) on a [validator](https://docs.solana.com/terminology#validator).
    /// Unsure of its context in accounts.
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

///
#[allow(clippy::large_enum_variant)]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum AccountDataEnumResponse {
    Array(Vec<String>), // 0 = data, 1 = "base64"
    AccountObject(AccountInfoDataDataResponse),
    //TokenObject(TokenDataResponse),
}

/// AccountInfoResponse represents an individual account return data.
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AccountInfoResponse {
    /// The Account Data response.
    pub data: AccountDataEnumResponse,
    /// Whether the account is executable or non-executable, where "executable programs (accounts) are
    /// those comprised of immutable code that own and create other accounts that store the state",
    /// and non-executable accounts are more similar to "'storage' accounts which contain all
    /// other types of data like program variables, token balances, NFTs, fungible currencies, etc."
    /// [view src](https://www.alchemy.com/overviews/solana-account-model#:~:text=Solana%20account%20types%20are%20of,separate%20both%20type%20of%20actions.)
    pub executable: bool,
    /// [lamports](https://docs.solana.com/terminology#lamport)
    pub lamports: u64,
    /// The owner of the Account
    pub owner: String,
    /// The [epoch](https://docs.solana.com/terminology#epoch) where
    /// [rent](https://docs.solana.com/terminology#rent) is owed
    pub rentEpoch: u64,
    pub space: i64,
}

/// The balance change of an account, containing the before and after balances.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct BalanceChange {
    /// The account the balance change occured to
    pub account: String,
    /// The balance before the balance-change occured
    pub before: u64,
    /// The balance after the balance-change occured
    pub after: u64,
}
