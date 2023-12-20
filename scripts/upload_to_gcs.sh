#!/bin/bash

# Loop to continually upload JSONL record files to GCS
while true; do
    gcloud storage mv output/instructions/*.jsonl gs://solana_instructions/
    gcloud storage mv output/transactions/*.jsonl gs://solana_transactions/
    gcloud storage mv output/blocks/*.json gs://solana_blocks/
    gcloud storage mv output/block_rewards/*.jsonl gs://solana_block_rewards/
    gcloud storage mv output/tokens/*.jsonl gs://solana_tokens/
    gcloud storage mv output/token_transfers/*.jsonl gs://solana_token_transfers/
    gcloud storage mv output/accounts/*.jsonl gs://solana_accounts/
done
