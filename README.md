# Threshold Signature Verification in SP1 zkVM

This project demonstrates threshold signature verification using FROST (Flexible Round-Optimized Schnorr Threshold) signatures for Ed25519 within the SP1 zkVM. It generates zero-knowledge proofs of signature verification that can be verified on-chain.

## Overview

The system consists of three main components:

1. **Threshold Signing Library** (`rust_threshold_signing/lib/`) - Implements FROST threshold signatures using `frost-ed25519`
2. **SP1 zkVM Program** (`rust_threshold_signing/program/`) - Verifies threshold signatures inside the zkVM and generates STARK proofs
3. **Solidity Verifier** (`solidity_threshold_signing/`) - On-chain verification of SP1 proofs

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  Threshold Signing Library (FROST Protocol)                 │
│  - Distributed Key Generation (DKG)                         │
│  - Round 1: Nonce commitments                               │
│  - Round 2: Signature shares                                │
│  - Aggregation: Lagrange interpolation                      │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  SP1 Host Program                                           │
│  - Generate threshold keys (3-of-5)                         │
│  - Coordinate signing process                               │
│  - Combine signature shares                                 │
│  - Generate zkVM proof                                      │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  SP1 Guest Program (RISC-V zkVM)                            │
│  - Deserialize signature and message                        │
│  - Verify Ed25519 signature                                 │
│  - Output public values (is_valid, pubkey, message)         │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  Solidity Verifier Contract                                 │
│  - Verify SP1 STARK proof                                   │
│  - Extract public values                                    │
│  - Emit verification event                                  │
└─────────────────────────────────────────────────────────────┘
```

## Prerequisites

- **Rust** (1.75+) - Install from [rustup.rs](https://rustup.rs/)
- **SP1 Toolchain** - Installed automatically by `./scripts/setup.sh`
- **Foundry** - Installed automatically by `./scripts/setup.sh`

## Quick Start

### 1. Setup Development Environment

```bash
./scripts/setup.sh
```

This will:
- Check for Rust installation
- Install SP1 toolchain if needed
- Install Foundry if needed
- Make all scripts executable

### 2. Run End-to-End Test

```bash
./scripts/e2e.sh
```

This will:
- Build Rust library, SP1 guest program, and host program
- Generate FROST threshold keys (3-of-5)
- Perform threshold signing
- Generate SP1 zero-knowledge proof
- Build and test Solidity contracts

### 3. Deploy to Blockchain (Optional)

First, configure your environment variables:

```bash
cd solidity_threshold_signing
cp .env.example .env
# Edit .env with your values
```

Then deploy:

```bash
./scripts/deploy.sh
```

## Project Structure

```
poc_threshold_signing_rust/
├── scripts/                           # Top-level automation scripts
│   ├── setup.sh                      # One-time development setup
│   ├── e2e.sh                        # End-to-end test
│   └── clean.sh                      # Clean build artifacts
├── rust_threshold_signing/
│   ├── lib/                          # Threshold signing library
│   │   └── src/
│   │       ├── threshold.rs          # FROST implementation
│   │       └── serialization.rs      # Network-ready serialization
│   ├── program/                      # SP1 guest program (RISC-V)
│   │   └── src/main.rs              # Signature verification in zkVM
│   ├── host/                         # SP1 host program
│   │   └── src/main.rs              # Proof generation orchestrator
│   └── scripts/
│       ├── build.sh                  # Build Rust components
│       ├── run.sh                    # Run proof generation
│       └── test.sh                   # Test Rust code
└── solidity_threshold_signing/
    ├── contracts/
    │   └── ThresholdVerifier.sol     # SP1 proof verifier
    ├── script/
    │   └── Deploy.s.sol              # Deployment script
    ├── test/
    │   └── ThresholdVerifier.t.sol   # Contract tests
    └── scripts/
        ├── build.sh                  # Build contracts
        ├── test.sh                   # Test contracts
        └── deploy.sh                 # Deploy to network
```

## How It Works

### 1. Threshold Key Generation

The system uses FROST's trusted dealer key generation for simplicity:

```rust
let (key_packages, pubkey_package) = generate_frost_keys(5, 3)?;
```

This generates:
- 5 secret key shares (one per signer)
- A shared public key
- Uses trusted dealer method (simpler for PoC, still produces valid FROST signatures)

**Note**: For production, implement the full Distributed Key Generation (DKG) protocol which doesn't require a trusted party. The DKG implementation skeleton is included in the code for future development.

### 2. Threshold Signing Process

**Round 1: Nonce Commitment**
```rust
let commitments = signer.round1_generate_nonces();
```

**Round 2: Signature Share Generation**
```rust
let share = signer.round2_sign(message, &signing_package)?;
```

**Aggregation: Lagrange Interpolation**
```rust
let signature = frost::aggregate(&signing_package, &shares, &pubkey)?;
```

### 3. Zero-Knowledge Proof Generation

The SP1 host program:
1. Coordinates threshold signing with 3-of-5 signers
2. Combines signature shares into final Ed25519 signature
3. Serializes signature and message
4. Passes to SP1 zkVM for proof generation

The SP1 guest program:
1. Receives signature and message
2. Verifies Ed25519 signature
3. Outputs public values (is_valid, pubkey, message)
4. SP1 generates STARK proof

### 4. On-Chain Verification

```solidity
function verifyThresholdSignature(
    bytes calldata proof,
    bytes calldata publicValues
) external returns (bool)
```

The Solidity contract verifies the STARK proof, confirming that:
- The signature was valid in the zkVM
- The public key matches
- The message matches

## Serialization Format

All messages use `bincode` for deterministic binary serialization:

**SignerMessage** - Coordinator → Signer
```rust
struct SignerMessage {
    signer_index: u8,
    message_hash: [u8; 32],
    nonce_commitment: [u8; 32],
}
```

**SignerResponse** - Signer → Coordinator
```rust
struct SignerResponse {
    signer_index: u8,
    signature_share: [u8; 32],
    nonce_share: [u8; 32],
}
```

**CombinedSignature** - Final Output
```rust
struct CombinedSignature {
    signature: [u8; 64],    // Ed25519 signature
    public_key: [u8; 32],   // Group public key
}
```

## Extending to Real Network

Currently, the "network" communication is simulated via direct function calls. To deploy in a distributed setting:

1. **Replace serialization with HTTP/gRPC**:
   ```rust
   // Instead of:
   let response = signer.receive_serialized_signing_request(&request);

   // Use:
   let response = http_client.post("/sign", request).await?;
   ```

2. **Add authentication**: Use TLS certificates or signatures to authenticate signers

3. **Implement timeout and retry logic**: Handle network failures gracefully

4. **Add state management**: Track signing sessions across multiple rounds

## Dependencies and SP1 Compatibility

### Standard vs. Patched Dependencies

This project uses **production-grade, unmodified threshold signature libraries**. The only modification is at the low-level cryptographic primitives layer to enable efficient proof generation in SP1 zkVM.

**✅ Standard (unmodified from crates.io):**
- `frost-ed25519` v2.2.0 - FROST threshold signature implementation
- `ed25519-dalek` v2.1 - Ed25519 signature verification
- All application logic and threshold signing protocols

**🔧 SP1-Patched (drop-in replacement):**
- `curve25519-dalek` v4.1.3 - Low-level elliptic curve operations
  - Uses SP1's optimized version from `https://github.com/sp1-patches/curve25519-dalek`
  - Same API, but includes precompile hooks for zkVM
  - 5-10x faster proof generation via specialized STARK tables
  - Applied via workspace-level `[patch.crates-io]` in `Cargo.toml`

**Why this matters:**
- The threshold signature logic itself is **standard and audited** (frost-ed25519)
- Only low-level math operations are optimized for zkVM
- The patch is **transparent** - frost-ed25519 doesn't know it's using a patched version
- Easy to replicate for other cryptographic protocols in SP1

**How to apply the patch:**

Add to your workspace `Cargo.toml`:
```toml
[patch.crates-io]
curve25519-dalek = { git = "https://github.com/sp1-patches/curve25519-dalek", tag = "patch-v4.1.3-v3.4.0" }
```

### Full Dependency List

**Rust:**
- `frost-ed25519` - FROST threshold signature implementation
- `curve25519-dalek` - Elliptic curve operations (SP1-patched version)
- `ed25519-dalek` - Ed25519 signature verification
- `sp1-sdk` / `sp1-zkvm` - SP1 zero-knowledge proving system
- `serde` / `bincode` - Serialization

**Solidity:**
- `@sp1-contracts/ISP1Verifier` - SP1 proof verifier interface
- Foundry - Smart contract development toolchain

## Testing

### Test Rust Components
```bash
cd rust_threshold_signing
./scripts/test.sh
```

### Test Solidity Contracts
```bash
cd solidity_threshold_signing
./scripts/test.sh
```

### Run All Tests
```bash
./scripts/e2e.sh
```

## Build Artifacts

After running the system, you'll find:

- `rust_threshold_signing/program/elf/riscv32im-succinct-zkvm-elf` - Compiled RISC-V program
- `solidity_threshold_signing/proof.bin` - Generated STARK proof
- `solidity_threshold_signing/vk.bin` - Verification key

## Security Considerations

⚠️ **This is a proof-of-concept for educational purposes**

Before production use:

1. **Security audit**: Have the code reviewed by cryptography experts
2. **Key management**: Implement secure key generation and storage
3. **Network security**: Use TLS and proper authentication
4. **Denial of service**: Add rate limiting and timeouts
5. **Signer selection**: Implement secure signer discovery and selection
6. **Proof costs**: Optimize proof generation for on-chain verification costs

## Resources

- [FROST Paper](https://eprint.iacr.org/2020/852.pdf)
- [FROST Ed25519 Spec](https://datatracker.ietf.org/doc/draft-irtf-cfrg-frost/)
- [SP1 Documentation](https://docs.succinct.xyz/)
- [frost-ed25519 Docs](https://docs.rs/frost-ed25519/)

## License

MIT

## Contributing

This is a proof-of-concept project. Feel free to fork and extend for your use case.
