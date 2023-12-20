//! This file contains various helper functions for interacting with transaction data.
use super::types::block_response_types::ParsedType;
use solana_transaction_status::{UiConfirmedBlock, UiTransaction};

pub struct TransactionAccounts {
    pub tx_signature: String,
    pub accounts: Vec<String>,
}

use log::debug;

/// retrieves a list of created accounts from a list of transactions.
/// if there are no created accounts, then an empty vector is returned.
pub fn get_pubkeys_from_transactions(
    transactions: Vec<&UiTransaction>,
) -> Vec<TransactionAccounts> {
    let mut all_transaction_accounts = Vec::new();
    for transaction in transactions.into_iter() {
        match &transaction.message {
            solana_transaction_status::UiMessage::Parsed(parsed_message) => {
                let instructions = parsed_message.instructions.clone();
                let mut new_account_pubkeys = Vec::new();
                for instruction in instructions {
                    match instruction {
                        solana_transaction_status::UiInstruction::Parsed(
                            parsed_or_partially_instruction,
                        ) => {
                            if let solana_transaction_status::UiParsedInstruction::Parsed(
                                parsed_instruction,
                            ) = parsed_or_partially_instruction
                            {
                                if parsed_instruction.parsed.is_object() {
                                    let instruction_parsed_type: ParsedType =
                                        parsed_instruction.parsed.into();
                                    let instruction_type = instruction_parsed_type.r#type;
                                    let instruction_info: Option<serde_json::Value> =
                                        instruction_parsed_type.info;

                                    if let (Some(t), Some(i)) = (instruction_type, instruction_info)
                                    {
                                        if parsed_instruction.program == "system"
                                            && t == "createAccount"
                                        {
                                            if let serde_json::Value::Object(info) = i {
                                                if let serde_json::Value::String(key) =
                                                    &info["newAccount"]
                                                {
                                                    if !key.is_empty() {
                                                        new_account_pubkeys.push(key.clone());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => panic!("should be requesting jsonParsed data"),
                    }
                }
                if !new_account_pubkeys.is_empty() {
                    let tx_signature = String::from(&transaction.signatures[0]);
                    let transaction_account = TransactionAccounts {
                        tx_signature,
                        accounts: new_account_pubkeys,
                    };
                    all_transaction_accounts.push(transaction_account);
                }
            }
            _ => continue,
        };
    }
    all_transaction_accounts
}

/// retrieves the transactions from a block.
/// if there are no transactions, then an empty vector is returned.
pub fn get_transactions_from_block(block: &UiConfirmedBlock) -> Vec<&UiTransaction> {
    if let Some(transactions) = &block.transactions {
        let transaction_len = transactions.len();
        let blockhash = &block.blockhash;
        debug!("number of transactions: {transaction_len} in block with hash {blockhash}");
    }
    block
        .transactions
        .iter()
        .flatten()
        .map(|transaction_with_meta| &transaction_with_meta.transaction)
        .map(|t| match t {
            solana_transaction_status::EncodedTransaction::Json(transaction) => transaction,
            _ => panic!("Unexpected transaction encoding"),
        })
        .collect()
}
