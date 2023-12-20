#!/bin/bash

LOG_FILE="/var/log/rabbitmq-install.log"

# Function to log messages
log_message() {
    echo "$(date): $1" | tee -a "$LOG_FILE"
}

# Error handling function
handle_error() {
    log_message "An error occurred: $1"
    exit 1
}

# Update and install essential dependencies
update_and_install_dependencies() {
    log_message "Updating packages and installing essential dependencies"
    apt-get update -y || handle_error "Failed to update packages"
    apt-get install curl gnupg apt-transport-https -y || handle_error "Failed to install essential dependencies"
}

# Add repository signing keys
add_repository_signing_keys() {
    log_message "Adding repository signing keys"
    curl -1sLf "https://keys.openpgp.org/vks/v1/by-fingerprint/0A9AF2115F4687BD29803A206B73A36E6026DFCA" | gpg --dearmor | tee /usr/share/keyrings/com.rabbitmq.team.gpg > /dev/null || handle_error "Failed to add main RabbitMQ signing key"
    # Additional keys for Erlang and RabbitMQ repositories can be added here
}

# Add RabbitMQ and Erlang repositories
add_rabbitmq_erlang_repositories() {
    log_message "Adding RabbitMQ and Erlang repositories"
    cat > /etc/apt/sources.list.d/rabbitmq.list <<EOF || handle_error "Failed to add RabbitMQ repository"
# Repository definitions...
EOF
    apt-get update -y || handle_error "Failed to update packages after adding repositories"
}

# Install Erlang and RabbitMQ
install_erlang_and_rabbitmq() {
    log_message "Installing Erlang and RabbitMQ"
    apt-get install -y erlang-base \
    erlang-asn1 erlang-crypto erlang-eldap erlang-ftp erlang-inets \
    erlang-mnesia erlang-os-mon erlang-parsetools erlang-public-key \
    erlang-runtime-tools erlang-snmp erlang-ssl \
    erlang-syntax-tools erlang-tftp erlang-tools erlang-xmerl || handle_error "Failed to install Erlang"
    apt-get install rabbitmq-server -y --fix-missing || handle_error "Failed to install RabbitMQ server"
    systemctl enable rabbitmq-server || handle_error "Failed to enable RabbitMQ server"
}

enable_management_plugin() {
    log_message "Enabling RabbitMQ Management Plugin"
    rabbitmq-plugins enable rabbitmq_management || handle_error "Failed to enable RabbitMQ Management Plugin"
    # Wait for RabbitMQ server to start and management plugin to be ready
    sleep 10
}


# Configure RabbitMQ
configure_rabbitmq() {
    log_message "Configuring RabbitMQ"
    read -p "Enter RabbitMQ user: " RABBITMQ_USER
    read -sp "Enter RabbitMQ password: " RABBITMQ_PASSWORD
    echo
    read -p "Enter RabbitMQ vhost: " RABBITMQ_VHOST
    read -p "Enter RabbitMQ queue name: " RABBITMQ_QUEUE
    
    rabbitmqctl add_vhost $RABBITMQ_VHOST || handle_error "Failed to add vhost $RABBITMQ_VHOST"
    rabbitmqctl add_user $RABBITMQ_USER $RABBITMQ_PASSWORD || handle_error "Failed to add user $RABBITMQ_USER"
    rabbitmqctl set_permissions -p $RABBITMQ_VHOST $RABBITMQ_USER "." "." ".*" || handle_error "Failed to set permissions for user $RABBITMQ_USER"
    rabbitmqadmin declare queue --vhost=$RABBITMQ_VHOST name=$RABBITMQ_QUEUE durable=true || handle_error "Failed to declare queue $RABBITMQ_QUEUE"
}

# Main execution
if [[ $EUID -ne 0 ]]; then
    log_message "This script must be run as root"
    exit 1
fi

log_message "Starting RabbitMQ installation script"
update_and_install_dependencies
add_repository_signing_keys
add_rabbitmq_erlang_repositories
install_erlang_and_rabbitmq
enable_management_plugin
configure_rabbitmq
log_message "RabbitMQ installation and configuration completed successfully"

exit 0
