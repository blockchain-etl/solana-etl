# Features

## Synopsis

You can either define `--features` in the `Cargo.toml` file specify them as part of a command.

`cargo build --features ARGS...`
`cargo run --features ARGS...`

The `--features` option is required to build or run the ETL project.

Currently, the following blockchains are supported:
- `SOLANA`

A message queue is required to be specified:
- `RABBITMQ` - a classic RabbitMQ queue
- `RABBITMQ_STREAM` - a RabbitMQ with Stream Queue plugin
- `GOOGLE_PUBSUB` - Google Cloud Pub/Sub
- `JSON` - JSON files (one file per record)
- `JSONL` - JSONL files (seven files per block - one for each table)

## Examples

1. Build the local project and its dependencies for the Google Pub/Sub publisher:
```
cargo build --release --features GOOGLE_PUBSUB
```

2. Run the local project and its dependencies for the JSON publisher:
```
cargo run --features JSON
```
