#!/bin/bash

# Define variables
VERSION="v1.0.0"
SERVICE_FILE_PATH="/etc/systemd/system/solana_etl_inserter.service"
SERVICE_USER="etl_user"
SERVICE_GROUP="etl_user"
SERVICE_WORKING_DIRECTORY="/var/solana_etl_inserter"
SERVICE_EXEC_START="${SERVICE_WORKING_DIRECTORY}/main myFirstConsumer"
SERVICE_ENVIRONMENT="GOOGLE_APPLICATION_CREDENTIALS=${SERVICE_WORKING_DIRECTORY}/solana-bq.json"
BINARY_URL="https://github.com/BCWResearch/solana-etl/releases/download/inserter.${VERSION}/blockchain_etl_inserter"
ENV_FILE_PATH="${SERVICE_WORKING_DIRECTORY}/.env"

# Check if the script is being run as root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root."
  exit 1
fi

# Create the etl_user and group if they do not exist
if ! id -u $SERVICE_USER > /dev/null 2>&1; then
  groupadd $SERVICE_GROUP
  useradd -g $SERVICE_GROUP $SERVICE_USER
fi

# Create the working directory if it doesn't exist
mkdir -p $SERVICE_WORKING_DIRECTORY

# Download the binary
curl -L $BINARY_URL -o ${SERVICE_WORKING_DIRECTORY}/blockchain_etl_inserter

# Rename the binary to main
mv ${SERVICE_WORKING_DIRECTORY}/blockchain_etl_inserter ${SERVICE_WORKING_DIRECTORY}/main

# Make the binary executable
chmod +x ${SERVICE_WORKING_DIRECTORY}/main

# Create the .env file
cat <<EOF > $ENV_FILE_PATH
QUEUE_NAME="solana-etl"
BQ_PROJECT_ID="<your project ID>"
BQ_DATASET_ID="crypto_solana_mainnet_us"
RABBITMQ_USER="jb"
RABBITMQ_PASS="jb"
RABBITMQ_HOST="10.0.0.3"
RABBITMQ_PORT="5672"
EOF

# Change ownership of the working directory to the service user and group
chown -R $SERVICE_USER:$SERVICE_GROUP $SERVICE_WORKING_DIRECTORY

# Create the systemd service file
cat <<EOF > $SERVICE_FILE_PATH
[Unit]
Description=Solana ETL Inserter
After=network.target

[Service]
ExecStart=$SERVICE_EXEC_START
WorkingDirectory=$SERVICE_WORKING_DIRECTORY
Restart=always
RestartSec=5
User=$SERVICE_USER
Group=$SERVICE_GROUP
#Environment="$SERVICE_ENVIRONMENT"
EnvironmentFile=$ENV_FILE_PATH
ExecStop=/bin/kill -2 \$MAINPID

[Install]
WantedBy=multi-user.target
EOF

# Reload the systemd daemon to apply the new service
systemctl daemon-reload

# Enable the service to start on boot
systemctl enable solana_etl_inserter.service

# Start the service
systemctl start solana_etl_inserter.service

echo "Systemd service for Solana ETL Inserter created and started successfully."
