# Solana ETL (was solana-config)

NOTE: You may be looking for the previous iteration of `solana-etl`, written in Python. That code has been moved to [here](https://github.com/blockchain-etl/solana-etl-airflow).

This repository contains all of the code for running a Solana ETL pipeline. The primary purpose of this is to serve data for Google BigQuery, but outputs for Google Pub/Sub, RabbitMQ, RabbitMQ Stream, JSON files, and JSONL files are supported.

For more information, please check the [documentation](/docs/).

# Setup
Use the script in `scripts/setup.sh` to automatically install system dependencies, clone the repo and all submodules, and compile:
-  Tested on Ubuntu LTS 22.04
```
bash scripts/setup.sh
```
NOTE: you may need to run with `sudo`.

Next, build and run the development profile (default) with appropriate features:

E.g. to output to Google Pub/Sub or to output JSON files, replace `<OUTPUT_TYPE>` with `GOOGLE_PUBSUB` or `JSON`, respectively:

`cargo build --features <OUTPUT_TYPE>`

Finally, execute with the appropriate function and parameters.

E.g. Index starting from genesis onwards:

`./target/debug/blockchain_etl_indexer index-range stream 0`

Or to index from genesis to block 10:
`./target/debug/blockchain_etl_indexer index-range stream 0 10`

And to index a list of specific blocks, provide a CSV filepath with `index-list` command:
`./target/debug/blockchain_etl_indexer index-list stream FILE_PATH.csv`
