package transformation

import "subscriber_rabbitmq/pbcodegen"

// returns a single block record.
// NOTE: the output is an array, but it will only contain a single element. this is for compatibility with bigquery insertion.
func TransformToBlockRecord(rawBlock *pbcodegen.EtlBlock) []*pbcodegen.BlockRecord {
	var blockRecord []*pbcodegen.BlockRecord

	slot := int64(rawBlock.Slot)

	blockData := rawBlock.GetBlock()
	var blockHeight *int64
	if blockData.BlockHeight != nil {
		_blockHeight := int64(blockData.BlockHeight.BlockHeight)
		blockHeight = &_blockHeight
	}
	blockHash := blockData.Blockhash
	previousBlockHash := blockData.PreviousBlockhash

	var blockTimestamp *int64
	if rawBlock.TableContext.BlockTimestamp != nil {
		_timestamp := rawBlock.TableContext.BlockTimestamp.Timestamp * 1e6
		blockTimestamp = &_timestamp
	}

	transactionCount := int64(blockData.TransactionCount)

	leaderReward := int64(blockData.LeaderReward)
	leader := blockData.Leader

	blockRecord = append(blockRecord, &pbcodegen.BlockRecord{Slot: &slot, Height: blockHeight, BlockHash: &blockHash, PreviousBlockHash: &previousBlockHash, BlockTimestamp: blockTimestamp, TransactionCount: &transactionCount, LeaderReward: &leaderReward, Leader: &leader})
	return blockRecord
}
