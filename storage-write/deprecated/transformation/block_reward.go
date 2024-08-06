package transformation

import (
	"fmt"
	"strconv"
	"subscriber_rabbitmq/pbcodegen"
)

func TransformToBlockRewardRecords(rawBlock *pbcodegen.EtlBlock) []*pbcodegen.BlockRewardRecord {
	var blockRewardRecords []*pbcodegen.BlockRewardRecord
	blockSlot := int64(rawBlock.Slot)
	blockHash := rawBlock.TableContext.BlockHash

	var blockTimestamp *int64
	if rawBlock.TableContext.BlockTimestamp != nil {
		_timestamp := rawBlock.TableContext.BlockTimestamp.Timestamp * 1e6
		blockTimestamp = &_timestamp
	}

	for _, reward := range rawBlock.BlockRewards {
		var commission *uint64
		if reward.Commission != "" {
			_commission, err := strconv.ParseUint(reward.Commission, 10, 64)
			if err != nil {
				fmt.Println("Error:", err)
				panic("terminating...")
			} else {
				commission = &_commission
			}
		}

		lamports := int64(reward.Lamports)
		rewardType := reward.RewardType.String()
		blockRewardRecord := pbcodegen.BlockRewardRecord{
			BlockSlot:      &blockSlot,
			BlockHash:      &blockHash,
			BlockTimestamp: blockTimestamp,
			Commission:     commission,
			Lamports:       &lamports, // NOTE: solana's official protocol buffers make this field an unsigned integer, but the value can actually be negative, so we cast it to a signed integer.
			PostBalance:    &reward.PostBalance,
			Pubkey:         &reward.Pubkey,
			RewardType:     &rewardType,
		}
		blockRewardRecords = append(blockRewardRecords, &blockRewardRecord)
	}

	return blockRewardRecords
}
