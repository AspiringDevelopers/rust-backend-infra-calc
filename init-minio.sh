#!/bin/bash

# Wait for MinIO to be ready
echo "Waiting for MinIO to be ready..."
sleep 5

# Install MinIO client if not already installed
if ! command -v mc &> /dev/null; then
    echo "Installing MinIO client..."
    curl https://dl.min.io/client/mc/release/linux-amd64/mc \
      --create-dirs \
      -o $HOME/minio-binaries/mc

    chmod +x $HOME/minio-binaries/mc
    export PATH=$PATH:$HOME/minio-binaries/
fi

# Configure MinIO client
echo "Configuring MinIO client..."
mc alias set local http://localhost:9000 minioadmin minioadmin

# Create bucket if it doesn't exist
echo "Creating bucket 'touchcalc-storage'..."
mc mb local/touchcalc-storage --ignore-existing

# Set public policy (optional, for development)
# mc anonymous set download local/touchcalc-storage

echo "MinIO setup complete!"
