package transformation

import (
	"fmt"
	"strconv"

	"subscriber_rabbitmq/pbcodegen"
)

func TransformToAccountRecords(rawBlock *pbcodegen.EtlBlock) ([]*pbcodegen.AccountRecord, []*pbcodegen.TokenRecord) {
	var accountRecords []*pbcodegen.AccountRecord
	var tokenRecords []*pbcodegen.TokenRecord

	blockSlot := int64(rawBlock.Slot)
	blockHash := rawBlock.TableContext.BlockHash

	var blockTimestamp *int64
	if rawBlock.TableContext.BlockTimestamp != nil {
		_timestamp := rawBlock.TableContext.BlockTimestamp.Timestamp * 1e6
		blockTimestamp = &_timestamp
	}
	// first accounts, and later tokens
	for _, accounts_data := range rawBlock.Accounts {
		for _, account := range accounts_data.GetAccounts() {
			var retrievalTimestamp *int64
			if account.RetrievalTimestamp != nil {
				_timestamp := account.RetrievalTimestamp.Timestamp * 1e6
				retrievalTimestamp = &_timestamp
			}

			authorizedVoters := make([]*pbcodegen.AuthorizedVoterRecord, 0, len(account.AuthorizedVoters))
			for _, authorizedVoter := range account.AuthorizedVoters {
				if authorizedVoter != nil {
					authorizedVoters = append(authorizedVoters, &pbcodegen.AuthorizedVoterRecord{
						AuthorizedVoter: &authorizedVoter.AuthorizedVoter,
						Epoch:           &authorizedVoter.Epoch,
					})
				}
			}

			if len(authorizedVoters) == 0 {
				authorizedVoters = append(authorizedVoters, &pbcodegen.AuthorizedVoterRecord{AuthorizedVoter: nil, Epoch: nil})
			}

			priorVoters := make([]*pbcodegen.PriorVoterRecord, 0, len(account.PriorVoters))
			for _, priorVoter := range account.PriorVoters {
				if priorVoter != nil {
					epochOfLastAuthorizedSwitch := int64(priorVoter.EpochOfLastAuthorizedSwitch)
					targetEpoch := int64(priorVoter.TargetEpoch)
					priorVoters = append(priorVoters, &pbcodegen.PriorVoterRecord{
						AuthorizedPubkey:            &priorVoter.AuthorizedPubkey,
						EpochOfLastAuthorizedSwitch: &epochOfLastAuthorizedSwitch,
						TargetEpoch:                 &targetEpoch,
					})
				}
			}

			if len(priorVoters) == 0 {
				priorVoters = append(priorVoters, &pbcodegen.PriorVoterRecord{AuthorizedPubkey: nil, EpochOfLastAuthorizedSwitch: nil, TargetEpoch: nil})
			}

			epochCredits := make([]*pbcodegen.EpochCreditRecord, 0, len(account.EpochCredits))
			if account.Votes != nil {
				for _, epochCredit := range account.EpochCredits {
					if epochCredit != nil {
						epochCredits = append(epochCredits, &pbcodegen.EpochCreditRecord{Credits: &epochCredit.Credits, Epoch: &epochCredit.Epoch, PreviousCredits: &epochCredit.PreviousCredits})
					}
				}
			}
			if len(epochCredits) == 0 {
				epochCredits = append(epochCredits, &pbcodegen.EpochCreditRecord{Credits: nil, Epoch: nil, PreviousCredits: nil})
			}

			votes := make([]*pbcodegen.VoteRecord, 0, len(account.Votes))
			if account.Votes != nil {
				for _, vote := range account.Votes {
					if vote != nil {
						votes = append(votes, &pbcodegen.VoteRecord{
							ConfirmationCount: &vote.ConfirmationCount,
							Slot:              &vote.Slot,
						})
					}
				}
			}

			if len(votes) == 0 {
				votes = append(votes, &pbcodegen.VoteRecord{ConfirmationCount: nil, Slot: nil})
			}

			var lastTimestamp []*pbcodegen.TimestampRecord
			if account.LastTimestamp == nil {
				lastTimestamp = append(lastTimestamp, &pbcodegen.TimestampRecord{Timestamp: nil, Slot: nil})
			} else {
				_timestamp := account.LastTimestamp.Timestamp * 1e6
				lastTimestamp = append(lastTimestamp, &pbcodegen.TimestampRecord{Timestamp: &_timestamp, Slot: &account.LastTimestamp.Slot})
			}

			var dataRecord []*pbcodegen.DataRecord
			if account.Data == nil {
				dataRecord = append(dataRecord, &pbcodegen.DataRecord{Raw: nil, Encoding: nil})
			} else {
				dataRecord = append(dataRecord, &pbcodegen.DataRecord{Raw: &account.Data.Raw, Encoding: &account.Data.Encoding})
			}

			rentEpoch := int64(account.RentEpoch)
			var tokenAmount *uint64
			if account.TokenAmount != nil && *account.TokenAmount != "" {
				_tokenAmount, err := strconv.ParseUint(*account.TokenAmount, 10, 64)
				if err != nil {
					fmt.Println("Error:", err)
					panic("terminating...")
				} else {
					tokenAmount = &_tokenAmount
				}
			}
			account_record := pbcodegen.AccountRecord{
				BlockSlot:            &blockSlot,
				BlockTimestamp:       blockTimestamp,
				BlockHash:            &blockHash,
				TxSignature:          accounts_data.TxSignature,
				RetrievalTimestamp:   retrievalTimestamp,
				Pubkey:               &account.Pubkey,
				Executable:           &account.Executable,
				Lamports:             &account.Lamports,
				Owner:                account.Owner,
				RentEpoch:            &rentEpoch,
				Program:              account.Program,
				Space:                account.Space,
				AccountType:          account.AccountType,
				IsNative:             account.IsNative,
				Mint:                 account.Mint,
				State:                account.State,
				TokenAmount:          tokenAmount,
				TokenAmountDecimals:  account.TokenAmountDecimals,
				ProgramData:          account.ProgramData,
				AuthorizedVoters:     authorizedVoters,
				AuthorizedWithdrawer: account.AuthorizedWithdrawer,
				PriorVoters:          priorVoters,
				NodePubkey:           account.NodePubkey,
				Commission:           account.Commission,
				EpochCredits:         epochCredits,
				Votes:                votes,
				RootSlot:             account.RootSlot,
				LastTimestamp:        lastTimestamp,
				Data:                 dataRecord,
			}
			accountRecords = append(accountRecords, &account_record)
		}

		for _, token := range accounts_data.GetTokens() {
			var retrievalTimestamp *int64
			if token.RetrievalTimestamp != nil {
				_timestamp := token.RetrievalTimestamp.Timestamp * 1e6
				retrievalTimestamp = &_timestamp
			}

			var creators []*pbcodegen.CreatorRecord
			if token.Creators == nil {
				creators = append(creators, &pbcodegen.CreatorRecord{Address: nil, Verified: nil, Share: nil})
			} else {
				for _, creator := range token.Creators {
					share := int64(creator.Share)
					creators = append(creators, &pbcodegen.CreatorRecord{Address: &creator.Address, Verified: &creator.Verified, Share: &share})
				}
			}

			tokenRecord := pbcodegen.TokenRecord{
				BlockSlot:            &blockSlot,
				BlockTimestamp:       blockTimestamp,
				BlockHash:            &blockHash,
				TxSignature:          accounts_data.TxSignature,
				RetrievalTimestamp:   retrievalTimestamp,
				IsNft:                &token.IsNft,
				Mint:                 &token.Mint,
				UpdateAuthority:      &token.UpdateAuthority,
				Name:                 &token.Name,
				Symbol:               &token.Symbol,
				Uri:                  &token.Uri,
				SellerFeeBasisPoints: &token.SellerFeeBasisPoints,
				Creators:             creators,
				PrimarySaleHappened:  &token.PrimarySaleHappened,
				IsMutable:            &token.IsMutable,
			}
			tokenRecords = append(tokenRecords, &tokenRecord)

		}
	}
	return accountRecords, tokenRecords
}
