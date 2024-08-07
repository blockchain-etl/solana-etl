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
        su - sol -c 'sh -c "$(curl -sSfL https://release.solana.com/v1.18.18/install)"' || handle_error "Solana installation failed"
        echo 'export PATH=$PATH:/home/sol/.local/share/solana/install/active_release/bin' >> /home/sol/.bashrc || handle_error "Failed to set PATH for Solana"
    else
        log_message "Solana is already installed for user sol"
    fi
}

# Create and configure tmpfs and swap
configure_tmpfs_and_swap() {
    log_message "Configuring tmpfs and swap"
    
    # Create the directory for Solana accounts
    mkdir -p /mnt/solana-accounts || handle_error "Failed to create /mnt/solana-accounts directory"
    
    # Add tmpfs entry to /etc/fstab
    if ! grep -q '/mnt/solana-accounts' /etc/fstab; then
        echo 'tmpfs /mnt/solana-accounts tmpfs rw,size=400G,user=sol 0 0' >> /etc/fstab || handle_error "Failed to add tmpfs entry to /etc/fstab"
    else
        log_message "tmpfs entry already exists in /etc/fstab"
    fi
    
    # Create swap file
    if ! grep -q '/swapfile' /etc/fstab; then
        log_message "Creating swap file"
        dd if=/dev/zero of=/swapfile bs=1MiB count=250KiB || handle_error "Failed to create swap file"
        chmod 0600 /swapfile || handle_error "Failed to set permissions for swap file"
        mkswap /swapfile || handle_error "Failed to format swap file"
        echo '/swapfile swap swap defaults 0 0' >> /etc/fstab || handle_error "Failed to add swap file entry to /etc/fstab"
    else
        log_message "Swap file entry already exists in /etc/fstab"
    fi

    # Enable swap
    swapon -a || handle_error "Failed to enable swap"
    
    # Mount tmpfs
    mount /mnt/solana-accounts || handle_error "Failed to mount tmpfs"
    
    # Confirm swap is active and tmpfs is mounted
    free -g || handle_error "Failed to confirm swap"
    mount | grep tmpfs || handle_error "Failed to confirm tmpfs"
}

# Create user sol log directory
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

# Configure log rotation
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
 --entrypoint entrypoint.mainnet-beta.solana.com:8001 \
 --entrypoint entrypoint2.mainnet-beta.solana.com:8001 \
 --entrypoint entrypoint3.mainnet-beta.solana.com:8001 \
 --entrypoint entrypoint4.mainnet-beta.solana.com:8001 \
 --entrypoint entrypoint5.mainnet-beta.solana.com:8001 \
 --known-validator 7Np41oeYqPefeNQEHSv1UDhYrehxin3NStELsSKCT4K2 \
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
 --ledger /solana/ledger \
 --limit-ledger-size 100000000 \
 --snapshot-interval-slots 5000 \
 --maximum-local-snapshot-age 500 \
 --wal-recovery-mode skip_any_corrupted_record \
 --enable-rpc-transaction-history \
 --enable-cpi-and-log-storage \
 --full-rpc-api \
 --private-rpc \
 --snapshots /solana/rest \
 --accounts-index-path /solana/rest/indexes \
 --accounts-hash-cache-path /solana/rest/cache \
 --log /home/sol/solana-rpc.log
EOF
        chmod +x /home/sol/rpc-start.sh || handle_error "Failed to create validator start script"
        chown sol:sol /home/sol/rpc-start.sh
    else
        log_message "Validator start script already exists"
    fi
}

# Optimize sysctl knobs
optimize_sysctl() {
    log_message "Optimizing sysctl knobs"
    cat > /etc/sysctl.d/21-solana-validator.conf <<EOF
# Increase UDP buffer sizes
net.core.rmem_default = 134217728
net.core.rmem_max = 134217728
net.core.wmem_default = 134217728
net.core.wmem_max = 134217728

# Increase memory mapped files limit
vm.max_map_count = 1000000

# Increase number of allowed open file descriptors
fs.nr_open = 1000000
EOF

    sysctl -p /etc/sysctl.d/21-solana-validator.conf || handle_error "Failed to apply sysctl configurations"
}

# Increase session file limits
increase_file_limits() {
    log_message "Increasing systemd and session file limits"

    # Ensure systemd service file limits are set
    if ! grep -q "DefaultLimitNOFILE=1000000" /etc/systemd/system.conf; then
        log_message "Setting DefaultLimitNOFILE in /etc/systemd/system.conf"
        echo "DefaultLimitNOFILE=1000000" >> /etc/systemd/system.conf || handle_error "Failed to set DefaultLimitNOFILE in /etc/systemd/system.conf"
    fi

    # Reload systemd configuration
    systemctl daemon-reload || handle_error "Failed to reload systemd daemon"

    # Set session file limits
    cat > /etc/security/limits.d/90-solana-nofiles.conf <<EOF
# Increase process file descriptor count limit
* - nofile 1000000
EOF
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

# Install Nginx
install_nginx() {
    log_message "Installing Nginx"
    apt update || handle_error "Failed to update package list"
    apt install -y nginx || handle_error "Failed to install Nginx"
}

# Create Nginx configuration for Solana RPC and remove the default configuration
configure_nginx() {
    log_message "Configuring Nginx for Solana RPC"

    # Remove the default Nginx configuration
    rm /etc/nginx/sites-enabled/default || handle_error "Failed to remove default Nginx configuration"
    
    # Create new Nginx configuration
    cat > /etc/nginx/sites-available/rpc.conf <<EOF
server {
    listen 80;
    server_name 10.0.0.12;  

    location /rpc {
        proxy_pass http://localhost:8899;  # Pointing to the local Solana RPC endpoint
        proxy_http_version 1.1;
        proxy_set_header Upgrade \$http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host \$host;
        proxy_cache_bypass \$http_upgrade;

        # Additional headers to forward (optional)
        proxy_set_header X-Forwarded-For \$proxy_add_x_forwarded_for;
        proxy_set_header X-Real-IP \$remote_addr;
    }
}
EOF
    
    # Enable the new configuration
    ln -s /etc/nginx/sites-available/rpc.conf /etc/nginx/sites-enabled/rpc.conf || handle_error "Failed to enable new Nginx configuration"
    
    # Restart Nginx to apply the changes
    systemctl restart nginx || handle_error "Failed to restart Nginx"
}

# Main execution
log_message "Starting Solana installation and configuration script"
create_user_sol
install_solana
configure_tmpfs_and_swap
create_validator_identity
create_validator_script
optimize_sysctl
increase_file_limits
create_systemd_service
install_nginx
configure_nginx
start_solana_rpc_service
log_message "Solana installation and configuration completed successfully"

exit 0
