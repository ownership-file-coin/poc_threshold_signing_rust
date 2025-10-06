# Threshold Signature Verification in SP1 zkVM - Implementation Plan

## Overview
This plan outlines the implementation of a proof-of-concept for threshold signature verification using ed25519-dalek in SP1 zkVM. The system will demonstrate serialization/deserialization of threshold signing operations without actual network communication, generating STARK proofs that can be verified on-chain.

## Project Structure

```
poc_threshold_signing_rust/
├── rust_threshold_signing/
│   ├── Cargo.toml
│   ├── host/                    # SP1 host program
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── main.rs
│   ├── program/                 # SP1 guest program (RISC-V)
│   │   ├── Cargo.toml
│   │   ├── elf/
│   │   └── src/
│   │       └── main.rs
│   └── lib/                     # Shared threshold signing logic
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── threshold.rs     # Threshold signing implementation
│           └── serialization.rs # Serialization helpers
└── solidity_threshold_signing/
    ├── contracts/
    │   └── ThresholdVerifier.sol
    ├── script/
    │   └── Deploy.s.sol
    ├── test/
    │   └── ThresholdVerifier.t.sol
    └── foundry.toml
```

## Implementation Steps

### Phase 1: Setup Project Structure

**Step 1.1**: Create directory structure
- Create `rust_threshold_signing/` directory
- Create `solidity_threshold_signing/` directory

**Step 1.2**: Initialize Rust workspace
- Create root `Cargo.toml` for workspace
- Set up `host/`, `program/`, and `lib/` subdirectories

**Step 1.3**: Initialize SP1 project
- Install SP1 toolchain if not present
- Configure SP1 build settings

**Step 1.4**: Initialize Foundry project
- Set up Foundry in `solidity_threshold_signing/`
- Configure for SP1 proof verification

### Phase 2: Implement Threshold Signing Library

**Step 2.1**: Create serialization helpers (`lib/src/serialization.rs`)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SignerMessage {
    pub signer_index: u8,
    pub message_hash: [u8; 32],
    pub nonce_commitment: [u8; 32],
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SignerResponse {
    pub signer_index: u8,
    pub signature_share: [u8; 32],
    pub nonce_share: [u8; 32],
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CombinedSignature {
    pub signature: [u8; 64],
    pub public_key: [u8; 32],
}

pub fn serialize<T: Serialize>(data: &T) -> Vec<u8> {
    bincode::serialize(data).expect("Serialization failed")
}

pub fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
    bincode::deserialize(bytes).expect("Deserialization failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signer_message_serialization_roundtrip() {
        let original = SignerMessage {
            signer_index: 1,
            message_hash: [42u8; 32],
            nonce_commitment: [99u8; 32],
        };

        let serialized = serialize(&original);
        let deserialized: SignerMessage = deserialize(&serialized);

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_signer_response_serialization_roundtrip() {
        let original = SignerResponse {
            signer_index: 2,
            signature_share: [123u8; 32],
            nonce_share: [45u8; 32],
        };

        let serialized = serialize(&original);
        let deserialized: SignerResponse = deserialize(&serialized);

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_combined_signature_serialization_roundtrip() {
        let original = CombinedSignature {
            signature: [77u8; 64],
            public_key: [88u8; 32],
        };

        let serialized = serialize(&original);
        let deserialized: CombinedSignature = deserialize(&serialized);

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_serialized_format_stability() {
        // Ensure serialization format is deterministic
        let msg = SignerMessage {
            signer_index: 5,
            message_hash: [1u8; 32],
            nonce_commitment: [2u8; 32],
        };

        let serialized1 = serialize(&msg);
        let serialized2 = serialize(&msg);

        assert_eq!(serialized1, serialized2);
    }

    #[test]
    fn test_different_values_produce_different_serialization() {
        let msg1 = SignerMessage {
            signer_index: 1,
            message_hash: [1u8; 32],
            nonce_commitment: [1u8; 32],
        };

        let msg2 = SignerMessage {
            signer_index: 2,
            message_hash: [1u8; 32],
            nonce_commitment: [1u8; 32],
        };

        let serialized1 = serialize(&msg1);
        let serialized2 = serialize(&msg2);

        assert_ne!(serialized1, serialized2);
    }
}
```

**Step 2.2**: Implement threshold signing logic (`lib/src/threshold.rs`)

This will use the `frost-ed25519` crate which implements the FROST (Flexible Round-Optimized Schnorr Threshold) protocol for Ed25519. The `frost-ed25519` crate internally uses `curve25519-dalek` for elliptic curve operations.

```rust
use frost_ed25519 as frost;

// Implementation using frost-ed25519 crate
// Key components:
// - Key generation using FROST's distributed key generation (DKG)
// - Round 1: Nonce commitment phase
// - Round 2: Signature share generation
// - Aggregation: Combine shares using FROST's aggregation logic
// - Output: Valid ed25519 signature verifiable by standard ed25519-dalek

pub struct ThresholdSigner {
    pub index: u8,
    pub secret_share: [u8; 32],
    pub threshold: u8,
    pub total_signers: u8,
}

impl ThresholdSigner {
    pub fn receive_serialized_signing_request(&self, serialized_msg: &[u8]) -> Vec<u8> {
        // TODO: Implement proper threshold signature share generation
        // 1. Deserialize signing request
        // 2. Generate nonce commitment
        // 3. Create signature share using secret share and FROST protocol
        // 4. Serialize and return response
        todo!("Implement real threshold signing")
    }
}

pub struct ThresholdCoordinator {
    pub threshold: u8,
    pub signers: Vec<ThresholdSigner>,
    pub combined_public_key: VerifyingKey,
}

impl ThresholdCoordinator {
    pub fn send_to_signer(&self, signer_index: usize, message: &[u8]) -> Vec<u8> {
        // TODO: Implement coordinator logic
        // 1. Serialize signing request with message
        // 2. Call receive_serialized_signing_request (simulating network)
        // 3. Return serialized signature share
        todo!("Implement coordinator send logic")
    }

    pub fn combine_signatures(&self, serialized_shares: Vec<Vec<u8>>) -> CombinedSignature {
        // TODO: Implement proper signature combination
        // 1. Deserialize all signature shares
        // 2. Verify threshold is met
        // 3. Use Lagrange interpolation to combine shares
        // 4. Produce valid ed25519 signature
        todo!("Implement Lagrange interpolation and signature combination")
    }
}
```

**Step 2.3**: Create lib.rs exports (`lib/src/lib.rs`)

```rust
pub mod threshold;
pub mod serialization;

pub use threshold::{ThresholdSigner, ThresholdCoordinator};
pub use serialization::{SignerMessage, SignerResponse, CombinedSignature, serialize, deserialize};
```

### Phase 3: Implement SP1 Guest Program (RISC-V)

**Step 3.1**: Create guest program (`program/src/main.rs`)

```rust
#![no_main]
sp1_zkvm::entrypoint!(main);

use threshold_signing_lib::{CombinedSignature, deserialize};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

pub fn main() {
    // Read inputs from SP1 stdin
    let message = sp1_zkvm::io::read::<Vec<u8>>();
    let combined_sig_bytes = sp1_zkvm::io::read::<Vec<u8>>();

    // Deserialize the combined signature
    let combined_sig: CombinedSignature = deserialize(&combined_sig_bytes);

    // Verify the signature inside zkVM
    let verifying_key = VerifyingKey::from_bytes(&combined_sig.public_key)
        .expect("Invalid public key");
    let signature = Signature::from_bytes(&combined_sig.signature);

    let is_valid = verifying_key.verify(&message, &signature).is_ok();

    // Write verification result to public output
    sp1_zkvm::io::commit(&is_valid);
    sp1_zkvm::io::commit(&combined_sig.public_key);
    sp1_zkvm::io::commit(&message);
}
```

**Step 3.2**: Configure guest Cargo.toml with proper dependencies

```toml
[package]
name = "threshold-signing-program"
version = "0.1.0"
edition = "2021"

[dependencies]
sp1-zkvm = { version = "3.0.0" }
threshold-signing-lib = { path = "../lib" }
ed25519-dalek = { version = "2.1", default-features = false, features = ["serde"] }
serde = { version = "1.0", default-features = false }
```

### Phase 4: Implement SP1 Host Program

**Step 4.1**: Create host program (`host/src/main.rs`)

```rust
use sp1_sdk::{ProverClient, SP1Stdin};
use threshold_signing_lib::{ThresholdSigner, ThresholdCoordinator, serialize};

fn main() {
    println!("=== Threshold Signature SP1 zkVM Demo ===\n");

    // Step 1: Setup threshold signing configuration
    let threshold = 3;
    let total_signers = 5;
    let message = b"Hello, threshold signatures in zkVM!";

    println!("Configuration:");
    println!("  Threshold: {}/{}", threshold, total_signers);
    println!("  Message: {:?}\n", String::from_utf8_lossy(message));

    // Step 2: Initialize signers with proper key generation
    // TODO: Use FROST or proper threshold key generation
    // - Generate master keypair
    // - Split into threshold shares using Shamir Secret Sharing
    // - Distribute shares to each ThresholdSigner
    // - Derive combined public key from shares

    let (signers, combined_public_key) = todo!("Implement proper key generation and distribution");

    let coordinator = ThresholdCoordinator {
        threshold,
        signers,
        combined_public_key,
    };

    // Step 3: Collect signature shares (demonstrating serialization)
    println!("Collecting signature shares...");
    let mut signature_shares = Vec::new();
    for i in 0..threshold as usize {
        println!("  Sending serialized request to signer {}", i);
        let serialized_response = coordinator.send_to_signer(i, message);
        println!("  Received serialized response from signer {}", i);
        signature_shares.push(serialized_response);
    }

    // Step 4: Combine signatures
    println!("\nCombining signature shares...");
    let combined_signature = coordinator.combine_signatures(signature_shares);
    let combined_sig_serialized = serialize(&combined_signature);
    println!("Combined signature created\n");

    // Step 5: Generate zkVM proof
    println!("Generating SP1 proof...");
    let client = ProverClient::new();
    let elf = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

    let mut stdin = SP1Stdin::new();
    stdin.write(&message.to_vec());
    stdin.write(&combined_sig_serialized);

    let (pk, vk) = client.setup(elf);
    let proof = client.prove(&pk, stdin).expect("Proving failed");

    println!("Proof generated successfully!\n");

    // Step 6: Verify proof
    println!("Verifying proof...");
    client.verify(&proof, &vk).expect("Verification failed");
    println!("Proof verified successfully!\n");

    // Step 7: Extract public outputs
    let is_valid = proof.public_values.read::<bool>();
    let public_key = proof.public_values.read::<[u8; 32]>();
    let message_out = proof.public_values.read::<Vec<u8>>();

    println!("=== Results ===");
    println!("Signature valid in zkVM: {}", is_valid);
    println!("Public key: {:?}", hex::encode(public_key));
    println!("Message: {:?}", String::from_utf8_lossy(&message_out));

    // Step 8: Save proof for Solidity verification
    println!("\nSaving proof for on-chain verification...");
    let proof_bytes = bincode::serialize(&proof).unwrap();
    std::fs::write("../solidity_threshold_signing/proof.bin", proof_bytes)
        .expect("Failed to write proof");

    let vk_bytes = bincode::serialize(&vk).unwrap();
    std::fs::write("../solidity_threshold_signing/vk.bin", vk_bytes)
        .expect("Failed to write verification key");

    println!("Proof and verification key saved!");
}
```

**Step 4.2**: Configure host Cargo.toml

```toml
[package]
name = "threshold-signing-host"
version = "0.1.0"
edition = "2021"

[dependencies]
sp1-sdk = "3.0.0"
threshold-signing-lib = { path = "../lib" }
ed25519-dalek = { version = "2.1", features = ["rand_core"] }
rand = "0.8"
bincode = "1.3"
hex = "0.4"
serde = { version = "1.0", features = ["derive"] }
```

### Phase 5: Implement Solidity Verifier

**Step 5.1**: Create SP1 verifier contract (`contracts/ThresholdVerifier.sol`)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract ThresholdSignatureVerifier {
    ISP1Verifier public immutable verifier;
    bytes32 public immutable programVKey;

    event SignatureVerified(
        bool isValid,
        bytes32 publicKey,
        bytes message
    );

    constructor(address _verifier, bytes32 _programVKey) {
        verifier = ISP1Verifier(_verifier);
        programVKey = _programVKey;
    }

    function verifyThresholdSignature(
        bytes calldata proof,
        bytes calldata publicValues
    ) external returns (bool) {
        // Verify the SP1 proof
        verifier.verifyProof(programVKey, publicValues, proof);

        // Decode public outputs
        (bool isValid, bytes32 publicKey, bytes memory message) =
            abi.decode(publicValues, (bool, bytes32, bytes));

        emit SignatureVerified(isValid, publicKey, message);

        return isValid;
    }

    function verifyThresholdSignatureView(
        bytes calldata publicValues
    ) external pure returns (bool isValid, bytes32 publicKey, bytes memory message) {
        (isValid, publicKey, message) = abi.decode(publicValues, (bool, bytes32, bytes));
    }
}
```

**Step 5.2**: Create deployment script (`script/Deploy.s.sol`)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {ThresholdSignatureVerifier} from "../contracts/ThresholdVerifier.sol";

contract DeployScript is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address sp1Verifier = vm.envAddress("SP1_VERIFIER");
        bytes32 programVKey = vm.envBytes32("PROGRAM_VKEY");

        vm.startBroadcast(deployerPrivateKey);

        ThresholdSignatureVerifier verifier = new ThresholdSignatureVerifier(
            sp1Verifier,
            programVKey
        );

        vm.stopBroadcast();

        console.log("ThresholdSignatureVerifier deployed at:", address(verifier));
    }
}
```

**Step 5.3**: Create test file (`test/ThresholdVerifier.t.sol`)

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {ThresholdSignatureVerifier} from "../contracts/ThresholdVerifier.sol";

contract ThresholdVerifierTest is Test {
    ThresholdSignatureVerifier public verifier;

    function setUp() public {
        // Mock SP1 verifier address
        address mockVerifier = address(0x1);
        bytes32 mockVKey = bytes32(uint256(1));

        verifier = new ThresholdSignatureVerifier(mockVerifier, mockVKey);
    }

    function test_DeploymentSuccessful() public {
        assertEq(address(verifier.verifier()), address(0x1));
    }
}
```

**Step 5.4**: Configure foundry.toml

```toml
[profile.default]
src = "contracts"
out = "out"
libs = ["lib"]
solc_version = "0.8.20"
evm_version = "paris"

[rpc_endpoints]
sepolia = "${SEPOLIA_RPC_URL}"
```

### Phase 6: Build Scripts and Automation

**Step 6.1**: Create build script (`rust_threshold_signing/scripts/build.sh`)

```bash
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
```

**Step 6.2**: Create run script (`rust_threshold_signing/scripts/run.sh`)

```bash
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
```

**Step 6.3**: Create test script (`rust_threshold_signing/scripts/test.sh`)

```bash
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
```

**Step 6.4**: Create Solidity build script (`solidity_threshold_signing/scripts/build.sh`)

```bash
#!/bin/bash
set -e

echo "Building Solidity contracts..."

# Install dependencies
forge install

# Build contracts
forge build

echo "Solidity build complete!"
```

**Step 6.5**: Create Solidity test script (`solidity_threshold_signing/scripts/test.sh`)

```bash
#!/bin/bash
set -e

echo "Testing Solidity contracts..."

forge test -vvv

echo "Solidity tests passed!"
```

**Step 6.6**: Create deployment script (`solidity_threshold_signing/scripts/deploy.sh`)

```bash
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
```

**Step 6.7**: Create end-to-end script (`scripts/e2e.sh` in project root)

```bash
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
```

**Step 6.8**: Create setup script (`scripts/setup.sh` in project root)

```bash
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
```

**Step 6.9**: Create clean script (`scripts/clean.sh` in project root)

```bash
#!/bin/bash
set -e

echo "Cleaning build artifacts..."

# Clean Rust artifacts
cd rust_threshold_signing
cargo clean
rm -rf program/elf/*.bin

# Clean Solidity artifacts
cd ../solidity_threshold_signing
forge clean
rm -f proof.bin vk.bin

echo "Clean complete!"
```

**Step 6.10**: Update project structure to include scripts

```
poc_threshold_signing_rust/
├── scripts/
│   ├── setup.sh           # Initial setup
│   ├── e2e.sh            # End-to-end test
│   └── clean.sh          # Clean artifacts
├── rust_threshold_signing/
│   └── scripts/
│       ├── build.sh      # Build Rust components
│       ├── run.sh        # Run proof generation
│       └── test.sh       # Test Rust code
└── solidity_threshold_signing/
    └── scripts/
        ├── build.sh      # Build contracts
        ├── test.sh       # Test contracts
        └── deploy.sh     # Deploy to network
```

### Phase 7: Documentation and Configuration

**Step 7.1**: Create example .env file (`solidity_threshold_signing/.env.example`)

```bash
# Private key for deployment (without 0x prefix)
PRIVATE_KEY=your_private_key_here

# SP1 Verifier contract address (network-specific)
SP1_VERIFIER=0x...

# Program verification key from SP1 build
PROGRAM_VKEY=0x...

# RPC URL for deployment
RPC_URL=https://sepolia.infura.io/v3/YOUR_INFURA_KEY

# Optional: Etherscan API key for verification
ETHERSCAN_API_KEY=your_etherscan_api_key
```

**Step 7.2**: Create README with usage instructions and compilation steps

Include sections:
- Prerequisites (Rust, SP1, Foundry)
- Quick Start (`./scripts/setup.sh` then `./scripts/e2e.sh`)
- Build instructions for each component
- Running the proof generation
- Deploying to blockchain
- Troubleshooting

**Step 7.3**: Document serialization format
- Explain SignerMessage, SignerResponse, CombinedSignature structures
- Show how data flows through the system

**Step 7.4**: Document how to extend to real network
- Replace simulated send/receive with actual HTTP/gRPC calls
- Security considerations for distributed deployment

## Shell Script Summary

The following scripts will be created to automate the build and deployment process:

### Project Root Scripts (`scripts/`)
- **`setup.sh`**: One-time setup - installs dependencies and makes scripts executable
- **`e2e.sh`**: End-to-end test - builds and tests all components
- **`clean.sh`**: Removes all build artifacts

### Rust Scripts (`rust_threshold_signing/scripts/`)
- **`build.sh`**: Compiles library, guest program (RISC-V), and host
- **`run.sh`**: Generates threshold signature and SP1 proof
- **`test.sh`**: Runs all Rust unit tests

### Solidity Scripts (`solidity_threshold_signing/scripts/`)
- **`build.sh`**: Compiles smart contracts with Foundry
- **`test.sh`**: Runs Foundry tests
- **`deploy.sh`**: Deploys verifier contract to blockchain

### Usage Flow
```bash
# First time setup
./scripts/setup.sh

# Run complete system
./scripts/e2e.sh

# Or run components individually
cd rust_threshold_signing
./scripts/build.sh
./scripts/run.sh

cd ../solidity_threshold_signing
./scripts/build.sh
./scripts/test.sh
./scripts/deploy.sh  # After configuring .env
```

## Dependencies

### Rust Dependencies
- `sp1-sdk`: SP1 proving system
- `sp1-zkvm`: SP1 zkVM runtime
- `frost-ed25519`: FROST threshold signature implementation (uses `curve25519-dalek` internally)
- `ed25519-dalek`: Ed25519 signature verification
- `curve25519-dalek`: Elliptic curve operations (dependency of frost-ed25519)
- `serde`: Serialization framework
- `bincode`: Binary serialization
- `rand`: Random number generation

### Solidity Dependencies
- `@sp1-contracts/ISP1Verifier`: SP1 verifier interface
- Foundry toolchain

## Execution Order

1. Create directory structure
2. Set up Cargo workspace and dependencies
3. Implement threshold signing library
4. Implement SP1 guest program
5. Implement SP1 host program
6. Build and test Rust components
7. Set up Foundry project
8. Implement Solidity verifier
9. Generate proof with host program
10. Deploy and test Solidity verifier
11. End-to-end integration test

## Success Criteria

- [ ] Real threshold signing implementation using FROST or equivalent
- [ ] Serialization/deserialization demonstrates network-ready message format
- [ ] SP1 guest program successfully verifies real ed25519 threshold signatures
- [ ] STARK proof generation completes successfully
- [ ] Solidity verifier accepts valid proofs
- [ ] All components integrated and tested end-to-end
- [ ] Code is well-documented and ready for network extension

## Notes

- Uses `frost-ed25519` crate for production-grade threshold signatures
- `frost-ed25519` internally uses `curve25519-dalek` for elliptic curve operations
- The resulting signatures are standard Ed25519 signatures verifiable by `ed25519-dalek`
- Network layer can be added by replacing serialize/deserialize calls with actual network requests
- Security audit required before production use

## Implementation Libraries

### Core Cryptography Stack
- **`frost-ed25519`**: Implements FROST threshold signature protocol for Ed25519
  - Provides distributed key generation (DKG)
  - Manages nonce commitments and signature shares
  - Handles Lagrange interpolation for signature aggregation
- **`curve25519-dalek`**: Underlying elliptic curve library (used by frost-ed25519)
  - Provides Ed25519 curve arithmetic
  - Scalar and point operations
  - Field arithmetic over Curve25519
- **`ed25519-dalek`**: Standard Ed25519 signature verification
  - Used in SP1 zkVM to verify the final combined signature

### Research Resources
- FROST paper: https://eprint.iacr.org/2020/852.pdf
- FROST Ed25519 spec: https://datatracker.ietf.org/doc/draft-irtf-cfrg-frost/
- curve25519-dalek docs: https://docs.rs/curve25519-dalek/
