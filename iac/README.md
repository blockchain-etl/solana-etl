# IAC for ETL-Core Infrastructure

## Overview
This repository contains the necessary infrastructure as code to set up a Solana RPC node and RabbitMQ for an ETL (Extract, Transform, Load) project. It provisions and configures a secure environment on Google Cloud Platform (GCP) for running a Solana node and RabbitMQ for efficient data handling and processing.

## Structure
- **`main/`**: Houses the main Terraform configuration for provisioning the GCP environment, including provider configuration and all resources.
- **`scripts/`**: Contains shell scripts for automated setup and configuration of Solana RPC node, RabbitMQ, ETL indexer, and inserter.

## Resources and Scripts

### Terraform-Provisioned Resources
The Terraform code in the `main/` directory provisions the following resources:

1. Virtual Private Cloud (VPC) network and subnetworks
2. Firewall rules for secure access
3. Compute Engine instances:
   - Solana RPC node VM
   - RabbitMQ VM
   - Indexer VM
   - Inserter VM
4. Persistent disks for data storage
5. Cloud NAT for outbound internet access
6. IAM service accounts and roles for secure operations
7) Bigquery Dataset and tables with predefined schemas 

All variables for these resources are defined in the `main/provider.tf` file.

### Scripts
The `scripts/` directory contains shell scripts that automate the setup, configuration, and management of various components:

- `solana-rpc/`: Scripts for Solana RPC node setup
  - `configure-solana-disks.sh`: Configures RAID for local SSDs to boost Solana VM performance
  - `solana-install.sh`: Downloads and installs the Solana CLI and configures the validator
- `rabbit-mq/`: Scripts for RabbitMQ setup and configuration
  - `run-rabbit.sh`: Sets up and runs RabbitMQ
- `indexer/`: Scripts for ETL Indexer setup
  - `indexer-service.sh`: Sets up and runs the ETL Indexer
- `inserter/`: Scripts for ETL Inserter setup
  - `inserter-service.sh`: Sets up and runs the ETL Inserter

These scripts ensure that all components are properly installed, configured, and set up for reliable operation and easy management.

## Deployment Environment
This project can be deployed from:
- A personal laptop
- A VM
- Google Cloud Shell

As long as the prerequisites are met and the Google Cloud SDK is properly configured, you can run the Terraform commands from any of these environments.

## Prerequisites
- Terraform >= 1.5.7
- Google Cloud SDK (gcloud) installed and configured
- A Google Cloud Platform account

## Setup Guide

### 1. Setting up Google Cloud SDK
1. Install the Google Cloud SDK by following the [official documentation](https://cloud.google.com/sdk/docs/install).
2. Authenticate with Google Cloud:
   ```bash
   gcloud auth login
   ```

### 2. Creating a New Google Cloud Project
1. Create a new project:
   ```bash
   gcloud projects create [PROJECT_ID] --name="[PROJECT_NAME]"
   ```
   Replace `[PROJECT_ID]` with a unique ID for your project, and `[PROJECT_NAME]` with a descriptive name.

2. Set the newly created project as the active project:
   ```bash
   gcloud config set project [PROJECT_ID]
   ```

3. List your existing billing accounts:
   ```bash
   gcloud billing accounts list
   ```
   This command will display a list of billing accounts you have access to, along with their IDs.

4. If you don't have a billing account or want to create a new one, follow the instructions in the [official Google Cloud documentation to create a billing account](https://cloud.google.com/billing/docs/how-to/create-billing-account). 

   Note: Creating a new billing account typically requires going through the Google Cloud Console, as it involves setting up payment methods and cannot be done entirely through the CLI.

5. Once you have a billing account ID, link it to your project:
   ```bash
   gcloud billing projects link [PROJECT_ID] --billing-account=[BILLING_ACCOUNT_ID]
   ```
   Replace `[BILLING_ACCOUNT_ID]` with the ID of the billing account you want to use.

### 3. Enabling Required APIs
Enable the necessary Google Cloud APIs:
```bash
gcloud services enable storage.googleapis.com iam.googleapis.com compute.googleapis.com iap.googleapis.com
```

### 4. Creating a GCS Bucket for Terraform State

To create a new GCS bucket, you can use the following command:

```bash
gcloud storage buckets create gs://BUCKET_NAME --project=PROJECT_ID --location=BUCKET_LOCATION
```

Replace the following:
- `BUCKET_NAME`: A globally unique name for your bucket
- `PROJECT_ID`: Your Google Cloud project ID
- `BUCKET_LOCATION`: The location for your bucket (e.g., `us-central1`)

You can also specify additional flags:
- `--uniform-bucket-level-access`: Enables uniform bucket-level access
- `--public-access-prevention`: Sets public access prevention
- `--default-storage-class`: Sets the default storage class (e.g., `STANDARD`, `NEARLINE`, `COLDLINE`, `ARCHIVE`)

For example:

```bash
gcloud storage buckets create gs://my-terraform-state --project=my-project-id --location=us-central1 --uniform-bucket-level-access --public-access-prevention=enforced --default-storage-class=STANDARD
```
For more detailed information on creating buckets, including available regions and storage classes, refer to the [official Google Cloud documentation on creating storage buckets](https://cloud.google.com/storage/docs/creating-buckets#command-line).


### It's esential for users to check resource quota/limits since Solana node is resource heavey Reference : https://cloud.google.com/docs/quotas/view-manage


### 5. Configuring Terraform

Now that you have set up your Google Cloud project and created a bucket for the Terraform state, you need to configure the Terraform files.

1. Navigate to the `main` directory in the Terraform codebase.

2. Open the `provider.tf` file and update the following:

   ```hcl
   terraform {
     backend "gcs" {
       bucket = "[BUCKET_NAME]"  # Replace with your bucket name
       prefix = "terraform/state"
     }
   }

   variable "project_id" { ###Name of a freshly created project
     default = <your project id>
   }
   ```

   Replace `[BUCKET_NAME]` with the name of the GCS bucket you created, and `[PROJECT_ID]` with your Google Cloud project ID.

3. Review and adjust other variables in the `provider.tf` file as needed.

### 6. Specify Inserter and Extractor app versions 

Specify the version for the Indexer app with the `VERSION` variable defined in the [script](./scripts/indexer/indexer-service.sh#L4).
Specify the version for the Inserter app with the `VERSION` variable defined in the [script](./scripts/inserter/inserter-service.sh#L4).

Latest application releases can be found here: https://github.com/blockchain-etl/solana-etl/releases


### 7. Initializing and Applying Terraform

1. Initialize Terraform:
   ```bash
   terraform init
   ```

2. Review the planned changes:
   ```bash
   terraform plan
   ```

3. Apply the Terraform configuration:
   ```bash
   terraform apply
   ```

## Solana Node Configuration

**Note: All scripts in this section should be run with root privileges (sudo).**

### Accessing the VMs

To SSH into any of the VMs (Solana RPC, Indexer, Inserter, or RabbitMQ):

```bash
gcloud compute ssh [VM_NAME] --zone=[YOUR_ZONE]
```

Replace `[VM_NAME]` with one of: `solana-rpc-vm`, `indexer-vm`, `inserter-vm`, or `rabbitmq-vm`.

### Understanding the Environment

- The Solana node runs under the `sol` user in the `/home/sol` directory.
- To switch to the `sol` user: `sudo su - sol`
- `sudo bash` runs a command with root privileges.

### Downloading Configuration Scripts

On the Solana RPC VM, download the configuration scripts:

```bash
curl -O https://raw.githubusercontent.com/blockchain-etl/solana-etl/main/iac/scripts/solana-rpc/configure-solana-disks.sh
curl -O https://raw.githubusercontent.com/blockchain-etl/solana-etl/main/iac/scripts/solana-rpc/solana-install.sh
```

### Configuring Disks and Installing Solana

1. Make the scripts executable:
   ```bash
   sudo chmod +x configure-solana-disks.sh solana-install.sh
   ```

2. Execute the disk configuration script:
   ```bash
   sudo bash configure-solana-disks.sh
   ```

3. Execute the Solana installation script:
   ```bash
   sudo bash solana-install.sh
   ```

These scripts handle disk configuration, Solana RPC node setup, and Nginx configuration. They will expose the RPC port on `8899` internally, which will be proxied through Nginx on port 80.

### Solana Node Details

- Primary executable: `/home/sol/.local/share/solana/install/active_release/bin/solana-validator`
- Configuration files: `/home/sol/solana-rpc/config`
- Ledger location: `/home/sol/solana-rpc/ledger`

The `solana-install.sh` script initiates a download of a recent snapshot, allowing the node to sync faster than starting from the genesis block.

### RAID Configuration for Local SSDs
To achieve optimal performance for the Solana RPC node, we use local SSDs configured in a RAID array using MDM (Multiple Device Management). This configuration provides higher I/O performance, which is crucial for Solana node operations.

The `configure-solana-disks.sh` script automatically sets up the RAID array using the following steps:

1. Identifies all available local SSDs.
2. Creates a RAID-0 array using mdadm.
3. Creates a file system on the RAID array.
4. Mounts the RAID array to the appropriate directory.

For more information on local SSDs in Google Cloud, refer to the [official documentation on local SSDs](https://cloud.google.com/compute/docs/disks/local-ssd).

### Solana Node Synchronization
**Important Note:** After installation, the Solana node needs time to catch up to the network. The ETL pipeline will not be operational until this synchronization is complete.

To check the status of your Solana node:

1. SSH into the Solana RPC VM:
   ```bash
   gcloud compute ssh solana-rpc-vm --zone=[YOUR_ZONE]
   ```
2. Switch to the `sol` user:
   ```bash
   sudo su - sol
   ```
3. Run the following command to monitor the catch-up process:
   ```bash
   solana catchup --follow --our-localhost 8899
   ```

This command will show you the current slot and how far behind the node is from the current network state. The node is considered caught up when it's within a few hundred slots of the current network slot.

### Monitoring Services

To tail log files:

- Solana RPC: `sudo journalctl -fu solana-validator`
- Indexer: `sudo journalctl -fu indexer`
- Inserter: `sudo journalctl -fu inserter`
- RabbitMQ: `sudo journalctl -fu rabbitmq-server`

### Managing Services

To start/stop/restart services:

```bash
sudo systemctl start/stop/restart [SERVICE_NAME]
```

Replace `[SERVICE_NAME]` with one of: `solana-validator`, `indexer`, `inserter`, or `rabbitmq-server`.

For more details on systemd service management, refer to the [systemd documentation](https://www.freedesktop.org/software/systemd/man/systemctl.html).

### Firewall Configuration

By default, SSH ports are open to all IPs. To restrict access to a specific IP:

1. Edit the `main/firewall.tf` file.
2. Locate the `google_compute_firewall` resource for SSH.
3. Update the `source_ranges` to include only your IP address:

   ```hcl
   source_ranges = ["YOUR_IP_ADDRESS/32"]
   ```

4. Apply the changes:
   ```bash
   terraform apply
   ```

### Project Teardown

To remove all created resources:

1. Navigate to the `main/` directory.
2. Run:
   ```bash
   terraform destroy -auto-approve
   ```

This command will remove all VMs, firewall settings, and other resources created by Terraform.

## Additional Resources
- [Solana Documentation](https://docs.solana.com/)
- [RabbitMQ Documentation](https://www.rabbitmq.com/documentation.html)
- [Google Cloud Documentation](https://cloud.google.com/docs)
