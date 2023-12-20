#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionByAddr {
    #[prost(message, repeated, tag = "1")]
    pub tx_by_addrs: ::prost::alloc::vec::Vec<TransactionByAddrInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionByAddrInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub err: ::core::option::Option<TransactionError>,
    #[prost(uint32, tag = "3")]
    pub index: u32,
    #[prost(message, optional, tag = "4")]
    pub memo: ::core::option::Option<Memo>,
    #[prost(message, optional, tag = "5")]
    pub block_time: ::core::option::Option<UnixTimestamp>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Memo {
    #[prost(string, tag = "1")]
    pub memo: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionError {
    #[prost(enumeration = "TransactionErrorType", tag = "1")]
    pub transaction_error: i32,
    #[prost(message, optional, tag = "2")]
    pub instruction_error: ::core::option::Option<InstructionError>,
    #[prost(message, optional, tag = "3")]
    pub transaction_details: ::core::option::Option<TransactionDetails>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstructionError {
    #[prost(uint32, tag = "1")]
    pub index: u32,
    #[prost(enumeration = "InstructionErrorType", tag = "2")]
    pub error: i32,
    #[prost(message, optional, tag = "3")]
    pub custom: ::core::option::Option<CustomError>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionDetails {
    #[prost(uint32, tag = "1")]
    pub index: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnixTimestamp {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomError {
    #[prost(uint32, tag = "1")]
    pub custom: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionErrorType {
    AccountInUse = 0,
    AccountLoadedTwice = 1,
    AccountNotFound = 2,
    ProgramAccountNotFound = 3,
    InsufficientFundsForFee = 4,
    InvalidAccountForFee = 5,
    AlreadyProcessed = 6,
    BlockhashNotFound = 7,
    InstructionError = 8,
    CallChainTooDeep = 9,
    MissingSignatureForFee = 10,
    InvalidAccountIndex = 11,
    SignatureFailure = 12,
    InvalidProgramForExecution = 13,
    SanitizeFailure = 14,
    ClusterMaintenance = 15,
    AccountBorrowOutstandingTx = 16,
    WouldExceedMaxBlockCostLimit = 17,
    UnsupportedVersion = 18,
    InvalidWritableAccount = 19,
    WouldExceedMaxAccountCostLimit = 20,
    WouldExceedAccountDataBlockLimit = 21,
    TooManyAccountLocks = 22,
    AddressLookupTableNotFound = 23,
    InvalidAddressLookupTableOwner = 24,
    InvalidAddressLookupTableData = 25,
    InvalidAddressLookupTableIndex = 26,
    InvalidRentPayingAccount = 27,
    WouldExceedMaxVoteCostLimit = 28,
    WouldExceedAccountDataTotalLimit = 29,
    DuplicateInstruction = 30,
    InsufficientFundsForRent = 31,
    MaxLoadedAccountsDataSizeExceeded = 32,
    InvalidLoadedAccountsDataSizeLimit = 33,
}
impl TransactionErrorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TransactionErrorType::AccountInUse => "ACCOUNT_IN_USE",
            TransactionErrorType::AccountLoadedTwice => "ACCOUNT_LOADED_TWICE",
            TransactionErrorType::AccountNotFound => "ACCOUNT_NOT_FOUND",
            TransactionErrorType::ProgramAccountNotFound => "PROGRAM_ACCOUNT_NOT_FOUND",
            TransactionErrorType::InsufficientFundsForFee => "INSUFFICIENT_FUNDS_FOR_FEE",
            TransactionErrorType::InvalidAccountForFee => "INVALID_ACCOUNT_FOR_FEE",
            TransactionErrorType::AlreadyProcessed => "ALREADY_PROCESSED",
            TransactionErrorType::BlockhashNotFound => "BLOCKHASH_NOT_FOUND",
            TransactionErrorType::InstructionError => "INSTRUCTION_ERROR",
            TransactionErrorType::CallChainTooDeep => "CALL_CHAIN_TOO_DEEP",
            TransactionErrorType::MissingSignatureForFee => "MISSING_SIGNATURE_FOR_FEE",
            TransactionErrorType::InvalidAccountIndex => "INVALID_ACCOUNT_INDEX",
            TransactionErrorType::SignatureFailure => "SIGNATURE_FAILURE",
            TransactionErrorType::InvalidProgramForExecution => {
                "INVALID_PROGRAM_FOR_EXECUTION"
            }
            TransactionErrorType::SanitizeFailure => "SANITIZE_FAILURE",
            TransactionErrorType::ClusterMaintenance => "CLUSTER_MAINTENANCE",
            TransactionErrorType::AccountBorrowOutstandingTx => {
                "ACCOUNT_BORROW_OUTSTANDING_TX"
            }
            TransactionErrorType::WouldExceedMaxBlockCostLimit => {
                "WOULD_EXCEED_MAX_BLOCK_COST_LIMIT"
            }
            TransactionErrorType::UnsupportedVersion => "UNSUPPORTED_VERSION",
            TransactionErrorType::InvalidWritableAccount => "INVALID_WRITABLE_ACCOUNT",
            TransactionErrorType::WouldExceedMaxAccountCostLimit => {
                "WOULD_EXCEED_MAX_ACCOUNT_COST_LIMIT"
            }
            TransactionErrorType::WouldExceedAccountDataBlockLimit => {
                "WOULD_EXCEED_ACCOUNT_DATA_BLOCK_LIMIT"
            }
            TransactionErrorType::TooManyAccountLocks => "TOO_MANY_ACCOUNT_LOCKS",
            TransactionErrorType::AddressLookupTableNotFound => {
                "ADDRESS_LOOKUP_TABLE_NOT_FOUND"
            }
            TransactionErrorType::InvalidAddressLookupTableOwner => {
                "INVALID_ADDRESS_LOOKUP_TABLE_OWNER"
            }
            TransactionErrorType::InvalidAddressLookupTableData => {
                "INVALID_ADDRESS_LOOKUP_TABLE_DATA"
            }
            TransactionErrorType::InvalidAddressLookupTableIndex => {
                "INVALID_ADDRESS_LOOKUP_TABLE_INDEX"
            }
            TransactionErrorType::InvalidRentPayingAccount => {
                "INVALID_RENT_PAYING_ACCOUNT"
            }
            TransactionErrorType::WouldExceedMaxVoteCostLimit => {
                "WOULD_EXCEED_MAX_VOTE_COST_LIMIT"
            }
            TransactionErrorType::WouldExceedAccountDataTotalLimit => {
                "WOULD_EXCEED_ACCOUNT_DATA_TOTAL_LIMIT"
            }
            TransactionErrorType::DuplicateInstruction => "DUPLICATE_INSTRUCTION",
            TransactionErrorType::InsufficientFundsForRent => {
                "INSUFFICIENT_FUNDS_FOR_RENT"
            }
            TransactionErrorType::MaxLoadedAccountsDataSizeExceeded => {
                "MAX_LOADED_ACCOUNTS_DATA_SIZE_EXCEEDED"
            }
            TransactionErrorType::InvalidLoadedAccountsDataSizeLimit => {
                "INVALID_LOADED_ACCOUNTS_DATA_SIZE_LIMIT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCOUNT_IN_USE" => Some(Self::AccountInUse),
            "ACCOUNT_LOADED_TWICE" => Some(Self::AccountLoadedTwice),
            "ACCOUNT_NOT_FOUND" => Some(Self::AccountNotFound),
            "PROGRAM_ACCOUNT_NOT_FOUND" => Some(Self::ProgramAccountNotFound),
            "INSUFFICIENT_FUNDS_FOR_FEE" => Some(Self::InsufficientFundsForFee),
            "INVALID_ACCOUNT_FOR_FEE" => Some(Self::InvalidAccountForFee),
            "ALREADY_PROCESSED" => Some(Self::AlreadyProcessed),
            "BLOCKHASH_NOT_FOUND" => Some(Self::BlockhashNotFound),
            "INSTRUCTION_ERROR" => Some(Self::InstructionError),
            "CALL_CHAIN_TOO_DEEP" => Some(Self::CallChainTooDeep),
            "MISSING_SIGNATURE_FOR_FEE" => Some(Self::MissingSignatureForFee),
            "INVALID_ACCOUNT_INDEX" => Some(Self::InvalidAccountIndex),
            "SIGNATURE_FAILURE" => Some(Self::SignatureFailure),
            "INVALID_PROGRAM_FOR_EXECUTION" => Some(Self::InvalidProgramForExecution),
            "SANITIZE_FAILURE" => Some(Self::SanitizeFailure),
            "CLUSTER_MAINTENANCE" => Some(Self::ClusterMaintenance),
            "ACCOUNT_BORROW_OUTSTANDING_TX" => Some(Self::AccountBorrowOutstandingTx),
            "WOULD_EXCEED_MAX_BLOCK_COST_LIMIT" => {
                Some(Self::WouldExceedMaxBlockCostLimit)
            }
            "UNSUPPORTED_VERSION" => Some(Self::UnsupportedVersion),
            "INVALID_WRITABLE_ACCOUNT" => Some(Self::InvalidWritableAccount),
            "WOULD_EXCEED_MAX_ACCOUNT_COST_LIMIT" => {
                Some(Self::WouldExceedMaxAccountCostLimit)
            }
            "WOULD_EXCEED_ACCOUNT_DATA_BLOCK_LIMIT" => {
                Some(Self::WouldExceedAccountDataBlockLimit)
            }
            "TOO_MANY_ACCOUNT_LOCKS" => Some(Self::TooManyAccountLocks),
            "ADDRESS_LOOKUP_TABLE_NOT_FOUND" => Some(Self::AddressLookupTableNotFound),
            "INVALID_ADDRESS_LOOKUP_TABLE_OWNER" => {
                Some(Self::InvalidAddressLookupTableOwner)
            }
            "INVALID_ADDRESS_LOOKUP_TABLE_DATA" => {
                Some(Self::InvalidAddressLookupTableData)
            }
            "INVALID_ADDRESS_LOOKUP_TABLE_INDEX" => {
                Some(Self::InvalidAddressLookupTableIndex)
            }
            "INVALID_RENT_PAYING_ACCOUNT" => Some(Self::InvalidRentPayingAccount),
            "WOULD_EXCEED_MAX_VOTE_COST_LIMIT" => Some(Self::WouldExceedMaxVoteCostLimit),
            "WOULD_EXCEED_ACCOUNT_DATA_TOTAL_LIMIT" => {
                Some(Self::WouldExceedAccountDataTotalLimit)
            }
            "DUPLICATE_INSTRUCTION" => Some(Self::DuplicateInstruction),
            "INSUFFICIENT_FUNDS_FOR_RENT" => Some(Self::InsufficientFundsForRent),
            "MAX_LOADED_ACCOUNTS_DATA_SIZE_EXCEEDED" => {
                Some(Self::MaxLoadedAccountsDataSizeExceeded)
            }
            "INVALID_LOADED_ACCOUNTS_DATA_SIZE_LIMIT" => {
                Some(Self::InvalidLoadedAccountsDataSizeLimit)
            }
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstructionErrorType {
    GenericError = 0,
    InvalidArgument = 1,
    InvalidInstructionData = 2,
    InvalidAccountData = 3,
    AccountDataTooSmall = 4,
    InsufficientFunds = 5,
    IncorrectProgramId = 6,
    MissingRequiredSignature = 7,
    AccountAlreadyInitialized = 8,
    UninitializedAccount = 9,
    UnbalancedInstruction = 10,
    ModifiedProgramId = 11,
    ExternalAccountLamportSpend = 12,
    ExternalAccountDataModified = 13,
    ReadonlyLamportChange = 14,
    ReadonlyDataModified = 15,
    DuplicateAccountIndex = 16,
    ExecutableModified = 17,
    RentEpochModified = 18,
    NotEnoughAccountKeys = 19,
    AccountDataSizeChanged = 20,
    AccountNotExecutable = 21,
    AccountBorrowFailed = 22,
    AccountBorrowOutstanding = 23,
    DuplicateAccountOutOfSync = 24,
    Custom = 25,
    InvalidError = 26,
    ExecutableDataModified = 27,
    ExecutableLamportChange = 28,
    ExecutableAccountNotRentExempt = 29,
    UnsupportedProgramId = 30,
    CallDepth = 31,
    MissingAccount = 32,
    ReentrancyNotAllowed = 33,
    MaxSeedLengthExceeded = 34,
    InvalidSeeds = 35,
    InvalidRealloc = 36,
    ComputationalBudgetExceeded = 37,
    PrivilegeEscalation = 38,
    ProgramEnvironmentSetupFailure = 39,
    ProgramFailedToComplete = 40,
    ProgramFailedToCompile = 41,
    Immutable = 42,
    IncorrectAuthority = 43,
    BorshIoError = 44,
    AccountNotRentExempt = 45,
    InvalidAccountOwner = 46,
    ArithmeticOverflow = 47,
    UnsupportedSysvar = 48,
    IllegalOwner = 49,
    MaxAccountsDataAllocationsExceeded = 50,
    MaxAccountsExceeded = 51,
    MaxInstructionTraceLengthExceeded = 52,
    BuiltinProgramsMustConsumeComputeUnits = 53,
}
impl InstructionErrorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstructionErrorType::GenericError => "GENERIC_ERROR",
            InstructionErrorType::InvalidArgument => "INVALID_ARGUMENT",
            InstructionErrorType::InvalidInstructionData => "INVALID_INSTRUCTION_DATA",
            InstructionErrorType::InvalidAccountData => "INVALID_ACCOUNT_DATA",
            InstructionErrorType::AccountDataTooSmall => "ACCOUNT_DATA_TOO_SMALL",
            InstructionErrorType::InsufficientFunds => "INSUFFICIENT_FUNDS",
            InstructionErrorType::IncorrectProgramId => "INCORRECT_PROGRAM_ID",
            InstructionErrorType::MissingRequiredSignature => {
                "MISSING_REQUIRED_SIGNATURE"
            }
            InstructionErrorType::AccountAlreadyInitialized => {
                "ACCOUNT_ALREADY_INITIALIZED"
            }
            InstructionErrorType::UninitializedAccount => "UNINITIALIZED_ACCOUNT",
            InstructionErrorType::UnbalancedInstruction => "UNBALANCED_INSTRUCTION",
            InstructionErrorType::ModifiedProgramId => "MODIFIED_PROGRAM_ID",
            InstructionErrorType::ExternalAccountLamportSpend => {
                "EXTERNAL_ACCOUNT_LAMPORT_SPEND"
            }
            InstructionErrorType::ExternalAccountDataModified => {
                "EXTERNAL_ACCOUNT_DATA_MODIFIED"
            }
            InstructionErrorType::ReadonlyLamportChange => "READONLY_LAMPORT_CHANGE",
            InstructionErrorType::ReadonlyDataModified => "READONLY_DATA_MODIFIED",
            InstructionErrorType::DuplicateAccountIndex => "DUPLICATE_ACCOUNT_INDEX",
            InstructionErrorType::ExecutableModified => "EXECUTABLE_MODIFIED",
            InstructionErrorType::RentEpochModified => "RENT_EPOCH_MODIFIED",
            InstructionErrorType::NotEnoughAccountKeys => "NOT_ENOUGH_ACCOUNT_KEYS",
            InstructionErrorType::AccountDataSizeChanged => "ACCOUNT_DATA_SIZE_CHANGED",
            InstructionErrorType::AccountNotExecutable => "ACCOUNT_NOT_EXECUTABLE",
            InstructionErrorType::AccountBorrowFailed => "ACCOUNT_BORROW_FAILED",
            InstructionErrorType::AccountBorrowOutstanding => {
                "ACCOUNT_BORROW_OUTSTANDING"
            }
            InstructionErrorType::DuplicateAccountOutOfSync => {
                "DUPLICATE_ACCOUNT_OUT_OF_SYNC"
            }
            InstructionErrorType::Custom => "CUSTOM",
            InstructionErrorType::InvalidError => "INVALID_ERROR",
            InstructionErrorType::ExecutableDataModified => "EXECUTABLE_DATA_MODIFIED",
            InstructionErrorType::ExecutableLamportChange => "EXECUTABLE_LAMPORT_CHANGE",
            InstructionErrorType::ExecutableAccountNotRentExempt => {
                "EXECUTABLE_ACCOUNT_NOT_RENT_EXEMPT"
            }
            InstructionErrorType::UnsupportedProgramId => "UNSUPPORTED_PROGRAM_ID",
            InstructionErrorType::CallDepth => "CALL_DEPTH",
            InstructionErrorType::MissingAccount => "MISSING_ACCOUNT",
            InstructionErrorType::ReentrancyNotAllowed => "REENTRANCY_NOT_ALLOWED",
            InstructionErrorType::MaxSeedLengthExceeded => "MAX_SEED_LENGTH_EXCEEDED",
            InstructionErrorType::InvalidSeeds => "INVALID_SEEDS",
            InstructionErrorType::InvalidRealloc => "INVALID_REALLOC",
            InstructionErrorType::ComputationalBudgetExceeded => {
                "COMPUTATIONAL_BUDGET_EXCEEDED"
            }
            InstructionErrorType::PrivilegeEscalation => "PRIVILEGE_ESCALATION",
            InstructionErrorType::ProgramEnvironmentSetupFailure => {
                "PROGRAM_ENVIRONMENT_SETUP_FAILURE"
            }
            InstructionErrorType::ProgramFailedToComplete => "PROGRAM_FAILED_TO_COMPLETE",
            InstructionErrorType::ProgramFailedToCompile => "PROGRAM_FAILED_TO_COMPILE",
            InstructionErrorType::Immutable => "IMMUTABLE",
            InstructionErrorType::IncorrectAuthority => "INCORRECT_AUTHORITY",
            InstructionErrorType::BorshIoError => "BORSH_IO_ERROR",
            InstructionErrorType::AccountNotRentExempt => "ACCOUNT_NOT_RENT_EXEMPT",
            InstructionErrorType::InvalidAccountOwner => "INVALID_ACCOUNT_OWNER",
            InstructionErrorType::ArithmeticOverflow => "ARITHMETIC_OVERFLOW",
            InstructionErrorType::UnsupportedSysvar => "UNSUPPORTED_SYSVAR",
            InstructionErrorType::IllegalOwner => "ILLEGAL_OWNER",
            InstructionErrorType::MaxAccountsDataAllocationsExceeded => {
                "MAX_ACCOUNTS_DATA_ALLOCATIONS_EXCEEDED"
            }
            InstructionErrorType::MaxAccountsExceeded => "MAX_ACCOUNTS_EXCEEDED",
            InstructionErrorType::MaxInstructionTraceLengthExceeded => {
                "MAX_INSTRUCTION_TRACE_LENGTH_EXCEEDED"
            }
            InstructionErrorType::BuiltinProgramsMustConsumeComputeUnits => {
                "BUILTIN_PROGRAMS_MUST_CONSUME_COMPUTE_UNITS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "GENERIC_ERROR" => Some(Self::GenericError),
            "INVALID_ARGUMENT" => Some(Self::InvalidArgument),
            "INVALID_INSTRUCTION_DATA" => Some(Self::InvalidInstructionData),
            "INVALID_ACCOUNT_DATA" => Some(Self::InvalidAccountData),
            "ACCOUNT_DATA_TOO_SMALL" => Some(Self::AccountDataTooSmall),
            "INSUFFICIENT_FUNDS" => Some(Self::InsufficientFunds),
            "INCORRECT_PROGRAM_ID" => Some(Self::IncorrectProgramId),
            "MISSING_REQUIRED_SIGNATURE" => Some(Self::MissingRequiredSignature),
            "ACCOUNT_ALREADY_INITIALIZED" => Some(Self::AccountAlreadyInitialized),
            "UNINITIALIZED_ACCOUNT" => Some(Self::UninitializedAccount),
            "UNBALANCED_INSTRUCTION" => Some(Self::UnbalancedInstruction),
            "MODIFIED_PROGRAM_ID" => Some(Self::ModifiedProgramId),
            "EXTERNAL_ACCOUNT_LAMPORT_SPEND" => Some(Self::ExternalAccountLamportSpend),
            "EXTERNAL_ACCOUNT_DATA_MODIFIED" => Some(Self::ExternalAccountDataModified),
            "READONLY_LAMPORT_CHANGE" => Some(Self::ReadonlyLamportChange),
            "READONLY_DATA_MODIFIED" => Some(Self::ReadonlyDataModified),
            "DUPLICATE_ACCOUNT_INDEX" => Some(Self::DuplicateAccountIndex),
            "EXECUTABLE_MODIFIED" => Some(Self::ExecutableModified),
            "RENT_EPOCH_MODIFIED" => Some(Self::RentEpochModified),
            "NOT_ENOUGH_ACCOUNT_KEYS" => Some(Self::NotEnoughAccountKeys),
            "ACCOUNT_DATA_SIZE_CHANGED" => Some(Self::AccountDataSizeChanged),
            "ACCOUNT_NOT_EXECUTABLE" => Some(Self::AccountNotExecutable),
            "ACCOUNT_BORROW_FAILED" => Some(Self::AccountBorrowFailed),
            "ACCOUNT_BORROW_OUTSTANDING" => Some(Self::AccountBorrowOutstanding),
            "DUPLICATE_ACCOUNT_OUT_OF_SYNC" => Some(Self::DuplicateAccountOutOfSync),
            "CUSTOM" => Some(Self::Custom),
            "INVALID_ERROR" => Some(Self::InvalidError),
            "EXECUTABLE_DATA_MODIFIED" => Some(Self::ExecutableDataModified),
            "EXECUTABLE_LAMPORT_CHANGE" => Some(Self::ExecutableLamportChange),
            "EXECUTABLE_ACCOUNT_NOT_RENT_EXEMPT" => {
                Some(Self::ExecutableAccountNotRentExempt)
            }
            "UNSUPPORTED_PROGRAM_ID" => Some(Self::UnsupportedProgramId),
            "CALL_DEPTH" => Some(Self::CallDepth),
            "MISSING_ACCOUNT" => Some(Self::MissingAccount),
            "REENTRANCY_NOT_ALLOWED" => Some(Self::ReentrancyNotAllowed),
            "MAX_SEED_LENGTH_EXCEEDED" => Some(Self::MaxSeedLengthExceeded),
            "INVALID_SEEDS" => Some(Self::InvalidSeeds),
            "INVALID_REALLOC" => Some(Self::InvalidRealloc),
            "COMPUTATIONAL_BUDGET_EXCEEDED" => Some(Self::ComputationalBudgetExceeded),
            "PRIVILEGE_ESCALATION" => Some(Self::PrivilegeEscalation),
            "PROGRAM_ENVIRONMENT_SETUP_FAILURE" => {
                Some(Self::ProgramEnvironmentSetupFailure)
            }
            "PROGRAM_FAILED_TO_COMPLETE" => Some(Self::ProgramFailedToComplete),
            "PROGRAM_FAILED_TO_COMPILE" => Some(Self::ProgramFailedToCompile),
            "IMMUTABLE" => Some(Self::Immutable),
            "INCORRECT_AUTHORITY" => Some(Self::IncorrectAuthority),
            "BORSH_IO_ERROR" => Some(Self::BorshIoError),
            "ACCOUNT_NOT_RENT_EXEMPT" => Some(Self::AccountNotRentExempt),
            "INVALID_ACCOUNT_OWNER" => Some(Self::InvalidAccountOwner),
            "ARITHMETIC_OVERFLOW" => Some(Self::ArithmeticOverflow),
            "UNSUPPORTED_SYSVAR" => Some(Self::UnsupportedSysvar),
            "ILLEGAL_OWNER" => Some(Self::IllegalOwner),
            "MAX_ACCOUNTS_DATA_ALLOCATIONS_EXCEEDED" => {
                Some(Self::MaxAccountsDataAllocationsExceeded)
            }
            "MAX_ACCOUNTS_EXCEEDED" => Some(Self::MaxAccountsExceeded),
            "MAX_INSTRUCTION_TRACE_LENGTH_EXCEEDED" => {
                Some(Self::MaxInstructionTraceLengthExceeded)
            }
            "BUILTIN_PROGRAMS_MUST_CONSUME_COMPUTE_UNITS" => {
                Some(Self::BuiltinProgramsMustConsumeComputeUnits)
            }
            _ => None,
        }
    }
}
