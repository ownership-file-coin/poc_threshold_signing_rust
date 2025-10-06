#!/bin/bash
set -e

echo "=== End-to-End Threshold Signing Test ==="

# Build and run Rust components
echo ""
echo "Step 1: Building Rust components..."
cd rust_threshold_signing
./scripts/build.sh

echo ""
echo "Step 2: Running threshold signing and proof generation..."
./scripts/run.sh

# Build and test Solidity
echo ""
echo "Step 3: Building Solidity contracts..."
cd ../solidity_threshold_signing
./scripts/build.sh

echo ""
echo "Step 4: Testing Solidity contracts..."
./scripts/test.sh

echo ""
echo "=== End-to-End Test Complete ==="
echo "Proof and verification key are available in solidity_threshold_signing/"
echo "To deploy, set environment variables and run: cd solidity_threshold_signing && ./scripts/deploy.sh"
