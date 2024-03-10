#!/bin/bash
set -x

BINARY_NAME=sqslistener
SERVICE_NAME=sqslistener.service

# Build
cargo build --release

# Install bin
chmod +x "target/release/$BINARY_NAME"
sudo cp "target/release/$BINARY_NAME" /usr/bin/

# Kill service
pkill "$BINARY_NAME"
systemctl disable "$SERVICE_NAME"
systemctl stop "$SERVICE_NAME"

# Install service
SERVICE_FILE_PATH="/etc/systemd/system/$SERVICE_NAME"
sudo cp "installation/systemd/$SERVICE_NAME" "$SERVICE_FILE_PATH"

# set username for service
sudo sed -i "s/USER_PLACEHOLDER/$USER/g" "$SERVICE_FILE_PATH"

sudo systemctl start "$SERVICE_NAME"
sudo systemctl enable "$SERVICE_NAME"


