# Getting Started
## Install System Dependencies
Tested on Ubuntu LTS 22.04:
```
sudo apt install git cargo g++ protobuf-compiler
```
## Clone the Repo
```
git clone https://github.com/blockchain-etl/solana-etl.git
```
## Compile the Code
```
cd solana-etl
cargo build –-release --features <OUTPUT>
```
NOTE: you must replace `<OUTPUT>` in the above command with one of the supported output types, depending on how you would like to run the indexer. The supported outputs are:
1. `JSON`
2. `JSONL`
3. `GOOGLE_PUBSUB`
4. `RABBITMQ_CLASSIC`
5. `RABBITMQ_STREAM`

If you would like to upload the records as files to GCS buckets, then you should use `JSONL` as output, and you can run [this script](/scripts/upload_to_gcs.sh) to continually upload them.

If you would like to write the records to BigQuery using the Storage Write API, then you should use `RABBITMQ_CLASSIC` as output, and setup a RabbitMQ instance using the scripts in the iac directory. That documentation is available [here](/iac/README.md) and the Storage Write API scripts are available [here](/storage-write).

## Configure the Environment Variables
See the [documentation on environment variables](/docs/environment-variables.md).

## Run the Indexer
There are two CLI options to choose from:
1. `index-range`
2. `index-list`
Option 1 requires that you pass a starting slot to index from, and you can optionally provide a second slot as the ending index. The start is inclusive, and the end is exclusive.

Option 2 requires that you pass the path to a CSV file containing a list of specified slots to index.

As an example, if you would like to index from the genesis block onwards, you can run the following command:
```
RUST_LOG=WARN ./target/release/blockchain_etl_indexer index-range stream 0
```
NOTE: `RUST_LOG` specifies the logging level. For more information, see the `log` crate and [its logging levels](https://docs.rs/log/latest/log/enum.Level.html).
