#!/bin/bash
set -e

echo "Running SP1 Threshold Signing Demo..."

# Ensure everything is built
./scripts/build.sh

# Run the host program to generate proof
echo "Generating proof..."
cd host
cargo run --release

echo "Proof generation complete!"
echo "Proof and verification key saved to ../solidity_threshold_signing/"
