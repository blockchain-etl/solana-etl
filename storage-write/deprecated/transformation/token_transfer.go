package transformation

import (
	"strconv"
	"subscriber_rabbitmq/pbcodegen"
)

type TokenTransferData struct {
	source        *string
	destination   *string
	authority     *string
	value         *string
	fee           *uint64
	feeDecimals   *uint64
	memo          *string
	decimals      *uint64
	mint          *string
	mintAuthority *string
	transferType  *string
	txSignature   string
}

func extractTokenTransfer(program string, instruction_type string, tx_signature string, token_instruction *pbcodegen.Parsed, prev_instruction *pbcodegen.InnerInstruction) *TokenTransferData {
	/*
		NOTE: the program ID for token2022 is `TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb`
		but the program is still spl-token, like the existing spl-token (pre-2022)
	*/

	const TOKEN_PROGRAM = "spl-token" // this is the program for both types of token (spl and spl-2022)
	const MEMO_PROGRAM = "spl-memo"   // this is the program for both types of memo (v1 and v2)
	const SYSTEM_PROGRAM = "system"
	const TRANSFER_CHECKED = "transferChecked"
	const TRANSFER_CHECKED_WITH_FEE = "transferCheckedWithFee"
	const BURN_CHECKED = "burnChecked"
	const MINT_TO_CHECKED = "mintToChecked"
	//const MEMO_V1_ACCOUNT = "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo"
	//const MEMO_V2_ACCOUNT = "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
	BURN := "burn"
	SPL_TRANSFER := "spl-transfer"
	SPL_TRANSFER_WITH_FEE := "spl-transfer-with-fee"
	MINT_TO := "mintTo"
	TRANSFER := "transfer"

	token_instruction_generated := token_instruction.GetTokenTransferInstruction()
	if token_instruction_generated == nil {
		return nil
	}

	/*
		"all incoming transfers must have an accompanying memo instruction right before the transfer instruction."
		source: https://spl.solana.com/token-2022/extensions#required-memo-on-transfer
	*/
	var memo *string

	if prev_instruction != nil && prev_instruction.Program != nil {
		if *prev_instruction.Program == MEMO_PROGRAM {
			if prev_instruction.Data != nil && *prev_instruction.Data != "" {
				panic("Memo data should be parsed, because of jsonParsed block encoding")
			} else if prev_instruction.ParsedString != nil && *prev_instruction.ParsedString != "" {
				memo = prev_instruction.ParsedString
			} else if prev_instruction.ParsedDict != nil {
				memo = prev_instruction.ParsedDict.Info
			}
		}
	}

	var valueConv *string
	if token_instruction_generated.Amount == nil {
		valueConv = nil
	} else {
		_valueConv := strconv.FormatUint(*token_instruction_generated.Amount, 10)
		valueConv = &_valueConv
	}

	if program == TOKEN_PROGRAM {
		switch instruction_type {
		case TRANSFER:
			return &TokenTransferData{
				source:        token_instruction_generated.Source,
				destination:   token_instruction_generated.Destination,
				authority:     token_instruction_generated.Authority,
				value:         valueConv,
				decimals:      nil,
				fee:           token_instruction_generated.FeeAmount,
				feeDecimals:   token_instruction_generated.FeeAmountDecimals,
				memo:          memo,
				mint:          nil,
				mintAuthority: token_instruction_generated.MintAuthority,
				transferType:  &SPL_TRANSFER,
				txSignature:   tx_signature,
			}

		case TRANSFER_CHECKED:
			return &TokenTransferData{
				source:        token_instruction_generated.Source,
				destination:   token_instruction_generated.Destination,
				authority:     token_instruction_generated.Authority,
				value:         token_instruction_generated.TokenAmount,
				decimals:      token_instruction_generated.TokenAmountDecimals,
				fee:           token_instruction_generated.FeeAmount,
				feeDecimals:   token_instruction_generated.FeeAmountDecimals,
				memo:          memo,
				mint:          token_instruction_generated.Mint,
				mintAuthority: nil,
				transferType:  &SPL_TRANSFER,
				txSignature:   tx_signature,
			}
		case TRANSFER_CHECKED_WITH_FEE:
			return &TokenTransferData{
				source:        token_instruction_generated.Source,
				destination:   token_instruction_generated.Destination,
				authority:     token_instruction_generated.Authority,
				value:         token_instruction_generated.TokenAmount,
				decimals:      token_instruction_generated.TokenAmountDecimals,
				fee:           token_instruction_generated.FeeAmount,
				feeDecimals:   token_instruction_generated.FeeAmountDecimals,
				memo:          memo,
				mint:          token_instruction_generated.Mint,
				mintAuthority: nil,
				transferType:  &SPL_TRANSFER_WITH_FEE,
				txSignature:   tx_signature,
			}
		case BURN:
			return &TokenTransferData{
				source:        nil,
				destination:   nil,
				authority:     token_instruction_generated.Authority,
				value:         valueConv,
				decimals:      nil,
				fee:           token_instruction_generated.FeeAmount,
				feeDecimals:   token_instruction_generated.FeeAmountDecimals,
				memo:          memo,
				mint:          token_instruction_generated.Mint,
				mintAuthority: nil,
				transferType:  &BURN,
				txSignature:   tx_signature,
			}
		case BURN_CHECKED:
			return &TokenTransferData{
				source:        nil,
				destination:   nil,
				authority:     token_instruction_generated.Authority,
				value:         valueConv,
				decimals:      token_instruction_generated.Decimals,
				fee:           token_instruction_generated.FeeAmount,
				feeDecimals:   token_instruction_generated.FeeAmountDecimals,
				memo:          memo,
				mint:          token_instruction_generated.Mint,
				mintAuthority: nil,
				transferType:  &BURN,
				txSignature:   tx_signature,
			}
		case MINT_TO:
			return &TokenTransferData{
				source:        nil,
				destination:   nil,
				authority:     nil,
				value:         valueConv,
				decimals:      nil,
				fee:           token_instruction_generated.FeeAmount,
				feeDecimals:   token_instruction_generated.FeeAmountDecimals,
				memo:          memo,
				mint:          token_instruction_generated.Mint,
				mintAuthority: token_instruction_generated.MintAuthority,
				transferType:  &MINT_TO,
				txSignature:   tx_signature,
			}
		case MINT_TO_CHECKED:
			return &TokenTransferData{
				source:        nil,
				destination:   nil,
				authority:     nil,
				value:         valueConv,
				decimals:      token_instruction_generated.Decimals, // solana-etl only does this for mint-checked.
				fee:           token_instruction_generated.FeeAmount,
				feeDecimals:   token_instruction_generated.FeeAmountDecimals,
				memo:          memo,
				mint:          token_instruction_generated.Mint,
				mintAuthority: token_instruction_generated.MintAuthority,
				transferType:  &MINT_TO,
				txSignature:   tx_signature,
			}
		default:
			return nil
		}
	} else if program == SYSTEM_PROGRAM && instruction_type == TRANSFER {

		return &TokenTransferData{
			source:        token_instruction_generated.Source,
			destination:   token_instruction_generated.Destination,
			authority:     token_instruction_generated.Authority,
			value:         valueConv,
			decimals:      nil,
			fee:           nil,
			feeDecimals:   nil,
			memo:          memo,
			mint:          nil,
			mintAuthority: nil,
			transferType:  &TRANSFER,
			txSignature:   tx_signature,
		}
	} else {
		return nil
	}
}
