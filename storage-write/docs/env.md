A `.env` is expected in the root, containing the following environment variables:
1. `GOOGLE_APPLICATION_CREDENTIALS`: a file path to an IAM key
2. `BQ_PROJECT_ID`: the GCP project containing the dataset
3. `BQ_DATASET_ID`: the dataset name
4. `RABBITMQ_USER`: the user for the rabbitMQ server
5. `RABBITMQ_PASS`: the password for the rabbitMQ server
6. `RABBITMQ_HOST`: the host address for the rabbitMQ server
7. `RABBITMQ_PORT`: the port for the rabbitMQ server
8. `BATCH_SIZE`: the number of records to append in each stream
9. `INSERTER_COUNT`: the number of streams to use when committing records
