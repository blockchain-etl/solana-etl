# IAC for ETL-Core infrastructure

## Overview
This repository contains the necessary infrastructure as code configure a secure environment on Google Cloud Platform (GCP) and deploy a Solana RPC node and RabbitMQ for the Solana-ETL pipeline.

## Structure
- `init/`: Contains Terraform scripts for enabling required APIs in the GCP environment.
- `main/`: Houses the main Terraform configuration for provisioning the GCP environment. This includes setting up firewalls, VPC, and all prerequisites to securely run a Solana node.
- `scripts/`: Includes shell scripts for configuring disks, downloading, and running the Solana RPC node, as well as setting up RabbitMQ configuration automatically.

## Usage
1. **API Initialization**:
   - Navigate to the `init/` directory.
   - Run the Terraform scripts to enable the necessary APIs in your GCP project adapt Region and Project variable to your needs

2. **Environment Provisioning**:
   - Move to the `main/` directory.
   - Execute the Terraform scripts to provision the GCP environment, including firewalls, VPC, and other required infrastructure components.adapt the `variables.tf` to your needs and run the Terraform scripts to set up the GCP environment.


3. **Solana Node Configuration**:
   - After provisioning the infrastructure, download and run the scripts located in the `scripts/solana-rpc` directory on the Solana node.
   - These scripts will handle disk configuration, Solana RPC node setup and will expose RPC port on "8899"

4. **RabbitMQ configuration**
   - After provisioning the RabbitMQ server, download the scripts/rabbitmq/run-rabbit.sh and run it,
   - This script automates the installation and configuration of RabbitMQ on a Linux system, including updating packages, installing dependencies, setting up repositories, installing Erlang and RabbitMQ, enabling the management plugin, and configuring RabbitMQ settings.

## Prerequisites
- A Google Cloud Platform account.
- Terraform installed on your local machine.
- Basic knowledge of GCP, Terraform, and shell scripting.
