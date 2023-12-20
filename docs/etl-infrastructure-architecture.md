# ETL Infrastructure Architecture

## Architecture Framework
The overall infrastructure is depicted below.

![architecture](/docs/img/architecture.png)


## Macro Infrastructure
An RPC node is expected to serve requests for account and token data. Block data (which includes the data for the blocks, block rewards, transactions instructions, and token transfers tables) can optionally be provided using the Solana Foundation's BigTable if an authorization key is available, otherwise the RPC node can be used for this. Upon response from the data source, the data is converted into a Protocol Buffers data format and either sent to a streaming queue such as Google Cloud Pub/Sub or RabbitMQ, or written to JSON or JSONL files.

The detailed extraction process is explained in the [extraction](/docs/extraction.md) document.
