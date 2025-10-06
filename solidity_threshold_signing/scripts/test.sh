#!/bin/bash
set -e

echo "Testing Solidity contracts..."

forge test -vvv

echo "Solidity tests passed!"
