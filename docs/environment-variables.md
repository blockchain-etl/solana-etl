# Environment Variables

## Synopsis

You can define enviornmental variables in a `.env` file. Examples are illustrated in `.env.example.`

## Variables
- `NUM_EXTRACTOR_THREADS`
The number of indexing workers to use (number of threads to allocate to this program).

- `ENDPOINT`
Specifies the address to use for json RPC requests.

- `FALLBACK_ENDPOINT`
Specifies the address to use for json RPC requests, when the primary endpoint is failing. This value can be the same `ENDPOINT`.

- `NUM_EXTRACTOR_THREADS`
Specifies the number of concurrent threads to run an extract job.

- `ENABLE_METRICS`
This variable determines whether to launch a metrics server to collect metrics for Prometheus.

- `METRICS_ADDRESS`
Required only if `ENABLE_METRICS` is true. Specifies the address of the metrics server.

- `METRICS_PORT`
Required only if `ENABLE_METRICS` is true. Specifies the port of the metrics server.

- `RABBITMQ_ADDRESS`
Specifies the address of RabbitMQ.

- `RABBITMQ_PORT`
Specifies the port of RabbitMQ.

- `QUEUE_NAME`
Used to specify the name of the RabbitMQ queue when using the deprecated `SINGLE_PUBLISHER`.

- `BIGTABLE_CRED`
Specifies the file path of the credential file required to access GCP Bigtable.

- `GCP_CREDENTIALS_JSON_PATH`
Required only if _STREAM_EXPORTER_ is set to `GOOGLE_PUBSUB`. Specifies the file path of the credential file required to access Google Pubsub.

- `GOOGLE_PUBSUB_TOPIC`
Required only if _STREAM_EXPORTER_ is set to `GOOGLE_PUBSUB`. Specifies the Google Pubsub topic to be used during exporting using the deprecated `SINGLE_PUBLISHER`. It is assumed that the PubSub Topic is already created.

- `OUTPUT_DIR`
Required only if _STREAM_EXPORTER_ is set to `JSON` or `JSONL`. Specifies the directory to output records to.

- `QUEUE_NAME_BLOCKS`
Specifies the name of the output subdirectory for block records when using `JSON` or `JSONL`, and specifies the Google Pub/Sub topic or RabbitMQ queue name when using those features.

- `QUEUE_NAME_BLOCK_REWARDS`
Specifies the name of the output subdirectory for block reward records when using `JSON` or `JSONL`, and specifies the Google Pub/Sub topic or RabbitMQ queue name when using those features.

- `QUEUE_NAME_ACCOUNTS`
Specifies the name of the output subdirectory for account records when using `JSON` or `JSONL`, and specifies the Google Pub/Sub topic or RabbitMQ queue name when using those features.

- `QUEUE_NAME_INSTRUCTIONS`
Specifies the name of the output subdirectory for instruction records when using `JSON` or `JSONL`, and specifies the Google Pub/Sub topic or RabbitMQ queue name when using those features.

- `QUEUE_NAME_TOKEN_TRANSFERS`
Specifies the name of the output subdirectory for token transfer records when using `JSON` or `JSONL`, and specifies the Google Pub/Sub topic or RabbitMQ queue name when using those features.

- `QUEUE_NAME_TOKENS`
Specifies the name of the output subdirectory for token records when using `JSON` or `JSONL`, and specifies the Google Pub/Sub topic or RabbitMQ queue name when using those features.

- `QUEUE_NAME_TRANSACTIONS`
Specifies the name of the output subdirectory for transaction records when using `JSON` or `JSONL`, and specifies the Google Pub/Sub topic or RabbitMQ queue name when using those features.
