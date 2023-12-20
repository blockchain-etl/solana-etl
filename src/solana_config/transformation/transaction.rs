use crate::solana_config::proto_codegen::{confirmed_block::CompiledAccount, etl_block::EtlBlock};

#[cfg(feature = "STRING_TIMESTAMP")]
use {
    crate::solana_config::proto_codegen::records_string_timestamp::{
        BalanceChangeRecord, InstructionRecord, TokenBalanceRecord, TokenTransferRecord,
        TransactionAccountRecord, TransactionRecord,
    },
    chrono::{NaiveDateTime, TimeZone, Utc},
};

#[cfg(feature = "INT_TIMESTAMP")]
use crate::solana_config::proto_codegen::records_int_timestamp::{
    BalanceChangeRecord, InstructionRecord, TokenBalanceRecord, TokenTransferRecord,
    TransactionAccountRecord, TransactionRecord,
};

use super::{instruction, token_transfer};

#[derive(Clone)]
pub struct CommonTableFields {
    pub block_slot: Option<i64>,
    pub block_hash: Option<String>,
    #[cfg(feature = "STRING_TIMESTAMP")]
    pub block_timestamp: Option<String>,
    #[cfg(feature = "INT_TIMESTAMP")]
    pub block_timestamp: Option<i64>,
    pub tx_signature: Option<String>,
}

pub fn transform_to_transaction_records(
    etl_block: &EtlBlock,
) -> (
    Vec<TransactionRecord>,
    Vec<InstructionRecord>,
    Vec<TokenTransferRecord>,
) {
    let mut transaction_records = Vec::with_capacity(etl_block.transactions.len());
    let mut instruction_records = Vec::new();
    let mut token_transfer_records = Vec::new();

    let table_context = etl_block.table_context.to_owned().unwrap();
    let block_slot = Some(etl_block.slot as i64);
    let block_hash = Some(table_context.block_hash.to_owned());

    #[cfg(feature = "STRING_TIMESTAMP")]
    let block_timestamp = table_context.block_timestamp.as_ref().map(|bt| {
        Utc.from_utc_datetime(&NaiveDateTime::from_timestamp_opt(bt.timestamp, 0).unwrap())
            .to_rfc3339()
    });
    #[cfg(feature = "INT_TIMESTAMP")]
    let block_timestamp = table_context
        .block_timestamp
        .to_owned()
        .map(|bt| bt.timestamp * 1_000_000);

    for (tx_index, tx_with_meta) in etl_block.transactions.iter().enumerate() {
        let recent_block_hash: Option<String>;
        let tx_lead_signature: Option<String>;
        let transaction_account_records: Vec<TransactionAccountRecord>;
        let tx_accounts: Vec<CompiledAccount>;
        let common_table_fields: CommonTableFields;
        match &tx_with_meta.transaction {
            Some(tx) => {
                tx_lead_signature = tx.signatures.get(0).cloned();
                common_table_fields = CommonTableFields {
                    block_slot,
                    block_hash: block_hash.to_owned(),
                    block_timestamp: block_timestamp.to_owned(),
                    tx_signature: tx_lead_signature.to_owned(),
                };
                match &tx.message {
                    None => {
                        recent_block_hash = None;
                        tx_accounts = Vec::new();
                        transaction_account_records = Vec::new();
                    }
                    Some(tx_message) => {
                        recent_block_hash = Some(tx_message.recent_blockhash.to_owned());
                        tx_accounts = tx_message.account_keys.to_owned();
                        transaction_account_records = tx_accounts
                            .iter()
                            .map(|k| TransactionAccountRecord {
                                pubkey: Some(k.pubkey.to_owned()),
                                signer: Some(k.signer),
                                writable: Some(k.writable),
                            })
                            .collect();

                        // instructions
                        instruction_records.reserve_exact(tx_message.instructions.len());
                        let mut prev_instruction = None;
                        for (instruction_index, instruction) in
                            tx_message.instructions.iter().enumerate()
                        {
                            let instruction_record = instruction::transform_to_instruction_record(
                                common_table_fields.to_owned(),
                                instruction,
                                None,
                                instruction_index,
                            );

                            instruction_records.push(instruction_record);

                            let (instruction_type, token_transfer_instruction) = instruction
                                .parsed_dict
                                .to_owned()
                                .map(|p| (p.r#type, p.token_transfer_instruction))
                                .unwrap_or((None, None));

                            // token transfers
                            if let Some(r#type) = instruction_type.to_owned() {
                                let token_transfer_record =
                                    token_transfer::transform_to_token_transfer_record(
                                        common_table_fields.to_owned(),
                                        &r#type,
                                        instruction.program.to_owned().unwrap_or_default(),
                                        token_transfer_instruction,
                                        prev_instruction,
                                    );

                                if let Some(tt) = token_transfer_record {
                                    token_transfer_records.push(tt);
                                }
                            }
                            prev_instruction = Some(instruction.to_owned());
                        }
                    }
                }
            }
            None => {
                recent_block_hash = None;
                tx_accounts = Vec::new();
                transaction_account_records = Vec::new();
                tx_lead_signature = None;
                common_table_fields = CommonTableFields {
                    block_slot,
                    block_hash: block_hash.to_owned(),
                    block_timestamp: block_timestamp.to_owned(),
                    tx_signature: None,
                };
            }
        }

        match &tx_with_meta.meta {
            Some(meta) => {
                let balance_changes: Vec<BalanceChangeRecord> = tx_accounts
                    .into_iter()
                    .enumerate()
                    .map(|(i, account)| BalanceChangeRecord {
                        account: Some(account.pubkey),
                        before: Some(meta.pre_balances[i]),
                        after: Some(meta.post_balances[i]),
                    })
                    .collect();
                let pre_token_balances = meta
                    .pre_token_balances
                    .iter()
                    .map(|bal| TokenBalanceRecord {
                        account_index: Some(bal.account_index as i64),
                        mint: Some(bal.mint.to_owned()),
                        owner: Some(bal.owner.to_owned()),
                        amount: bal
                            .ui_token_amount
                            .as_ref()
                            .map(|ui_token| ui_token.amount.to_owned()),
                        decimals: bal
                            .ui_token_amount
                            .to_owned()
                            .map(|ui_token| ui_token.decimals as i64),
                    })
                    .collect();
                let post_token_balances = meta
                    .post_token_balances
                    .iter()
                    .map(|bal| TokenBalanceRecord {
                        account_index: Some(bal.account_index as i64),
                        mint: Some(bal.mint.to_owned()),
                        owner: Some(bal.owner.to_owned()),
                        amount: bal
                            .ui_token_amount
                            .as_ref()
                            .map(|ui_token| ui_token.amount.to_owned()),
                        decimals: bal
                            .ui_token_amount
                            .to_owned()
                            .map(|ui_token| ui_token.decimals as i64),
                    })
                    .collect();

                let err = &meta.err;
                let transaction_record = TransactionRecord {
                    block_slot: Some(etl_block.slot as i64),
                    block_hash: Some(table_context.block_hash.to_owned()),
                    block_timestamp: block_timestamp.to_owned(),
                    recent_block_hash,
                    signature: tx_lead_signature.clone(),
                    index: Some(tx_index as i64),
                    fee: Some(meta.fee),
                    status: Some(
                        err.as_ref()
                            .map_or(String::from("Success"), |_| String::from("Failure")),
                    ),
                    err: err.as_ref().map(|e| e.err.to_owned()),
                    compute_units_consumed: meta.compute_units_consumed,
                    accounts: transaction_account_records,
                    log_messages: meta.log_messages.to_owned(),
                    balance_changes,
                    pre_token_balances,
                    post_token_balances,
                };
                transaction_records.push(transaction_record);

                for inner_instructions in &meta.inner_instructions {
                    let parent_index = inner_instructions.index;
                    // this will be updated at the end of each iteration to be the current inner_instruction
                    let mut prev_inner_instruction = None;

                    instruction_records.reserve_exact(inner_instructions.instructions.len());
                    for (inner_index, inner_instruction) in
                        inner_instructions.instructions.iter().enumerate()
                    {
                        let instruction_record = instruction::transform_to_instruction_record(
                            common_table_fields.to_owned(),
                            inner_instruction,
                            Some(parent_index),
                            inner_index,
                        );
                        instruction_records.push(instruction_record);

                        // token transfers
                        let (instruction_type, token_transfer_instruction) = inner_instruction
                            .parsed_dict
                            .to_owned()
                            .map(|p| (p.r#type, p.token_transfer_instruction))
                            .unwrap_or((None, None));
                        if let Some(r#type) = instruction_type.to_owned() {
                            let token_transfer_record =
                                token_transfer::transform_to_token_transfer_record(
                                    common_table_fields.to_owned(),
                                    &r#type,
                                    inner_instruction.program.to_owned().unwrap_or_default(),
                                    token_transfer_instruction,
                                    prev_inner_instruction,
                                );
                            if let Some(tt) = token_transfer_record {
                                token_transfer_records.push(tt);
                            };
                        }
                        prev_inner_instruction = Some(inner_instruction.to_owned());
                    }
                }
            }
            None => {
                let transaction_record = TransactionRecord {
                    block_slot,
                    block_hash: block_hash.clone(),
                    block_timestamp: block_timestamp.to_owned(),
                    recent_block_hash,
                    signature: tx_lead_signature.clone(),
                    index: Some(tx_index as i64),
                    fee: None,
                    status: None,
                    err: None,
                    compute_units_consumed: None,
                    accounts: transaction_account_records,
                    log_messages: Vec::new(),
                    balance_changes: Vec::new(),
                    pre_token_balances: Vec::new(),
                    post_token_balances: Vec::new(),
                };
                transaction_records.push(transaction_record);
            }
        }
    }

    (
        transaction_records,
        instruction_records,
        token_transfer_records,
    )
}
