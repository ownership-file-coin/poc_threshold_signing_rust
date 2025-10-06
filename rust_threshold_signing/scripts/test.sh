#!/bin/bash
set -e

echo "Testing Rust components..."

# Test library
echo "Testing threshold signing library..."
cd lib
cargo test
cd ..

# Test host
echo "Testing host program..."
cd host
cargo test
cd ..

echo "All Rust tests passed!"
