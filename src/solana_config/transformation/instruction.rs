use serde_json::Value;

use crate::solana_config::proto_codegen::confirmed_block::InnerInstruction;

#[cfg(feature = "STRING_TIMESTAMP")]
use crate::solana_config::proto_codegen::records_string_timestamp::{
    InstructionRecord, ParamsRecord,
};

#[cfg(feature = "INT_TIMESTAMP")]
use crate::solana_config::proto_codegen::records_int_timestamp::{InstructionRecord, ParamsRecord};

use super::transaction::CommonTableFields;

pub fn transform_to_instruction_record(
    common_table_fields: CommonTableFields,
    instruction: &InnerInstruction,
    parent_index: Option<u32>,
    inner_index: usize,
) -> InstructionRecord {
    let instruction_type: Option<String>;
    let instruction_parsed: Option<String>;
    let param_records: Vec<ParamsRecord>;
    match &instruction.parsed_dict {
        None => {
            instruction_type = None;
            instruction_parsed = instruction.parsed_string.to_owned();
            param_records = Vec::new();
        }
        Some(parsed) => {
            instruction_type = parsed.r#type.to_owned();
            instruction_parsed = parsed.parsed.to_owned();

            match &parsed.info {
                None => {
                    param_records = Vec::new();
                }
                Some(params) => {
                    let params_deserialized: Value = serde_json::from_str(params).unwrap();
                    param_records = match &params_deserialized {
                        Value::Object(o) => o
                            .iter()
                            .map(|(key, value)| ParamsRecord {
                                key: Some(key.to_owned()),
                                value: Some(value.to_string()),
                            })
                            .collect(),
                        _ => panic!("instruction parameters are not a key-value pair"),
                    };
                }
            }
        }
    }
    InstructionRecord {
        block_slot: common_table_fields.block_slot,
        block_hash: common_table_fields.block_hash,
        block_timestamp: common_table_fields.block_timestamp,
        tx_signature: common_table_fields.tx_signature,
        index: Some(inner_index as i64),
        parent_index: parent_index.map(|i| i as i64),
        accounts: instruction.accounts.to_owned(),
        data: instruction.data.to_owned(),
        parsed: instruction_parsed,
        program: instruction.program.to_owned(),
        program_id: instruction.program_id.to_owned(),
        instruction_type: instruction_type.to_owned(),
        params: param_records,
    }
}
