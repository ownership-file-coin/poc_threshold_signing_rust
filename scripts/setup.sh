#!/bin/bash
set -e

echo "Setting up development environment..."

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust is not installed. Install from https://rustup.rs/"
    exit 1
fi

# Check for SP1
if ! command -v cargo-prove &> /dev/null; then
    echo "Installing SP1 toolchain..."
    curl -L https://sp1.succinct.xyz | bash
    sp1up
fi

# Check for Foundry
if ! command -v forge &> /dev/null; then
    echo "Installing Foundry..."
    curl -L https://foundry.paradigm.xyz | bash
    foundryup
fi

# Make scripts executable
echo "Making scripts executable..."
chmod +x scripts/*.sh
chmod +x rust_threshold_signing/scripts/*.sh
chmod +x solidity_threshold_signing/scripts/*.sh

echo ""
echo "Setup complete!"
echo ""
echo "Next steps:"
echo "1. Run './scripts/e2e.sh' to test the complete system"
echo "2. Configure .env file for deployment"
echo "3. Run 'cd solidity_threshold_signing && ./scripts/deploy.sh' to deploy"
