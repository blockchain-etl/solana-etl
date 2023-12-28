package transformation

import (
	"encoding/json"
	"fmt"
	"strconv"
	pbcodegen "subscriber_rabbitmq/pbcodegen"
)

func TransformToTransactionRecords(rawBlock *pbcodegen.EtlBlock) ([]*pbcodegen.TransactionRecord, []*pbcodegen.InstructionRecord, []*pbcodegen.TokenTransferRecord) {
	var TransactionRecords []*pbcodegen.TransactionRecord
	var InstructionRecords []*pbcodegen.InstructionRecord
	var TokenTransferRecords []*pbcodegen.TokenTransferRecord

	blockContext := rawBlock.TableContext

	blockSlot := int64(rawBlock.Slot)
	blockHash := blockContext.BlockHash

	var blockTimestamp *int64
	if rawBlock.TableContext.BlockTimestamp != nil {
		_timestamp := rawBlock.TableContext.BlockTimestamp.Timestamp * 1e6
		blockTimestamp = &_timestamp
	}

	for transaction_index, transaction := range rawBlock.Transactions {
		txLeadSignature := ""
		var recentBlockHash *string
		var transaction_accounts []*pbcodegen.CompiledAccount
		var transactionAccountRecords []*pbcodegen.TransactionAccountRecord

		if transaction.Transaction != nil {
			transaction_raw := transaction.Transaction

			if signatures := transaction_raw.GetSignatures(); len(signatures) > 0 {
				txLeadSignature = signatures[0]
			}
			transaction_message := transaction_raw.GetMessage()

			if transaction_message != nil {
				transaction_accounts = transaction_message.GetAccountKeys()
				recentBlockHash = &transaction_message.RecentBlockhash
			} else {
				recentBlockHash = nil
			}

			for _, account := range transaction_accounts {
				transactionAccountRecord := pbcodegen.TransactionAccountRecord{
					Pubkey:   &account.Pubkey,
					Signer:   &account.Signer,
					Writable: &account.Writable,
				}
				transactionAccountRecords = append(transactionAccountRecords, &transactionAccountRecord)
			}
			if len(transactionAccountRecords) == 0 {
				transactionAccountRecord := pbcodegen.TransactionAccountRecord{
					Pubkey:   nil,
					Signer:   nil,
					Writable: nil,
				}
				transactionAccountRecords = append(transactionAccountRecords, &transactionAccountRecord)
			}

			var prev_instruction *pbcodegen.InnerInstruction
			for instruction_index, instruction := range transaction_message.GetInstructions() {
				if instruction == nil {
					continue
				}
				instruction_parsed_dict := instruction.ParsedDict
				var instruction_parsed *string
				var instruction_type *string
				var paramRecords []*pbcodegen.ParamsRecord
				if instruction_parsed_dict == nil {
					instruction_parsed = instruction.ParsedString
				} else {
					instruction_type = instruction_parsed_dict.Type
					instruction_parsed = instruction_parsed_dict.Parsed

					param_keys_values := instruction_parsed_dict.Info
					if param_keys_values != nil {
//fmt.Println("the param_keys_values is", *param_keys_values)		
				var keys_values_map map[string]interface{}
						err := json.Unmarshal([]byte(*param_keys_values), &keys_values_map)
						if err != nil {
							fmt.Println("Error:", err)
							panic("Unexpected instruction params value")
						}
						if len(keys_values_map) == 0 {
							fmt.Println("the keys and values map is empty")
						}
						for k, v := range keys_values_map {
//fmt.Println("the k is:", k)
kCopy := k // Create a copy of k
							jsonBytes, err := json.Marshal(v)
							if err != nil {
								panic(err)
							}
							jsonString := string(jsonBytes)
//fmt.Println("the v string is:", jsonString)
							param_record := pbcodegen.ParamsRecord{
								Key:   &kCopy,
								Value: &jsonString,
							}
//fmt.Println("the param record is:", param_record)					
		paramRecords = append(paramRecords, &param_record)
						}
					}
				}

				accounts := make([]string, len(instruction.Accounts))
				copy(accounts, instruction.Accounts)

				if len(accounts) == 0 {
					// NOTE: there must be at least 1 "valid" nullstring in the list since the record is `repeated` in BQ.
					accounts = append(accounts, "")
				}

				if len(paramRecords) == 0 {
					paramRecords = append(paramRecords, &pbcodegen.ParamsRecord{
						Key:   nil,
						Value: nil,
					})
				}

				index := int64(instruction_index)
				InstructionRecord := pbcodegen.InstructionRecord{
					BlockSlot:       &blockSlot,
					BlockTimestamp:  blockTimestamp,
					BlockHash:       &blockHash,
					TxSignature:     &txLeadSignature,
					Index:           &index,
					ParentIndex:     nil,
					Accounts:        accounts,
					Data:            instruction.Data,
					Program:         instruction.Program,
					ProgramId:       instruction.ProgramId,
					InstructionType: instruction_type,
					Params:          paramRecords,
					Parsed:          instruction_parsed,
				}

				InstructionRecords = append(InstructionRecords, &InstructionRecord)

				var tokenTransfer *TokenTransferData
				if instruction_type != nil {
					tokenTransfer = extractTokenTransfer(instruction.GetProgram(), *instruction_type, txLeadSignature, instruction_parsed_dict, prev_instruction)
				}

				if tokenTransfer != nil {
					var value *uint64
					if tokenTransfer.value != nil && *tokenTransfer.value != "" {
						_value, err := strconv.ParseUint(*tokenTransfer.value, 10, 64)
						if err != nil {
							fmt.Println("Error:", err)
							panic("terminating...")
						} else {
							value = &_value
						}
					}

					TokenTransferRecord := pbcodegen.TokenTransferRecord{
						BlockSlot:      &blockSlot,
						BlockTimestamp: blockTimestamp,
						BlockHash:      &blockHash,
						Source:         tokenTransfer.source,
						Destination:    tokenTransfer.destination,
						Authority:      tokenTransfer.authority,
						Value:          value,
						Decimals:       tokenTransfer.decimals,
						Fee:            tokenTransfer.fee,
						FeeDecimals:    tokenTransfer.feeDecimals,
						Memo:           tokenTransfer.memo,
						Mint:           tokenTransfer.mint,
						MintAuthority:  tokenTransfer.mintAuthority,
						TransferType:   tokenTransfer.transferType,
						TxSignature:    &txLeadSignature,
					}
					TokenTransferRecords = append(TokenTransferRecords, &TokenTransferRecord)
				}
				prev_instruction = instruction
			}
		}

		tx_meta := transaction.GetMeta()

		if tx_meta == nil {
			var preTokenBalances []*pbcodegen.TokenBalanceRecord
			preTokenBalance := pbcodegen.TokenBalanceRecord{
				AccountIndex: nil,
				Mint:         nil,
				Owner:        nil,
				Amount:       nil,
				Decimals:     nil,
			}
			preTokenBalances = append(preTokenBalances, &preTokenBalance)

			var postTokenBalances []*pbcodegen.TokenBalanceRecord
			postTokenBalance := pbcodegen.TokenBalanceRecord{
				AccountIndex: nil,
				Mint:         nil,
				Owner:        nil,
				Amount:       nil,
				Decimals:     nil,
			}
			postTokenBalances = append(postTokenBalances, &postTokenBalance)

			var balanceChanges []*pbcodegen.BalanceChangeRecord
			balanceChangeRecord := pbcodegen.BalanceChangeRecord{
				Account: nil,
				Before:  nil,
				After:   nil,
			}
			balanceChanges = append(balanceChanges, &balanceChangeRecord)

			lm := []string{""}

			txIndex := int64(transaction_index)
			TransactionRecord := pbcodegen.TransactionRecord{
				Signature:            &txLeadSignature,
				BlockHash:            &blockHash,
				RecentBlockHash:      recentBlockHash,
				BlockSlot:            &blockSlot,
				BlockTimestamp:       blockTimestamp,
				Index:                &txIndex,
				Fee:                  nil,
				Status:               nil,
				Err:                  nil,
				ComputeUnitsConsumed: nil,
				Accounts:             transactionAccountRecords,
				LogMessages:          lm,
				BalanceChanges:       balanceChanges,
				PreTokenBalances:     preTokenBalances,
				PostTokenBalances:    postTokenBalances,
			}
			TransactionRecords = append(TransactionRecords, &TransactionRecord)

		} else {
			fee := tx_meta.GetFee()
			tx_err := tx_meta.GetErr()
			var status string
			if tx_err == nil {
				status = "Success"
			} else {
				status = "Fail"
			}
			errString := tx_err.GetErr()

			lm := tx_meta.GetLogMessages()
			var log_messages []string
			if len(lm) > 0 {
				log_messages = append(log_messages, lm...)
			} else {
				log_messages = append(log_messages, "")
			}

			pre_balances := tx_meta.GetPreBalances()
			post_balances := tx_meta.GetPostBalances()

			var balanceChanges []*pbcodegen.BalanceChangeRecord
			for i, account := range transaction_accounts {
				if i >= len(pre_balances) || i >= len(post_balances) {
					break
				}
				pre_balance := pre_balances[i]
				post_balance := post_balances[i]
				pubkey := account.GetPubkey()
				balanceChange := pbcodegen.BalanceChangeRecord{
					Account: &pubkey,
					Before:  &pre_balance,
					After:   &post_balance,
				}
				balanceChanges = append(balanceChanges, &balanceChange)
			}

			var preTokenBalances []*pbcodegen.TokenBalanceRecord
			for _, preTokenBalance := range tx_meta.GetPreTokenBalances() {
				uiTokenAmount := preTokenBalance.GetUiTokenAmount()
				accountIndex := int64(preTokenBalance.AccountIndex)
				decimals := int64(uiTokenAmount.Decimals)
				preTokenBalanceRecord := pbcodegen.TokenBalanceRecord{
					AccountIndex: &accountIndex,
					Mint:         &preTokenBalance.Mint,
					Owner:        &preTokenBalance.Owner,
					Amount:       &uiTokenAmount.Amount,
					Decimals:     &decimals,
				}
				preTokenBalances = append(preTokenBalances, &preTokenBalanceRecord)
			}
			if len(preTokenBalances) == 0 {
				preTokenBalances = append(preTokenBalances, &pbcodegen.TokenBalanceRecord{AccountIndex: nil, Mint: nil, Owner: nil, Amount: nil, Decimals: nil})
			}

			var postTokenBalances []*pbcodegen.TokenBalanceRecord
			for _, postTokenBalance := range tx_meta.GetPostTokenBalances() {
				uiTokenAmount := postTokenBalance.GetUiTokenAmount()
				accountIndex := int64(postTokenBalance.AccountIndex)
				decimals := int64(uiTokenAmount.Decimals)
				postTokenBalanceRecord := pbcodegen.TokenBalanceRecord{
					AccountIndex: &accountIndex,
					Mint:         &postTokenBalance.Mint,
					Owner:        &postTokenBalance.Owner,
					Amount:       &uiTokenAmount.Amount,
					Decimals:     &decimals,
				}
				postTokenBalances = append(postTokenBalances, &postTokenBalanceRecord)
			}
			if len(postTokenBalances) == 0 {
				postTokenBalances = append(postTokenBalances, &pbcodegen.TokenBalanceRecord{AccountIndex: nil, Mint: nil, Owner: nil, Amount: nil, Decimals: nil})
			}

			if !tx_meta.GetInnerInstructionsNone() {
				for _, inner_instructions := range tx_meta.GetInnerInstructions() {
					var prev_inner_instruction *pbcodegen.InnerInstruction
					for j, inner_instruction := range inner_instructions.GetInstructions() {
						if inner_instruction == nil {
							continue
						}
						instruction_parsed_dict := inner_instruction.ParsedDict
						var instruction_parsed *string
						var instruction_type *string
						var param_records []*pbcodegen.ParamsRecord
						if instruction_parsed_dict == nil {
							instruction_type = nil
							instruction_parsed = inner_instruction.ParsedString
							param_record := pbcodegen.ParamsRecord{
								Key:   nil,
								Value: nil,
							}
							param_records = append(param_records, &param_record)

						} else {
							instruction_type = instruction_parsed_dict.Type
							instruction_parsed = instruction_parsed_dict.Parsed

							param_keys_values := instruction_parsed_dict.Info
							if param_keys_values == nil {
								param_records = append(param_records, &pbcodegen.ParamsRecord{
									Key:   nil,
									Value: nil,
								})
							} else {
								var keys_values_map map[string]interface{}
								err := json.Unmarshal([]byte(*param_keys_values), &keys_values_map)
								if err != nil {
									fmt.Println("Error:", err)
									panic("Unexpected instruction params value")
								}
								for k, v := range keys_values_map {
kCopy := k // Create a copy of k
									jsonBytes, err := json.Marshal(v)
									if err != nil {
										panic(err)
									}
									jsonString := string(jsonBytes)

//fmt.Println("the inner k is:", k)
//fmt.Println("the inner v string is:", jsonString)
									param_record := pbcodegen.ParamsRecord{
										Key:   &kCopy,
										Value: &jsonString,
									}
//fmt.Println("the inner param_record is:", param_record)					
				param_records = append(param_records, &param_record)
								}
							}
						}

						var accounts []string
						if len(inner_instruction.Accounts) > 0 {
							accounts = append(accounts, inner_instruction.Accounts...)
						} else {
							accounts = append(accounts, "")
						}

						innerIndex := int64(j)
						parentIndex := int64(inner_instructions.Index)
						InstructionRecord := pbcodegen.InstructionRecord{
							BlockSlot:       &blockSlot,
							BlockTimestamp:  blockTimestamp,
							BlockHash:       &blockHash,
							TxSignature:     &txLeadSignature,
							Index:           &innerIndex,
							ParentIndex:     &parentIndex,
							Accounts:        accounts,
							Data:            inner_instruction.Data,
							Program:         inner_instruction.Program,
							ProgramId:       inner_instruction.ProgramId,
							InstructionType: instruction_type,
							Params:          param_records,
							Parsed:          instruction_parsed,
						}

						InstructionRecords = append(InstructionRecords, &InstructionRecord)
						var tokenTransfer *TokenTransferData
						if instruction_type != nil {
							tokenTransfer = extractTokenTransfer(inner_instruction.GetProgram(), *instruction_type, txLeadSignature, instruction_parsed_dict, prev_inner_instruction)
						}

						if tokenTransfer != nil {
							var value *uint64
							if tokenTransfer.value != nil && *tokenTransfer.value != "" {
								_value, err := strconv.ParseUint(*tokenTransfer.value, 10, 64)
								if err != nil {
									fmt.Println("Error:", err)
									panic("terminating...")
								} else {
									value = &_value
								}
							}
							tokenTransfer_record := pbcodegen.TokenTransferRecord{
								BlockSlot:      &blockSlot,
								BlockTimestamp: blockTimestamp,
								BlockHash:      &blockHash,
								Source:         tokenTransfer.source,
								Destination:    tokenTransfer.destination,
								Authority:      tokenTransfer.authority,
								Value:          value,
								Decimals:       tokenTransfer.decimals,
								Mint:           tokenTransfer.mint,
								MintAuthority:  tokenTransfer.mintAuthority,
								TransferType:   tokenTransfer.transferType,
								TxSignature:    &txLeadSignature,
							}
							TokenTransferRecords = append(TokenTransferRecords, &tokenTransfer_record)
						}
						prev_inner_instruction = inner_instruction
					}

				}
			}

			txIndex := int64(transaction_index)
			TransactionRecord := pbcodegen.TransactionRecord{
				BlockSlot:            &blockSlot,
				BlockTimestamp:       blockTimestamp,
				BlockHash:            &blockHash,
				Signature:            &txLeadSignature,
				RecentBlockHash:      recentBlockHash,
				Index:                &txIndex,
				Fee:                  &fee,
				Status:               &status,
				Err:                  &errString,
				ComputeUnitsConsumed: tx_meta.ComputeUnitsConsumed,
				Accounts:             transactionAccountRecords,
				LogMessages:          log_messages,
				BalanceChanges:       balanceChanges,
				PreTokenBalances:     preTokenBalances,
				PostTokenBalances:    postTokenBalances,
			}
			TransactionRecords = append(TransactionRecords, &TransactionRecord)
		}
	}
	return TransactionRecords, InstructionRecords, TokenTransferRecords
}
