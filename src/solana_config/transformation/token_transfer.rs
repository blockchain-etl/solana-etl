use crate::solana_config::proto_codegen::confirmed_block::{
    InnerInstruction, TokenTransferInstruction,
};

#[cfg(feature = "STRING_TIMESTAMP")]
use crate::solana_config::proto_codegen::records_string_timestamp::TokenTransferRecord;

#[cfg(feature = "INT_TIMESTAMP")]
use crate::solana_config::proto_codegen::records_int_timestamp::TokenTransferRecord;

use super::transaction::CommonTableFields;

pub fn transform_to_token_transfer_record(
    common_table_fields: CommonTableFields,
    instruction_type: &str,
    program: String,
    token_transfer_instruction: Option<TokenTransferInstruction>,
    prev_instruction: Option<InnerInstruction>,
) -> Option<TokenTransferRecord> {
    pub const TOKEN_PROGRAM: &str = "spl-token";
    pub const MEMO_PROGRAM: &str = "spl-memo";
    pub const SYSTEM_PROGRAM: &str = "system";
    pub const TRANSFER_CHECKED: &str = "transferChecked";
    pub const TRANSFER_CHECKED_WITH_FEE: &str = "transferCheckedWithFee";
    pub const BURN_CHECKED: &str = "burnChecked";
    pub const MINT_TO_CHECKED: &str = "mintToChecked";
    pub const BURN: &str = "burn";
    pub const SPL_TRANSFER: &str = "spl-transfer";
    pub const SPL_TRANSFER_WITH_FEE: &str = "spl-transfer-with-fee";
    pub const MINT_TO: &str = "mintTo";
    pub const TRANSFER: &str = "transfer";

    match token_transfer_instruction {
        None => None,
        Some(t) => {
            let block_slot = common_table_fields.block_slot;
            let block_hash = common_table_fields.block_hash;
            let block_timestamp = common_table_fields.block_timestamp;
            let tx_signature = common_table_fields.tx_signature;
            /*
                "all incoming transfers must have an accompanying memo instruction right before the transfer instruction."
                source: https://spl.solana.com/token-2022/extensions#required-memo-on-transfer
            */
            let memo = match prev_instruction {
                Some(prev) if Some(MEMO_PROGRAM) == prev.program.as_deref() => {
                    match (prev.parsed_string.as_deref(), &prev.parsed_dict) {
                        (Some(parsed_string), _) if !parsed_string.is_empty() => {
                            Some(parsed_string.to_string())
                        }
                        (_, Some(parsed_dict))
                            if parsed_dict.info.as_ref().map_or(true, |s| !s.is_empty()) =>
                        {
                            parsed_dict.info.clone()
                        }
                        _ => None,
                    }
                }
                _ => None,
            };

            let source = t.source;
            let destination = t.destination;
            let authority = t.authority;
            let value = t.amount;
            let decimals = t.decimals;
            let mint = t.mint;
            let mint_authority = t.mint_authority;
            //let tx_signature = instruction.signature;
            let fee = t.fee_amount;
            let fee_decimals = t.fee_amount_decimals;
            let token_transfer_record = if program == TOKEN_PROGRAM {
                match instruction_type {
                    TRANSFER => TokenTransferRecord {
                        block_slot,
                        block_hash,
                        block_timestamp,
                        tx_signature,
                        source,
                        destination,
                        authority,
                        value,
                        decimals: None,
                        mint: None,
                        mint_authority,
                        transfer_type: Some(String::from(SPL_TRANSFER)),
                        fee,
                        fee_decimals,
                        memo,
                    },
                    TRANSFER_CHECKED => TokenTransferRecord {
                        block_slot,
                        block_hash,
                        block_timestamp,
                        tx_signature,
                        source,
                        destination,
                        authority,
                        value,
                        decimals,
                        mint,
                        mint_authority: None,
                        transfer_type: Some(String::from(SPL_TRANSFER)),
                        fee,
                        fee_decimals,
                        memo,
                    },
                    TRANSFER_CHECKED_WITH_FEE => TokenTransferRecord {
                        block_slot,
                        block_hash,
                        block_timestamp,
                        tx_signature,
                        source,
                        destination,
                        authority,
                        value,
                        decimals,
                        mint,
                        mint_authority: None,
                        transfer_type: Some(String::from(SPL_TRANSFER_WITH_FEE)),
                        fee,
                        fee_decimals,
                        memo,
                    },
                    BURN => TokenTransferRecord {
                        block_slot,
                        block_hash,
                        block_timestamp,
                        tx_signature,
                        source: None,
                        destination: None,
                        authority,
                        value,
                        decimals: None,
                        mint,
                        mint_authority: None,
                        transfer_type: Some(String::from(BURN)),
                        fee,
                        fee_decimals,
                        memo,
                    },
                    BURN_CHECKED => TokenTransferRecord {
                        block_slot,
                        block_hash,
                        block_timestamp,
                        tx_signature,
                        source: None,
                        destination: None,
                        authority,
                        value,
                        decimals,
                        mint,
                        mint_authority: None,
                        transfer_type: Some(String::from(BURN)),
                        fee,
                        fee_decimals,
                        memo,
                    },
                    MINT_TO => TokenTransferRecord {
                        block_slot,
                        block_hash,
                        block_timestamp,
                        tx_signature,
                        source: None,
                        destination: None,
                        authority: None,
                        value,
                        decimals: None,
                        mint,
                        mint_authority,
                        transfer_type: Some(String::from(MINT_TO)),
                        fee,
                        fee_decimals,
                        memo,
                    },
                    MINT_TO_CHECKED => TokenTransferRecord {
                        block_slot,
                        block_hash,
                        block_timestamp,
                        tx_signature,
                        source: None,
                        destination: None,
                        authority: None,
                        value,
                        decimals,
                        mint,
                        mint_authority,
                        transfer_type: Some(String::from(MINT_TO)),
                        fee,
                        fee_decimals,
                        memo,
                    },
                    _ => return None,
                }
            } else if program == SYSTEM_PROGRAM && instruction_type == TRANSFER {
                TokenTransferRecord {
                    block_slot,
                    block_hash,
                    block_timestamp,
                    tx_signature,
                    source,
                    destination,
                    authority,
                    decimals: None,
                    fee: None,
                    fee_decimals: None,
                    value,
                    mint: None,
                    mint_authority: None,
                    transfer_type: Some(String::from(TRANSFER)),
                    memo,
                }
            } else {
                return None;
            };

            Some(token_transfer_record)
        }
    }
}
