#!/bin/bash

# If no argument is provided, run cargo build and call this script again
if [ -z "$1" ]; then
    echo "Building project in release mode..."
    cargo build --release || { echo "Build failed"; exit 1; }

    # Find libsancov.a
    LIB_PATH=$(find target/release -name "libastra_sancov.a" | head -n 1)

    if [ -z "$LIB_PATH" ]; then
        echo "libsancov.a not found"
        exit 1
    fi

    echo "Re-running installer with library path: $LIB_PATH"
    exec "$0" "$LIB_PATH"
fi

# installer logic
LIBRARY_PATH=$1

# Detect OS
if [[ "$OSTYPE" == "darwin"* ]]; then
    INSTALL_PATH="/usr/local/lib"
    echo "macOS detected, installing to $INSTALL_PATH"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    INSTALL_PATH="/usr/lib"
    echo "Linux detected, installing to $INSTALL_PATH"
else
    echo "Unsupported OS"
    exit 1
fi

# Copy the library
sudo cp "$LIBRARY_PATH" "$INSTALL_PATH" && echo "Library installed successfully!" || {
    echo "Failed to install library"
    exit 1
}

# Install the astra binary so the user can call it from any shell
cargo install --path crates/astra|| {
    echo "Failed to install astra binary"
    exit 1
}

echo "Astra installed successfully!"
