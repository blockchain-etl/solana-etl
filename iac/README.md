# IAC for ETL-Core Infrastructure

## Overview
This repository contains the necessary infrastructure as code to set up a Solana RPC node and RabbitMQ for an ETL (Extract, Transform, Load) project. It provisions and configures a secure environment on Google Cloud Platform (GCP) for running a Solana node and RabbitMQ for efficient data handling and processing.

## Prerequisites
- Terraform >= 1.5.7
- A GCP bucket for storing Terraform states
- A Google Cloud Platform account
- Authenticate Terraform to Google Cloud Platform:
    ```bash
    gcloud auth application-default login
    ```

## Structure
- **`init/`**: Contains Terraform scripts for enabling required APIs in the GCP environment.
- **`main/`**: Houses the main Terraform configuration for provisioning the GCP environment, including firewalls, VPC, and all prerequisites to securely run a Solana node.
- **`scripts/`**: Includes shell scripts for configuring disks, downloading and running the Solana RPC node, and setting up RabbitMQ configuration automatically.

## Deployment Steps

### 1. API Initialization
1. Navigate to the `init/` directory.
2. Run the Terraform scripts to enable the necessary APIs in your GCP project. Adapt the region and project variables to your needs:
    ```bash
    terraform apply -auto-approve
    ```
3. Note: Make sure to replace the [bucket name](./init/main.tf#L10) with your own.

### 2. Environment Provisioning
1. Move to the `main/` directory.
2. Adapt the [variables.tf](./main/variables.tf#L14) file to your needs and run the Terraform scripts to set up the GCP environment, including firewalls, VPC, and other required infrastructure components:
    ```bash
    terraform apply -auto-approve
    ```
3. Note: Make sure to replace the [project_id](./main/variables.tf#L14) and [bucket name](./main/provider.tf#L10) with your own.

### 3. App Provisioning
1. The Terraform code will create:
   - 1 RabbitMQ VM
   - 1 Inserter VM
   - 1 Indexer VM
   - 1 Solana RPC VM
2. Specify the version for the Indexer app with the `VERSION` variable defined in the [script](./scripts/indexer/indexer-service.sh#L4).
3. Specify the version for the Inserter app with the `VERSION` variable defined in the [script](./scripts/inserter/inserter-service.sh#L4).
4. To connect to a pre-existing BigQuery dataset:
   - A GCP service account with `roles/bigquery.dataEditor` on the target BigQuery dataset is required.
   - The service account key needs to be copied to the Indexer VM.
   - The location of the service account key is defined by the `SERVICE_ENVIRONMENT` variable in the [script](./scripts/inserter/inserter-service.sh#L10).
5. Make sure the [slot value](./scripts/indexer/indexer-service.sh#12) is not older than the `Full Snapshot Slot` value from the Solana RPC node.
6. Note: If the target BigQuery dataset is in the same project, the service account key is not required.

### 4. BigQuery Provisioning
1. The BigQuery dataset is provisioned by the [bq.tf](./main/bq.tf) file with pre-created data tables.
2. This setup is only used to create a target BigQuery dataset within the same project. Otherwise, the code should be commented out.

### 5. Solana Node Configuration
1. After provisioning the infrastructure, download and run the scripts located in the `scripts/solana-rpc` directory on the Solana node.
2. These scripts handle disk configuration, Solana RPC node setup, and will expose the RPC port on `8899`.
3. Execute the `configure-solana-disks.sh` script first and ensure it completes successfully:
    ```bash
    chmod +x configure-solana-disks.sh
    ./configure-solana-disks.sh
    ```
4. Execute the `solana-install.sh` script and ensure it completes successfully:
    ```bash
    chmod +x solana-install.sh
    ./solana-install.sh
    ```
5. Note: Ensure the scripts are run with root privileges.
