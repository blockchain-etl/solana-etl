 # ETL-Core Documentation
 Here we should explain what the ETL-Core is and how it works.

 ## ETL Infrastructure Architecture
 ### Architecture Framework
 The `etl-core` repository will serve as the primary engine for ETL actions, operating at the network level and service level, and can accept custom configurations. Developers will be able to set up custom configurations within `etl-core`.  Once the network and export service is selected, users can use `etl-core` to export the desired blockchain data.


 Currently, the Solana blockchain is supported in [etl-solana-config](https://github.com/BCWResearch/etl-solana-config).

 ### Macro Infrastructure
 An RPC node is expected to serve requests. Blocks are continually requested using the node, and if necessary, other data such as accounts may be requested as well. Upon response, the data is converted into a Protocol Buffers data format and sent to a streaming queue, such as Google Cloud Pub/Sub or RabbitMQ. You will need a transformer and loader that listens for the messages, transforms them to match the table schema, and inserts them into BigQuery.

 ## Response Deserialization
 To deserialize JSON responses from the blockchain node, we expect the blockchain configuration to specify the structure of the response in a Rust `struct` and annotate it with the `Deserialize` macro from the `serde` library. This macro generates deserialization code for the developer which eases development, but more importantly allows us to deserialize it with the `simd-json` library.

 The `simd-json` library uses CPU vector extensions for accelerated JSON deserialization. Currently, the library supports x86 and ARM vector extensions, but falls back to standard deserialization if used on a system that doesn't support SIMD.
 * Since x86's AVX2 is 256-bit, while ARM's NEON is 128-bit, *you can expect best performance on x86*.
 * This library is only used when compiled in the `release` profile, because its error messages are less descriptive. For development, it is recommended that you compile in debug mode (the default profile), which will use the `serde` deserializer, thus providing more descriptive errors.

 ## Environmental Variables
 ### Synopsis

 You can define enviornmental variables in a `.env` file. Examples are illustrated in `.env.example.`

 ### Variables
- `ENDPOINT`
 **Required**. Specifies the address to use for json RPC requests.

 - `FALLBACK_ENDPOINT`
 **Required**. Specifies the address to use for json RPC requests, when the primary endpoint is failing. This value can be the same `ENDPOINT`.

 - `NUM_EXTRACTOR_THREADS`
 **Required**. Specifies the number of concurrent threads to run an extract job.

 - `ENABLE_METRICS`
 **Required**. This variable determines whether to launch a metrics server to collect metrics for Prometheus.

 - `METRICS_ADDRESS`
 Optional. Required only if `ENABLE_METRICS` is true. Specifies the address of the metrics server.

 - `METRICS_PORT`
 Optional. Required only if `ENABLE_METRICS` is true. Specifies the port of the metrics server.

 - `RABBITMQ_ADDRESS`
 Optional. Required only if _STREAM_EXPORTER_  is set to `RABBITMQ_STREAM`. Specifies the address of RabbitMQ.

 - `RABBITMQ_PORT`
 Optional. Required only if _STREAM_EXPORTER_  is set to `RABBITMQ_STREAM`. Specifies the port of RabbitMQ.

 - `BIGTABLE_CRED`
 Optional. Specifies the file path of the credential file required to access GCP Bigtable.

 - `GCP_CREDENTIALS_JSON_PATH`
 Optional. Required only if _STREAM_EXPORTER_  is set to `GOOGLE_PUBSUB`. Specifies the file path of the credential file required to access Google Pubsub.

 - `GOOGLE_PUBSUB_TOPIC`
 Optional. Required only if _STREAM_EXPORTER_ is set to `GOOGLE_PUBSUB`. Specifies the Google Pubsub topic to be used during exporting. It is assumed that the PubSub Topic is already created.

 ## Data Extraction

 All RPC requests are retried with backoff upon failure, with failures logged at the `warning` level.

 Blocks are requested from the node by the `call_getBlock()` function.

 The `call_getBlockHeight()` function requests the current block height.

 The `call_getMultipleAccounts()` function requests account data for a list of pubkeys. These pubkeys come from the created accounts and token mints in the block data.

 The blockchain configuration is expected to define the HTTP requests that these functions make in a `<BLOCKCHAIN_CONFIG>/types/request_types.rs` file. These requests should be specified using `struct`s called `BlockHeightRequest` and `BlockRequest`, and should implement `serde::Serialize`. It is recommended that you annotate the struct with `#[derive(serde::Serialize)]`  to simplify this process and generate the code.

 ### Concurrency

 The master thread continually sends slot values to a concurrent queue for worker threads to index.

 Long-lived threads are created at the start of runtime by the master thread, and continually pull tasks (slot values) from the concurrent queue. Each thread makes requests to the node for the block data at that slot, then deserializes the response, and transmits the data to a stream queue.
 * For communication with the stream queue (which supports concurrent producers), each thread serializes its data using the protocol buffers interface, and transmits the information.

 ## Features

 ### Synopsis

 You can either define `--features` in the `Cargo.toml` file inside the `etl-core` repository or specify them as part of a command.

 `cargo build --features ARGS...`
 `cargo run --features ARGS...`

 The `--features` option is required to build or run the ETL project.

 ### Arguments

 Currently, the following blockchains are supported:
 - `SOLANA`

 A message queue is required to be specified:
 - `RABBITMQ` - a classic RabbitMQ queue
 - `RABBITMQ_STREAM` - a RabbitMQ with Stream Queue plugin
 - `GOOGLE_PUBSUB` - Google Cloud Pub/Sub

 ### Examples

 1. Build the local project and its dependencies for the _SOLANA_ blockchain
 ```
 cargo build --release --features SOLANA,RABBITMQ_STREAM
 ```

 2. Run the local project and its dependencies for the _SOLANA_blockchain and _RABBITMQ_STREAM_ exporter
 ```
 cargo run --features SOLANA,RABBITMQ_STREAM
 ```

 ## Limitations
 - Only limited number of `Token-2022 Program` information is extracted.
 - `SOLANA_BIGTABLE` feature can only request 1000 confirmed slots each time.

 ## Project Progress

 ### Deployment Method
| Metrics                | Development Status |
| ---------------------- | ------------------ |
| Dockerfile             | In Development     |
| Helm Chart             | In Development     |

 ### Export Method
| Metrics                | Development Status |
| ---------------------- | ------------------ |
| CSV                    | Completed          |
| Google PubSub          | Completed          |
| RabbitMQ               | Completed          |

 ### Extraction Source
| Metrics                | Development Status |
| ---------------------- | ------------------ |
| Bigtable               | Completed          |
| JSON RPC               | Completed          |

 ### Metrics Collection
| Metrics                | Development Status |
| ---------------------- | ------------------ |
| Block Request Count    | In Development     |
| Failed Block Count     | Not Started        |

 ### Tables
| Table            | Development Status |
| ---------------- | ------------------ |
| Accounts         | Completed          |
| Blocks           | Completed          |
| Instructions     | Completed          |
| Tokens           | Completed          |
| Token Transfers  | Completed          |
| Transactions     | Completed          |


 ## Protocol Buffers

 We use protocol buffers to serialize our data for transmission to a pub/sub system like RabbitMQ or Google Cloud Pub/Sub.

 Some blockchains provide their own protobuf interfaces, so when possible, we will attempt to use those.

 ### Codegen
 To generate Rust code from our protobuf interface, we use the `PROST` library. This is a popular library for Rust, and is used by the Solana blockchain with their official "storage" protobuf. We perform this codegen at compile time, using a custom Rust build script: `build_proto.rs`. This script uses the `include!` macro to import the protobuf build script from the blockchain-specific configuration. It is expected that each blockchain config will define its own protobuf build script.