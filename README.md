# Solana ETL (was solana-config)

NOTE: You may be looking for the previous iteration of `solana-etl`, written in Python. That code has been moved to [here](https://github.com/blockchain-etl/solana-etl-airflow).

This repository contains all of the code for running a Solana ETL pipeline. The primary purpose of this is to serve data for Google BigQuery, but outputs for Google Pub/Sub, RabbitMQ, RabbitMQ Stream, JSON files, and JSONL files are supported.

If you would like to modify this code for your own purposes, see the [documentation](/docs/).

## Quickstart
To deploy your own instance of this pipeline, you can find full end-to-end deployment scripts and documentation [here](/iac/).

## Architecture Overview
The overall infrastructure is depicted below.

![architecture](/docs/img/architecture.png)

For more information check the [documentation](/docs/).

## Usage
For instructions on system setup, compilation, and running, see the documentation on [Getting Started](/docs/getting-started.md).
