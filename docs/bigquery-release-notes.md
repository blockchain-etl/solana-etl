# BigQuery Release Notes

Solana Foundation's BigTable was used for the initial data upload into the BigQuery Public Dataset. However, the BigTable is known to be missing a small number of blocks. We have calculated this number using the blockheight and record count to be 13,602 blocks, which is about 0.006% of all blocks at the time of writing. Additionally, a small number of blocks are missing or have duplicate transactions due to stops and starts during the initial data upload. We have calculated this number to be 18,879 blocks, or 0.009% of all blocks at the time of writing.
