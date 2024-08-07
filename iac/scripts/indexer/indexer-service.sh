#!/bin/bash

# Define variables
VERSION="v1.0.1"
SERVICE_NAME="blockchain_etl_indexer"
RELEASE_URL="https://github.com/BCWResearch/solana-etl/releases/download/extractor.$VERSION/$SERVICE_NAME"
INSTALL_DIR="/var/etl_files"
SERVICE_FILE_PATH="/etc/systemd/system/${SERVICE_NAME}.service"
USER_NAME="etl_user"
GROUP_NAME="etl_user"
ENV_FILE_PATH="${INSTALL_DIR}/.env"
SLOTVALUE="282024813"  # Define the slot value variable

# Create the user and group if they do not exist
if ! id "$USER_NAME" &>/dev/null; then
    echo "Creating user $USER_NAME..."
    sudo useradd -r -s /bin/false "$USER_NAME"
fi

if ! getent group "$GROUP_NAME" &>/dev/null; then
    echo "Creating group $GROUP_NAME..."
    sudo groupadd "$GROUP_NAME"
fi

# Add the user to the group
echo "Adding user $USER_NAME to group $GROUP_NAME..."
sudo usermod -a -G "$GROUP_NAME" "$USER_NAME"

# Create the installation directory if it does not exist
if [ ! -d "$INSTALL_DIR" ]; then
    echo "Creating installation directory $INSTALL_DIR..."
    sudo mkdir -p "$INSTALL_DIR"
    sudo chown "$USER_NAME:$GROUP_NAME" "$INSTALL_DIR"
fi

# Download the binary file
echo "Downloading binary version $VERSION..."
sudo curl -L -o "${INSTALL_DIR}/${SERVICE_NAME}" "$RELEASE_URL"

# Make the binary executable
echo "Setting executable permissions for the binary..."
sudo chmod +x "${INSTALL_DIR}/${SERVICE_NAME}"

# Define the .env file content
ENV_FILE_CONTENT="RABBITMQ_USER=jb
RABBITMQ_PASSWORD=jb
RABBITMQ_PORT=5672
RABBITMQ_ADDRESS=10.0.0.3
NUM_EXTRACTOR_THREADS=20
ENABLE_METRICS=true
METRICS_PORT=4000
METRICS_ADDRESS=127.0.0.1
RPC_METHOD_TIMEOUT=30
ENDPOINT=http://10.0.0.12/rpc
QUEUE_NAME=\"solana-etl\"
"

# Create the .env file
echo "Creating .env file at ${ENV_FILE_PATH}..."
echo "$ENV_FILE_CONTENT" | sudo tee $ENV_FILE_PATH > /dev/null

# Define the service file content
SERVICE_CONTENT="[Unit]
Description=Blockchain-ETL indexer
After=network.target

[Service]
ExecStart=${INSTALL_DIR}/${SERVICE_NAME} index-range stream $SLOTVALUE
WorkingDirectory=${INSTALL_DIR}
Restart=no
User=${USER_NAME}
Group=${GROUP_NAME}
EnvironmentFile=${ENV_FILE_PATH}
Environment=\"RUST_LOG=warn\"
ExecStop=/bin/kill -2 \$MAINPID
TimeoutStopSec=1800s

[Install]
WantedBy=multi-user.target
"

# Create the service file
echo "Creating service file at ${SERVICE_FILE_PATH}..."
echo "$SERVICE_CONTENT" | sudo tee $SERVICE_FILE_PATH > /dev/null

# Set correct permissions for the service file
sudo chmod 644 $SERVICE_FILE_PATH

# Reload systemd configuration
echo "Reloading systemd configuration..."
sudo systemctl daemon-reload

# Enable the service to start on boot
echo "Enabling the service..."
sudo systemctl enable "${SERVICE_NAME}.service"

# Optionally, start the service immediately
echo "Starting the service..."
sudo systemctl start "${SERVICE_NAME}.service"

echo "User, group, binary, .env file, and service setup complete."
