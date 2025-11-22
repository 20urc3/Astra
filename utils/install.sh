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

# refreshing ldconfig
sudo ldconfig || {
    echo "Failed to refresh LDCONFIG"
    exit1
}

# Install binaries
cargo install --path crates/astra || {
    echo "Failed to install astra binary"
    exit 1
}

cargo install --path crates/astra_cc || {
    echo "Failed to install astra_cc"
    exit1
}

cargo install --path crates/astra_cxx || {
    echo "Failed to install astra_cxx"
    exit1
}

echo "Astra and dependencies are installed successfully!"
