#!/bin/bash
set -e

echo "Building SP1 Threshold Signing Project..."

# Build the library
echo "Building threshold signing library..."
cd lib
cargo build --release
cd ..

# Build the guest program (RISC-V)
echo "Building SP1 guest program..."
cd program
cargo prove build
cd ..

# Build the host program
echo "Building SP1 host program..."
cd host
cargo build --release
cd ..

echo "Build complete!"
