#!/bin/bash

# Define a log file
LOG_FILE="/var/log/mdadm-setup.log"

# Function to log messages
log_message() {
    echo "$(date): $1" | tee -a "$LOG_FILE"
}

# Error handling function
handle_error() {
    log_message "An error occurred. Exiting with error code $1"
    exit "$1"
}

# Function to update and install mdadm
install_mdadm() {
    log_message "Updating packages and installing mdadm"
    apt update && apt install mdadm --no-install-recommends || handle_error $?
}

# Function to create RAID arrays
create_raid() {
    local raid_device="$1"
    shift
    local devices=("$@")
    log_message "Creating RAID device $raid_device with devices ${devices[*]}"
    mdadm --create "$raid_device" --level=0 --raid-devices=${#devices[@]} "${devices[@]}" || handle_error $?
}

# Function to format RAID devices
format_raid_device() {
    local raid_device="$1"
    log_message "Formatting RAID device $raid_device"
    mkfs.ext4 -F "$raid_device" || handle_error $?
}

# Function to mount RAID devices
mount_raid_device() {
    local raid_device="$1"
    local mount_point="$2"
    log_message "Mounting $raid_device at $mount_point"
    mkdir -p "$mount_point"
    mount "$raid_device" "$mount_point" || handle_error $?
}

# Function to update fstab
update_fstab() {
    local raid_device="$1"
    local mount_point="$2"
    local uuid
    uuid=$(blkid -s UUID -o value "$raid_device") || handle_error $?
    log_message "Adding $raid_device to /etc/fstab"
    echo "UUID=$uuid $mount_point ext4 discard,defaults,nofail 0 2" >> /etc/fstab || handle_error $?
}

# Function to set permissions and ownership
set_permissions() {
    local directory="$1"
    local user="$2"
    local group="$3"
    log_message "Setting permissions and ownership for $directory"
    chmod a+w "$directory" || handle_error $?
    chown -R "$user:$group" "$directory" || handle_error $?
}

# Function to create additional directories
create_additional_directories() {
    log_message "Creating additional directories"
    mkdir -p /solana/ledger /solana/rest /solana/rest/cache || handle_error "Failed to create additional directories"
    chown -R sol:sol /solana || handle_error "Failed to change ownership of /solana"
}

# Main execution
install_mdadm

# Create RAID arrays
create_raid /dev/md0 /dev/disk/by-id/google-local-nvme-ssd-{0..7}
create_raid /dev/md1 /dev/disk/by-id/google-local-nvme-ssd-{8..15}

# Format RAID devices
format_raid_device /dev/md0
format_raid_device /dev/md1

# Mount RAID devices
mount_raid_device /dev/md0 /solana/ledger
mount_raid_device /dev/md1 /solana/rest

# Update fstab
update_fstab /dev/md0 /solana/ledger
update_fstab /dev/md1 /solana/rest

# Set permissions and ownership
set_permissions /solana/ledger sol sol
set_permissions /solana/rest sol sol

# Create additional directories
create_additional_directories

log_message "Script completed successfully"

exit 0
