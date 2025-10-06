#!/bin/bash
set -e

# Check for required environment variables
if [ -z "$PRIVATE_KEY" ]; then
    echo "Error: PRIVATE_KEY environment variable not set"
    exit 1
fi

if [ -z "$SP1_VERIFIER" ]; then
    echo "Error: SP1_VERIFIER environment variable not set"
    exit 1
fi

if [ -z "$PROGRAM_VKEY" ]; then
    echo "Error: PROGRAM_VKEY environment variable not set"
    exit 1
fi

if [ -z "$RPC_URL" ]; then
    echo "Error: RPC_URL environment variable not set"
    exit 1
fi

echo "Deploying ThresholdSignatureVerifier..."

forge script script/Deploy.s.sol:DeployScript \
    --rpc-url $RPC_URL \
    --broadcast \
    --verify

echo "Deployment complete!"
