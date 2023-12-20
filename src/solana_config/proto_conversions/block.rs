//! This module has structs and implementations that can help convert block
//! data between conversions.
use crate::solana_config::proto_codegen::confirmed_block::{
    BlockHeight, CompiledAccount, ConfirmedBlock, ConfirmedTransaction, InnerInstruction,
    InnerInstructions, Message, MessageAddressTableLookup,
    MessageHeader as generated_message_header, Parsed, ReturnData, Reward as generated_reward,
    RewardType as generated_reward_type, TokenBalance, TokenTransferInstruction, Transaction,
    TransactionError, TransactionStatusMeta, UiTokenAmount as generated_token_amount,
    UnixTimestamp,
};
use crate::solana_config::types::block_response_types::{
    InstructionError, ParsedType, TransactionErrorSolana,
};
use solana_account_decoder::parse_token::UiTokenAmount;
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::UiParsedInstruction;

use crate::solana_config::proto_codegen::transaction_by_addr::{self as tx_by_addr};
use solana_sdk::message::MessageHeader;
use solana_transaction_status::parse_accounts::ParsedAccount;
use solana_transaction_status::{
    parse_accounts::ParsedAccountSource, EncodedTransaction, EncodedTransactionWithStatusMeta,
    Reward, RewardType, UiAddressTableLookup, UiConfirmedBlock, UiInnerInstructions, UiInstruction,
    UiMessage, UiTransaction, UiTransactionReturnData, UiTransactionStatusMeta,
    UiTransactionTokenBalance,
};

/// converts from a single solana block to protobuf types.
/// block rewards and transactions are pulled out of the block.
pub fn parsed_block_to_proto(
    item: UiConfirmedBlock,
) -> (
    ConfirmedBlock,
    Vec<generated_reward>,
    Vec<ConfirmedTransaction>,
) {
    let rewards_inner = item.rewards.unwrap_or_default();
    let transactions_inner = item.transactions.unwrap_or_default();
    let transaction_count = transactions_inner.len() as u32;

    let block = ConfirmedBlock {
        previous_blockhash: item.previous_blockhash.clone(),
        blockhash: item.blockhash.clone(),
        parent_slot: item.parent_slot,
        block_time: item.block_time.map(|t| UnixTimestamp { timestamp: t }),
        block_height: item.block_height.map(BlockHeight::from),
        transaction_count,
        leader: rewards_inner
            .get(0)
            .map(|r| r.pubkey.clone())
            .unwrap_or_default(),
        leader_reward: rewards_inner
            .get(0)
            .map(|r| r.lamports as u64)
            .unwrap_or_default(),
    };

    let block_rewards: Vec<generated_reward> = rewards_inner
        .into_iter()
        .map(generated_reward::from)
        .collect();
    let transactions = transactions_inner
        .into_iter()
        .map(ConfirmedTransaction::from)
        .collect();

    (block, block_rewards, transactions)
}

/// Type Conversion for BlockHeight
/// NOTE: this is used for the type conversion from the rpc `getBlock()` json response
/// to the protobuf struct - NOT `getBlockHeight()`
impl From<u64> for BlockHeight {
    fn from(item: u64) -> Self {
        BlockHeight { block_height: item }
    }
}

impl From<Reward> for generated_reward {
    fn from(item: Reward) -> Self {
        generated_reward {
            pubkey: item.pubkey,
            lamports: item.lamports as u64,
            post_balance: item.post_balance,
            reward_type: match item.reward_type {
                None => generated_reward_type::Unspecified,
                Some(RewardType::Fee) => generated_reward_type::Fee,
                Some(RewardType::Rent) => generated_reward_type::Rent,
                Some(RewardType::Staking) => generated_reward_type::Staking,
                Some(RewardType::Voting) => generated_reward_type::Voting,
            } as i32,
            commission: item.commission.map(|c| c.to_string()).unwrap_or_default(),
        }
    }
}

impl From<UiTokenAmount> for generated_token_amount {
    fn from(item: UiTokenAmount) -> Self {
        generated_token_amount {
            amount: item.amount,
            decimals: item.decimals as u32,
            ui_amount: item.ui_amount.unwrap_or_default(),
            ui_amount_string: item.ui_amount_string,
        }
    }
}

impl From<UiTransactionTokenBalance> for TokenBalance {
    fn from(item: UiTransactionTokenBalance) -> Self {
        TokenBalance {
            account_index: item.account_index as u32,
            mint: item.mint,
            ui_token_amount: Some(generated_token_amount::from(item.ui_token_amount)),
            owner: match item.owner {
                OptionSerializer::None | OptionSerializer::Skip => String::new(),

                OptionSerializer::Some(s) => s,
            },
            program_id: match item.program_id {
                OptionSerializer::None | OptionSerializer::Skip => String::new(),

                OptionSerializer::Some(s) => s,
            },
        }
    }
}

impl From<serde_json::Value> for TokenTransferInstruction {
    fn from(value: serde_json::Value) -> Self {
        TokenTransferInstruction {
            amount: value
                .get("amount")
                .and_then(|v| v.as_str())
                .map(|str_value| str_value.parse::<u64>().unwrap_or_default()),
            source: value
                .get("source")
                .and_then(|v| v.as_str())
                .map(String::from),
            destination: value
                .get("destination")
                .and_then(|v| v.as_str())
                .map(String::from),
            authority: value
                .get("authority")
                .and_then(|v| v.as_str())
                .map(String::from),
            mint_authority: value
                .get("mintAuthority")
                .and_then(|v| v.as_str())
                .map(String::from),
            mint: value.get("mint").and_then(|v| v.as_str()).map(String::from),
            token_amount: value
                .get("tokenAmount")
                .and_then(|ta| ta.get("amount"))
                .and_then(|v| v.as_str())
                .map(String::from),
            token_amount_decimals: value
                .get("tokenAmount")
                .and_then(|tad| tad.get("decimals"))
                .and_then(|v| v.as_u64()),
            decimals: value.get("decimals").and_then(|v| v.as_u64()),
            lamports: value.get("lamports").and_then(|v| v.as_u64()),
            fee_amount: value
                .get("feeAmount")
                .and_then(|fee| fee.get("amount"))
                .and_then(|v| v.as_str())
                .map(|str_value| str_value.parse::<u64>().unwrap_or_default()),
            fee_amount_decimals: value
                .get("feeAmount")
                .and_then(|fee| fee.get("decimals"))
                .and_then(|v| v.as_u64()),
        }
    }
}

impl From<ParsedType> for Parsed {
    fn from(item: ParsedType) -> Self {
        Parsed {
            parsed: match item.parsed {
                None => Some(String::new()),
                Some(s) => Some(serde_json::json!(s).to_string()),
            },
            r#type: item.r#type,
            info: match item.info.clone() {
                None => Some(String::new()),
                Some(s) => Some(serde_json::json!(s).to_string()),
            },
            token_transfer_instruction: item.info.clone().map(TokenTransferInstruction::from),
        }
    }
}

impl From<serde_json::Value> for ParsedType {
    fn from(value: serde_json::Value) -> Self {
        serde_json::from_value(value)
            .expect("ParsedInstruction's ParsedDict should match ParsedType format")
    }
}

impl From<UiInstruction> for InnerInstruction {
    fn from(item: UiInstruction) -> Self {
        match item {
            UiInstruction::Compiled(_) => panic!("FATAL: should be using JsonParsed encoding."),
            UiInstruction::Parsed(instruction) => match instruction {
                UiParsedInstruction::Parsed(_instruction) => {
                    let (parsed_dict, parsed_string) = match _instruction.parsed {
                        serde_json::Value::String(s) => (None, Some(s)),
                        v => (
                            Some(Parsed::from(ParsedType::from(serde_json::json!(v)))),
                            None,
                        ),
                    };
                    InnerInstruction {
                        program: Some(_instruction.program),
                        program_id: Some(_instruction.program_id),
                        stack_height: None, // TODO: from version solana version 1.16, _instruction.stack_height is available.
                        parsed_dict,
                        parsed_string,
                        accounts: Vec::new(),
                        data: None,
                    }
                }
                UiParsedInstruction::PartiallyDecoded(_instruction) => InnerInstruction {
                    program: None,
                    program_id: Some(_instruction.program_id),
                    stack_height: None, // TODO: from version solana version 1.16, _instruction.stack_height is available.
                    parsed_dict: None,
                    parsed_string: None,
                    accounts: _instruction.accounts,
                    data: Some(_instruction.data),
                },
            },
        }
    }
}

impl From<UiInnerInstructions> for InnerInstructions {
    fn from(item: UiInnerInstructions) -> Self {
        InnerInstructions {
            index: item.index as u32,
            instructions: item
                .instructions
                .into_iter()
                .map(InnerInstruction::from)
                .collect(),
        }
    }
}

impl From<UiTransactionReturnData> for ReturnData {
    fn from(item: UiTransactionReturnData) -> ReturnData {
        ReturnData {
            program_id: item.program_id,
            data: item.data.0,
        }
    }
}

impl From<TransactionErrorSolana> for tx_by_addr::TransactionError {
    fn from(transaction_error: TransactionErrorSolana) -> Self {
        Self {
            transaction_error: match transaction_error {
                TransactionErrorSolana::AccountInUse => {
                    tx_by_addr::TransactionErrorType::AccountInUse
                }
                TransactionErrorSolana::AccountLoadedTwice => {
                    tx_by_addr::TransactionErrorType::AccountLoadedTwice
                }
                TransactionErrorSolana::AccountNotFound => {
                    tx_by_addr::TransactionErrorType::AccountNotFound
                }
                TransactionErrorSolana::ProgramAccountNotFound => {
                    tx_by_addr::TransactionErrorType::ProgramAccountNotFound
                }
                TransactionErrorSolana::InsufficientFundsForFee => {
                    tx_by_addr::TransactionErrorType::InsufficientFundsForFee
                }
                TransactionErrorSolana::InvalidAccountForFee => {
                    tx_by_addr::TransactionErrorType::InvalidAccountForFee
                }
                TransactionErrorSolana::AlreadyProcessed => {
                    tx_by_addr::TransactionErrorType::AlreadyProcessed
                }
                TransactionErrorSolana::BlockhashNotFound => {
                    tx_by_addr::TransactionErrorType::BlockhashNotFound
                }
                TransactionErrorSolana::CallChainTooDeep => {
                    tx_by_addr::TransactionErrorType::CallChainTooDeep
                }
                TransactionErrorSolana::MissingSignatureForFee => {
                    tx_by_addr::TransactionErrorType::MissingSignatureForFee
                }
                TransactionErrorSolana::InvalidAccountIndex => {
                    tx_by_addr::TransactionErrorType::InvalidAccountIndex
                }
                TransactionErrorSolana::SignatureFailure => {
                    tx_by_addr::TransactionErrorType::SignatureFailure
                }
                TransactionErrorSolana::InvalidProgramForExecution => {
                    tx_by_addr::TransactionErrorType::InvalidProgramForExecution
                }
                TransactionErrorSolana::SanitizeFailure => {
                    tx_by_addr::TransactionErrorType::SanitizeFailure
                }
                TransactionErrorSolana::ClusterMaintenance => {
                    tx_by_addr::TransactionErrorType::ClusterMaintenance
                }
                TransactionErrorSolana::InstructionError(_, _) => {
                    tx_by_addr::TransactionErrorType::InstructionError
                }
                TransactionErrorSolana::AccountBorrowOutstanding => {
                    tx_by_addr::TransactionErrorType::AccountBorrowOutstandingTx
                }
                TransactionErrorSolana::WouldExceedMaxBlockCostLimit => {
                    tx_by_addr::TransactionErrorType::WouldExceedMaxBlockCostLimit
                }
                TransactionErrorSolana::UnsupportedVersion => {
                    tx_by_addr::TransactionErrorType::UnsupportedVersion
                }
                TransactionErrorSolana::InvalidWritableAccount => {
                    tx_by_addr::TransactionErrorType::InvalidWritableAccount
                }
                TransactionErrorSolana::WouldExceedMaxAccountCostLimit => {
                    tx_by_addr::TransactionErrorType::WouldExceedMaxAccountCostLimit
                }
                TransactionErrorSolana::WouldExceedAccountDataBlockLimit => {
                    tx_by_addr::TransactionErrorType::WouldExceedAccountDataBlockLimit
                }
                TransactionErrorSolana::TooManyAccountLocks => {
                    tx_by_addr::TransactionErrorType::TooManyAccountLocks
                }
                TransactionErrorSolana::AddressLookupTableNotFound => {
                    tx_by_addr::TransactionErrorType::AddressLookupTableNotFound
                }
                TransactionErrorSolana::InvalidAddressLookupTableOwner => {
                    tx_by_addr::TransactionErrorType::InvalidAddressLookupTableOwner
                }
                TransactionErrorSolana::InvalidAddressLookupTableData => {
                    tx_by_addr::TransactionErrorType::InvalidAddressLookupTableData
                }
                TransactionErrorSolana::InvalidAddressLookupTableIndex => {
                    tx_by_addr::TransactionErrorType::InvalidAddressLookupTableIndex
                }
                TransactionErrorSolana::InvalidRentPayingAccount => {
                    tx_by_addr::TransactionErrorType::InvalidRentPayingAccount
                }
                TransactionErrorSolana::WouldExceedMaxVoteCostLimit => {
                    tx_by_addr::TransactionErrorType::WouldExceedMaxVoteCostLimit
                }
                TransactionErrorSolana::WouldExceedAccountDataTotalLimit => {
                    tx_by_addr::TransactionErrorType::WouldExceedAccountDataTotalLimit
                }
                TransactionErrorSolana::DuplicateInstruction(_) => {
                    tx_by_addr::TransactionErrorType::DuplicateInstruction
                }
                TransactionErrorSolana::InsufficientFundsForRent { .. } => {
                    tx_by_addr::TransactionErrorType::InsufficientFundsForRent
                }
                TransactionErrorSolana::MaxLoadedAccountsDataSizeExceeded => {
                    tx_by_addr::TransactionErrorType::MaxLoadedAccountsDataSizeExceeded
                }
                TransactionErrorSolana::InvalidLoadedAccountsDataSizeLimit => {
                    tx_by_addr::TransactionErrorType::InvalidLoadedAccountsDataSizeLimit
                }
            } as i32,
            instruction_error: match transaction_error {
                TransactionErrorSolana::InstructionError(index, ref instruction_error) => {
                    Some(tx_by_addr::InstructionError {
                        index: index as u32,
                        error: match instruction_error {
                            InstructionError::GenericError => {
                                tx_by_addr::InstructionErrorType::GenericError
                            }
                            InstructionError::InvalidArgument => {
                                tx_by_addr::InstructionErrorType::InvalidArgument
                            }
                            InstructionError::InvalidInstructionData => {
                                tx_by_addr::InstructionErrorType::InvalidInstructionData
                            }
                            InstructionError::InvalidAccountData => {
                                tx_by_addr::InstructionErrorType::InvalidAccountData
                            }
                            InstructionError::AccountDataTooSmall => {
                                tx_by_addr::InstructionErrorType::AccountDataTooSmall
                            }
                            InstructionError::InsufficientFunds => {
                                tx_by_addr::InstructionErrorType::InsufficientFunds
                            }
                            InstructionError::IncorrectProgramId => {
                                tx_by_addr::InstructionErrorType::IncorrectProgramId
                            }
                            InstructionError::MissingRequiredSignature => {
                                tx_by_addr::InstructionErrorType::MissingRequiredSignature
                            }
                            InstructionError::AccountAlreadyInitialized => {
                                tx_by_addr::InstructionErrorType::AccountAlreadyInitialized
                            }
                            InstructionError::UninitializedAccount => {
                                tx_by_addr::InstructionErrorType::UninitializedAccount
                            }
                            InstructionError::UnbalancedInstruction => {
                                tx_by_addr::InstructionErrorType::UnbalancedInstruction
                            }
                            InstructionError::ModifiedProgramId => {
                                tx_by_addr::InstructionErrorType::ModifiedProgramId
                            }
                            InstructionError::ExternalAccountLamportSpend => {
                                tx_by_addr::InstructionErrorType::ExternalAccountLamportSpend
                            }
                            InstructionError::ExternalAccountDataModified => {
                                tx_by_addr::InstructionErrorType::ExternalAccountDataModified
                            }
                            InstructionError::ReadonlyLamportChange => {
                                tx_by_addr::InstructionErrorType::ReadonlyLamportChange
                            }
                            InstructionError::ReadonlyDataModified => {
                                tx_by_addr::InstructionErrorType::ReadonlyDataModified
                            }
                            InstructionError::DuplicateAccountIndex => {
                                tx_by_addr::InstructionErrorType::DuplicateAccountIndex
                            }
                            InstructionError::ExecutableModified => {
                                tx_by_addr::InstructionErrorType::ExecutableModified
                            }
                            InstructionError::RentEpochModified => {
                                tx_by_addr::InstructionErrorType::RentEpochModified
                            }
                            InstructionError::NotEnoughAccountKeys => {
                                tx_by_addr::InstructionErrorType::NotEnoughAccountKeys
                            }
                            InstructionError::AccountDataSizeChanged => {
                                tx_by_addr::InstructionErrorType::AccountDataSizeChanged
                            }
                            InstructionError::AccountNotExecutable => {
                                tx_by_addr::InstructionErrorType::AccountNotExecutable
                            }
                            InstructionError::AccountBorrowFailed => {
                                tx_by_addr::InstructionErrorType::AccountBorrowFailed
                            }
                            InstructionError::AccountBorrowOutstanding => {
                                tx_by_addr::InstructionErrorType::AccountBorrowOutstanding
                            }
                            InstructionError::DuplicateAccountOutOfSync => {
                                tx_by_addr::InstructionErrorType::DuplicateAccountOutOfSync
                            }
                            InstructionError::Custom(_) => tx_by_addr::InstructionErrorType::Custom,
                            InstructionError::InvalidError => {
                                tx_by_addr::InstructionErrorType::InvalidError
                            }
                            InstructionError::ExecutableDataModified => {
                                tx_by_addr::InstructionErrorType::ExecutableDataModified
                            }
                            InstructionError::ExecutableLamportChange => {
                                tx_by_addr::InstructionErrorType::ExecutableLamportChange
                            }
                            InstructionError::ExecutableAccountNotRentExempt => {
                                tx_by_addr::InstructionErrorType::ExecutableAccountNotRentExempt
                            }
                            InstructionError::UnsupportedProgramId => {
                                tx_by_addr::InstructionErrorType::UnsupportedProgramId
                            }
                            InstructionError::CallDepth => {
                                tx_by_addr::InstructionErrorType::CallDepth
                            }
                            InstructionError::MissingAccount => {
                                tx_by_addr::InstructionErrorType::MissingAccount
                            }
                            InstructionError::ReentrancyNotAllowed => {
                                tx_by_addr::InstructionErrorType::ReentrancyNotAllowed
                            }
                            InstructionError::MaxSeedLengthExceeded => {
                                tx_by_addr::InstructionErrorType::MaxSeedLengthExceeded
                            }
                            InstructionError::InvalidSeeds => {
                                tx_by_addr::InstructionErrorType::InvalidSeeds
                            }
                            InstructionError::InvalidRealloc => {
                                tx_by_addr::InstructionErrorType::InvalidRealloc
                            }
                            InstructionError::ComputationalBudgetExceeded => {
                                tx_by_addr::InstructionErrorType::ComputationalBudgetExceeded
                            }
                            InstructionError::PrivilegeEscalation => {
                                tx_by_addr::InstructionErrorType::PrivilegeEscalation
                            }
                            InstructionError::ProgramEnvironmentSetupFailure => {
                                tx_by_addr::InstructionErrorType::ProgramEnvironmentSetupFailure
                            }
                            InstructionError::ProgramFailedToComplete => {
                                tx_by_addr::InstructionErrorType::ProgramFailedToComplete
                            }
                            InstructionError::ProgramFailedToCompile => {
                                tx_by_addr::InstructionErrorType::ProgramFailedToCompile
                            }
                            InstructionError::Immutable => {
                                tx_by_addr::InstructionErrorType::Immutable
                            }
                            InstructionError::IncorrectAuthority => {
                                tx_by_addr::InstructionErrorType::IncorrectAuthority
                            }
                            InstructionError::BorshIoError(_) => {
                                tx_by_addr::InstructionErrorType::BorshIoError
                            }
                            InstructionError::AccountNotRentExempt => {
                                tx_by_addr::InstructionErrorType::AccountNotRentExempt
                            }
                            InstructionError::InvalidAccountOwner => {
                                tx_by_addr::InstructionErrorType::InvalidAccountOwner
                            }
                            InstructionError::ArithmeticOverflow => {
                                tx_by_addr::InstructionErrorType::ArithmeticOverflow
                            }
                            InstructionError::UnsupportedSysvar => {
                                tx_by_addr::InstructionErrorType::UnsupportedSysvar
                            }
                            InstructionError::IllegalOwner => {
                                tx_by_addr::InstructionErrorType::IllegalOwner
                            }
                            InstructionError::MaxAccountsDataAllocationsExceeded => {
                                tx_by_addr::InstructionErrorType::MaxAccountsDataAllocationsExceeded
                            }
                            InstructionError::MaxAccountsExceeded => {
                                tx_by_addr::InstructionErrorType::MaxAccountsExceeded
                            }
                            InstructionError::MaxInstructionTraceLengthExceeded => {
                                tx_by_addr::InstructionErrorType::MaxInstructionTraceLengthExceeded
                            }
                        } as i32,
                        custom: match instruction_error {
                            InstructionError::Custom(custom) => {
                                Some(tx_by_addr::CustomError { custom: *custom })
                            }
                            _ => None,
                        },
                    })
                }
                _ => None,
            },
            transaction_details: match transaction_error {
                TransactionErrorSolana::DuplicateInstruction(index) => {
                    Some(tx_by_addr::TransactionDetails {
                        index: index as u32,
                    })
                }
                TransactionErrorSolana::InsufficientFundsForRent { account_index } => {
                    Some(tx_by_addr::TransactionDetails {
                        index: account_index as u32,
                    })
                }
                _ => None,
            },
        }
    }
}

impl From<UiTransactionStatusMeta> for TransactionStatusMeta {
    fn from(item: UiTransactionStatusMeta) -> Self {
        TransactionStatusMeta {
            err: match item.status {
                Ok(()) => None,
                Err(err) => Some(TransactionError {
                    err: err.to_string(),
                }),
            },
            fee: item.fee,
            pre_balances: item.pre_balances,
            post_balances: item.post_balances,
            inner_instructions_none: item.inner_instructions == OptionSerializer::None,
            inner_instructions: match item.inner_instructions {
                OptionSerializer::None | OptionSerializer::Skip => Vec::new(),
                OptionSerializer::Some(ii) => ii.into_iter().map(InnerInstructions::from).collect(),
            },
            log_messages_none: item.log_messages == OptionSerializer::None,
            log_messages: match item.log_messages {
                OptionSerializer::None | OptionSerializer::Skip => Vec::new(),
                OptionSerializer::Some(lm) => lm,
            },
            pre_token_balances: match item.pre_token_balances {
                OptionSerializer::None | OptionSerializer::Skip => Vec::new(),
                OptionSerializer::Some(ptb) => ptb.into_iter().map(TokenBalance::from).collect(),
            },
            post_token_balances: match item.post_token_balances {
                OptionSerializer::None | OptionSerializer::Skip => Vec::new(),
                OptionSerializer::Some(ptb) => ptb.into_iter().map(TokenBalance::from).collect(),
            },
            rewards: match item.rewards {
                OptionSerializer::None | OptionSerializer::Skip => Vec::new(),
                OptionSerializer::Some(rewards) => {
                    rewards.into_iter().map(generated_reward::from).collect()
                }
            },
            return_data_none: item.return_data == OptionSerializer::None,
            return_data: match item.return_data {
                OptionSerializer::None | OptionSerializer::Skip => None,
                OptionSerializer::Some(rd) => Some(ReturnData::from(rd)),
            },
            compute_units_consumed: Option::<u64>::from(item.compute_units_consumed),
        }
    }
}

impl From<MessageHeader> for generated_message_header {
    fn from(item: MessageHeader) -> Self {
        generated_message_header {
            num_required_signatures: item.num_required_signatures as u32,
            num_readonly_signed_accounts: item.num_readonly_signed_accounts as u32,
            num_readonly_unsigned_accounts: item.num_readonly_signed_accounts as u32,
        }
    }
}

impl From<UiAddressTableLookup> for MessageAddressTableLookup {
    fn from(item: UiAddressTableLookup) -> MessageAddressTableLookup {
        MessageAddressTableLookup {
            account_key: item.account_key,
            writable_indexes: item.writable_indexes,
            readonly_indexes: item.readonly_indexes,
        }
    }
}

impl From<ParsedAccount> for CompiledAccount {
    fn from(item: ParsedAccount) -> Self {
        CompiledAccount {
            pubkey: item.pubkey,
            signer: item.signer,
            source: match item.source {
                None => String::new(),
                Some(ParsedAccountSource::LookupTable) => String::from("LookupTable"),
                Some(ParsedAccountSource::Transaction) => String::from("Transaction"),
            },
            writable: item.writable,
        }
    }
}

impl From<UiMessage> for Message {
    fn from(item: UiMessage) -> Self {
        match item {
            UiMessage::Parsed(message) => Message {
                header: None,
                account_keys: message
                    .account_keys
                    .into_iter()
                    .map(CompiledAccount::from)
                    .collect(),
                recent_blockhash: message.recent_blockhash,
                instructions: message
                    .instructions
                    .into_iter()
                    .map(InnerInstruction::from)
                    .collect(),
                versioned: false, // TODO: what is this field for?
                address_table_lookups: message
                    .address_table_lookups
                    .unwrap_or_default()
                    .into_iter()
                    .map(MessageAddressTableLookup::from)
                    .collect(),
            },
            UiMessage::Raw(_) => panic!("FATAL: should be using JsonParsed encoding."),
        }
    }
}

impl From<UiTransaction> for Transaction {
    fn from(item: UiTransaction) -> Self {
        Transaction {
            signatures: item.signatures,
            message: Some(Message::from(item.message)),
        }
    }
}

impl From<EncodedTransaction> for Transaction {
    fn from(item: EncodedTransaction) -> Self {
        match item {
            EncodedTransaction::Json(t) => Transaction {
                signatures: t.signatures,
                message: Some(Message::from(t.message)),
            },
            EncodedTransaction::Accounts(_) => panic!("FATAL: should be using JsonParsed encoding"),
            EncodedTransaction::LegacyBinary(_) | EncodedTransaction::Binary(..) => {
                panic!("FATAL: should be using JsonParsed encoding")
            }
        }
    }
}

impl From<EncodedTransactionWithStatusMeta> for ConfirmedTransaction {
    fn from(item: EncodedTransactionWithStatusMeta) -> Self {
        ConfirmedTransaction {
            transaction: Some(Transaction::from(item.transaction)),
            meta: item.meta.map(TransactionStatusMeta::from),
            // NOTE: the transaction version is stored in the EncodedTransactionWithStatusMeta.
            // we may want to carry that info forward to the generated type.
        }
    }
}
