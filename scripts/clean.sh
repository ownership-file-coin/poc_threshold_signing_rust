#!/bin/bash
set -e

echo "Cleaning build artifacts..."

# Clean Rust artifacts
cd rust_threshold_signing
cargo clean
rm -f program/elf/*.bin

# Clean Solidity artifacts
cd ../solidity_threshold_signing
forge clean
rm -f proof.bin vk.bin

echo "Clean complete!"
