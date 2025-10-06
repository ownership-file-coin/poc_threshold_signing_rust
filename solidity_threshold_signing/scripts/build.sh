#!/bin/bash
set -e

echo "Building Solidity contracts..."

# Install dependencies
forge install

# Build contracts
forge build

echo "Solidity build complete!"
