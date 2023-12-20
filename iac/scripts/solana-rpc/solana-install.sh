#!/bin/bash

LOG_FILE="/var/log/solana-install.log"

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Function to log messages in green
log_message() {
    echo -e "${GREEN}$(date): $1${NC}" | tee -a "$LOG_FILE"
}

# Error handling function, log messages in red
handle_error() {
    echo -e "${RED}$(date): An error occurred: $1${NC}" | tee -a "$LOG_FILE"
    exit "$1"
}

# Check if the script is running as root
if [[ $EUID -ne 0 ]]; then
    log_message "This script must be run as root"
    exit 1
fi

# Create user 'sol' and add to sudo group
create_user_sol() {
    if id "sol" &>/dev/null; then
        log_message "User sol already exists"
    else
        log_message "Creating user sol"
        useradd -m sol -s /bin/bash || handle_error "Failed to create user sol"
        echo "sol ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers || handle_error "Failed to grant sol sudo privileges"
    fi
}

# Install Solana CLI as user 'sol'
install_solana() {
    if ! su - sol -c 'solana --version' &>/dev/null; then
        log_message "Installing Solana for user sol"
        su - sol -c 'sh -c "$(curl -sSfL https://release.solana.com/v1.17.9/install)"' || handle_error "Solana installation failed"
        echo 'export PATH=$PATH:/home/sol/.local/share/solana/install/active_release/bin' >> /home/sol/.bashrc || handle_error "Failed to set PATH for Solana"
    else
        log_message "Solana is already installed for user sol"
    fi
}

create_sol_log_directory() {
    local sol_log_dir="/home/sol/log"
    if [ ! -d "$sol_log_dir" ]; then
        log_message "Creating log directory for user sol"
        mkdir -p "$sol_log_dir" || handle_error "Failed to create log directory for user sol"
        chown sol:sol "$sol_log_dir" || handle_error "Failed to set ownership of log directory"
    else
        log_message "Log directory for user sol already exists"
    fi
}

configure_log_rotation() {
    local logrotate_conf="/etc/logrotate.d/solana-validator"
    
    if [ ! -f "$logrotate_conf" ]; then
        log_message "Configuring log rotation for Solana validator logs"
        cat > "$logrotate_conf" << EOF
/home/sol/log/*.log {
    weekly
    rotate 4
    compress
    delaycompress
    missingok
    notifempty
    create 640 sol sol
}
EOF
    else
        log_message "Log rotation for Solana validator logs is already configured"
    fi
}

# Create Validator Identity
create_validator_identity() {
    if ! sudo -u sol -i -- sh -c 'test -f ~/validator-keypair.json'; then
        log_message "Creating validator identity for user sol"
        sudo -u sol -i -- sh -c 'solana-keygen new -o ~/validator-keypair.json --no-bip39-passphrase' || handle_error "Failed to create validator identity"
    else
        log_message "Validator identity already exists for user sol"
    fi
}


# Create Validator Start Script
create_validator_script() {
    if [ ! -f /home/sol/rpc-start.sh ]; then
        log_message "Creating validator start script"
        cat > /home/sol/rpc-start.sh << 'EOF'
#!/bin/bash
export RUST_BACKTRACE=full
set -x
set -o errexit

# Validator start commands
solana-validator \
 --entrypoint entrypoint.mainnet-beta.solana.com:8001\
 --known-validator 7Np41oeYqPefeNQEHSv1UDhYrehxin3NStELsSKCT4K2\
 --known-validator GdnSyH3YtwcxFvQrVVJMm1JhTS4QVX7MFsX56uJLUfiZ \
 --known-validator DE1bawNcRJB9rVm3buyMVfr8mBEoyyu73NBovf2oXJsJ \
 --known-validator CakcnaRDHka2gXyfbEd2d3xsvkJkqsLw2akB3zsN1D2S \
 --expected-genesis-hash 5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d \
 --dynamic-port-range 8000-8020 \
 --rpc-port 8899 \
 --only-known-rpc \
 --identity /home/sol/validator-keypair.json \
 --no-voting \
 --no-wait-for-vote-to-start-leader \
 --accounts /mnt/solana-accounts \
 --ledger /solana/ledger  \
 --limit-ledger-size 100000000 \
 --snapshot-interval-slots 5000 \
 --maximum-local-snapshot-age 500 \
 --wal-recovery-mode skip_any_corrupted_record \
 --enable-rpc-transaction-history \
 --enable-cpi-and-log-storage \
 --full-rpc-api \
 --private-rpc \
 --snapshots /solana/rest\
 --accounts-index-path /solana/data/rest\
 --accounts-hash-cache-path /solana/rest/cache\
 --log ~/log/validator.log
EOF
        chmod +x /home/sol/rpc-start.sh || handle_error "Failed to create validator start script"
        chown sol:sol /home/sol/rpc-start.sh
    else
        log_message "Validator start script already exists"
    fi
}

# Create and Enable Systemd Service
create_systemd_service() {
    if [ ! -f /etc/systemd/system/solana-rpc.service ]; then
        log_message "Creating Solana RPC service"
        cat > /etc/systemd/system/solana-rpc.service << 'EOF'
[Unit]
Description=Solana RPC
After=network.target
Wants=systuner.service
StartLimitIntervalSec=0

[Service]
DefaultLimitNOFILE=1000000
Type=simple
Restart=on-failure
RestartSec=1
LimitNOFILE=1000000
LogRateLimitIntervalSec=0
User=sol
Environment=PATH=/bin:/usr/bin:/home/sol/.local/share/solana/install/active_release/bin
ExecStart=/home/sol/rpc-start.sh

[Install]
WantedBy=multi-user.target
EOF
        
        systemctl daemon-reload || handle_error "Failed to reload systemd daemon"
        systemctl enable solana-rpc.service || handle_error "Failed to enable Solana RPC service"
    else
        log_message "Solana RPC service already exists"
    fi
}

start_solana_rpc_service() {
    if systemctl is-active --quiet solana-rpc.service; then
        log_message "Solana RPC service is already running"
    else
        log_message "Starting Solana RPC service"
        systemctl start solana-rpc.service || handle_error "Failed to start Solana RPC service"
    fi
}

# Main execution
log_message "Starting Solana installation and configuration script"
create_user_sol
install_solana
create_validator_identity
create_validator_script
create_systemd_service
start_solana_rpc_service
log_message "Solana installation and configuration completed successfully"

exit 0
