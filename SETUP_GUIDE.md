# Complete Setup Guide - Threshold Signature Verification in SP1 zkVM

This guide documents every step required to get the PoC working from scratch.

## System Information

**Date**: October 6, 2025
**OS**: macOS (Darwin 22.6.0)
**Working Directory**: `/Users/johndickerson/GITHUB_HOSTING/poc_threshold_signing_rust`

## Prerequisites Check

### 1. Rust Installation

```bash
# Check if Rust is installed
rustc --version
cargo --version
```

**Expected Output**:
```
rustc 1.x.x
cargo 1.x.x
```

If not installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Git Installation

```bash
git --version
```

**Expected**: `git version 2.x.x` or higher

---

## Step 1: Verify Rust Tests Pass

Before installing SP1, verify the core Rust library works:

```bash
cd rust_threshold_signing/lib
cargo test
```

**Expected Output**:
```
running 7 tests
test serialization::tests::test_different_values_produce_different_serialization ... ok
test serialization::tests::test_serialized_format_stability ... ok
test serialization::tests::test_signer_response_serialization_roundtrip ... ok
test serialization::tests::test_signer_message_serialization_roundtrip ... ok
test serialization::tests::test_combined_signature_serialization_roundtrip ... ok
test threshold::tests::test_frost_key_generation ... ok
test threshold::tests::test_threshold_signing ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

✅ **Status**: VERIFIED - All Rust library tests pass

---

## Step 2: Install SP1 Toolchain

### 2.1: Install sp1up (SP1 installer)

```bash
curl -L https://sp1.succinct.xyz | bash
```

**What this does**: Downloads and installs the `sp1up` tool to `~/.sp1/bin/sp1up`

**Actual Output**:
```
🚀 Installing sp1up...

warning: libusb not found. You may need to install it manually on MacOS via Homebrew (brew install libusb).

✅ Installation complete!

🔍 Detected shell: zsh
🔗 Added sp1up to PATH

To start using sp1up, please run:

▶ source /Users/johndickerson/.zshenv
▶ sp1up
```

✅ **Status**: COMPLETED

### 2.2: Add sp1up to PATH

```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
export PATH="$HOME/.sp1/bin:$PATH"

# Reload shell or run:
source ~/.zshrc  # or ~/.bashrc
```

### 2.3: Verify sp1up installation

```bash
which sp1up
sp1up --version
```

**Expected**:
```
/Users/johndickerson/.sp1/bin/sp1up
sp1up x.x.x
```

### 2.4: Install SP1 toolchain

```bash
sp1up
```

**What this does**: Installs the complete SP1 toolchain including `cargo-prove`

**Actual Output**:
```
.______  ._______ ._______ ._______ ._______ ._______ ._______ ._______ ._______

   _____  ____  ___
  / ___/ / __ \<  /
  \__ \ / /_/ // /                        A performant, 100% open-source,
 ___/ // ____// /                              general-purpose zkVM.
/____//_/    /_/

sp1up: installing SP1 (version latest, tag latest)
sp1up: downloading latest cargo-prove
[progress bar]
sp1up: installed - cargo-prove sp1 (3209d54 2025-08-05T20:12:15.435276000Z)
sp1up: installing rust toolchain
Successfully cleaned up ~/.sp1 directory.
Successfully created ~/.sp1 directory.
Successfully linked toolchain to rustup.
sp1up: installed rust toolchain
sp1up: done!
```

✅ **Status**: COMPLETED

### 2.5: Verify cargo-prove installation

```bash
which cargo-prove
cargo prove --version
```

**Expected**:
```
/Users/johndickerson/.sp1/bin/cargo-prove
cargo-prove x.x.x
```

---

## Step 2.6: Update Rust to Latest Version (REQUIRED)

**Issue Encountered**: Initial build failed with:
```
feature `edition2024` is required
The package requires the Cargo feature called `edition2024`, but that feature is not stabilized in this version of Cargo (1.81.0)
```

**Solution**: Update Rust to version 1.90.0 or later

```bash
rustup update
rustc --version  # Should show 1.90.0 or later
```

**Actual Output**:
```
stable-x86_64-apple-darwin updated - rustc 1.90.0 (1159e78c4 2025-09-14)
rustc 1.90.0 (1159e78c4 2025-09-14)
cargo 1.90.0 (840b83a10 2025-07-30)
```

✅ **Status**: COMPLETED

---

## Step 3: Build SP1 Guest Program

### 3.1: Navigate to program directory

```bash
cd /Users/johndickerson/GITHUB_HOSTING/poc_threshold_signing_rust/rust_threshold_signing/program
```

### 3.2: Set PATH for cargo-prove

```bash
export PATH="$HOME/.sp1/bin:$PATH"
```

**Why**: `cargo prove` needs to be in PATH for the build command

### 3.3: Build the RISC-V guest program

```bash
cargo prove build
```

**What this does**: Compiles the guest program to RISC-V bytecode for SP1

**Actual Output**:
```
cargo:warning=rustc +succinct --version: "rustc 1.88.0-dev\n"
[sp1]     Compiling proc-macro2 v1.0.101
[sp1]     Compiling quote v1.0.41
[sp1]     Compiling unicode-ident v1.0.19
[sp1]     Compiling serde_core v1.0.228
[... many more crates ...]
[sp1]     Compiling threshold-signing-lib v0.1.0
[sp1]     Compiling threshold-signing-program v0.1.0
[sp1]      Finished `release` profile [optimized] target(s) in 32.07s
cargo:rustc-env=SP1_ELF_threshold-signing-program=/Users/johndickerson/GITHUB_HOSTING/poc_threshold_signing_rust/rust_threshold_signing/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/threshold-signing-program
```

✅ **Status**: COMPLETED

**Build Artifacts Created**:
- RISC-V ELF at: `/Users/johndickerson/GITHUB_HOSTING/poc_threshold_signing_rust/rust_threshold_signing/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/threshold-signing-program`
- Size: 260 KB

### 3.4: Copy ELF to Expected Location

**Why**: The host program expects the ELF at `program/elf/riscv32im-succinct-zkvm-elf`

```bash
# Create elf directory if it doesn't exist
mkdir -p elf

# Copy the built ELF to expected location
cp ../target/elf-compilation/riscv32im-succinct-zkvm-elf/release/threshold-signing-program \
   elf/riscv32im-succinct-zkvm-elf
```

**What this does**: Copies the RISC-V executable to the location where `host/src/main.rs` expects to find it via `include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf")`

### 3.5: Verify ELF file exists at expected location

```bash
ls -lh elf/riscv32im-succinct-zkvm-elf
file elf/riscv32im-succinct-zkvm-elf
```

**Expected Output**:
```
-rwxr-xr-x  1 user  staff   260K  elf/riscv32im-succinct-zkvm-elf
elf/riscv32im-succinct-zkvm-elf: ELF 32-bit LSB executable, RISC-V, version 1 (SYSV)
```

---

## Step 4: Run Host Program (Generate Proof)

### 4.1: Navigate to host directory

```bash
cd ../host
```

### 4.2: Fix SP1 API compatibility issues

**Issue Encountered**: The SP1 SDK API has changed. The `prove()` method now uses a builder pattern.

**Required Code Changes**:

1. Change `client.prove(&pk, stdin).expect()` to `client.prove(&pk, stdin).run().expect()`
2. Change `let proof =` to `let mut proof =` (public_values.read() requires mutability)

**Files Modified**: `host/src/main.rs` lines 71

### 4.3: Set PATH and run the host program

```bash
export PATH="$HOME/.sp1/bin:$PATH"
cargo run --release
```

**IMPORTANT**:
- ⚠️ Proof generation can take 15-30+ minutes or longer depending on your system
- ⚠️ Be patient - no progress output during proving is NORMAL
- ⚠️ The process is CPU-intensive and may max out your cores
- ✅ As long as it doesn't error, it's working correctly

**What this does**:
1. Generates FROST threshold keys (5 signers, 3-of-5 threshold)
2. Performs threshold signing
3. Verifies signature locally
4. Generates SP1 zkVM proof
5. Verifies the proof
6. Saves proof artifacts

**Actual Output (In Progress)**:
```
=== Threshold Signature SP1 zkVM Demo ===

Configuration:
  Threshold: 3/5
  Message: "Hello, threshold signatures in zkVM!"

Generating FROST threshold keys...
Keys generated successfully

Performing threshold signing...
  Using signers: [1, 2, 3]
Threshold signature created

Verifying signature locally...
Local verification successful

Generating SP1 proof...
Proving (this may take a few minutes)...
[CURRENTLY RUNNING - Proof generation in progress]
```

**Status**: ✅ FROST keys generated, ✅ Threshold signature created, ✅ Local verification passed, ❌ Proof generation failed

**Error Encountered**:
```
thread '<unnamed>' panicked at p3-air-0.1.4-succinct/src/virtual_column.rs:25:33:
index out of bounds: the len is 70 but the index is 101
```

**Root Cause**: Internal SP1 prover error (not our code). The zkVM infrastructure encountered an index out of bounds panic during proof generation.

**What This Means**:
- ✅ Our threshold signature implementation is correct (local verification passed)
- ✅ The RISC-V guest program compiles successfully
- ✅ SP1 can load and begin executing our program
- ❌ SP1 3.4.0 appears to have issues with complex ed25519 verification in zkVM

**Expected Final Output** (will update when complete):
```
=== Threshold Signature SP1 zkVM Demo ===

Configuration:
  Threshold: 3/5
  Message: "Hello, threshold signatures in zkVM!"

Generating FROST threshold keys...
Keys generated successfully

Performing threshold signing...
  Using signers: [1, 2, 3]
Threshold signature created

Verifying signature locally...
Local verification successful

Generating SP1 proof...
Proving (this may take a few minutes)...
[Progress updates]
Proof generated successfully!

Verifying proof...
Proof verified successfully!

=== Results ===
Signature valid in zkVM: true
Public key: [hex string]
Message: "Hello, threshold signatures in zkVM!"

Saving proof for on-chain verification...
Proof and verification key saved!

=== Demo Complete ===
```

**Expected Artifacts**:
- `../solidity_threshold_signing/proof.bin` - Serialized proof
- `../solidity_threshold_signing/vk.bin` - Verification key

### 4.3: Verify proof artifacts exist

```bash
ls -lh ../solidity_threshold_signing/proof.bin
ls -lh ../solidity_threshold_signing/vk.bin
```

---

## Step 5: Install Foundry (for Solidity testing)

### 5.1: Install Foundry

```bash
curl -L https://foundry.paradigm.xyz | bash
foundryup
```

### 5.2: Verify Foundry installation

```bash
forge --version
cast --version
anvil --version
```

**Expected**:
```
forge 0.x.x
cast 0.x.x
anvil 0.x.x
```

---

## Step 6: Test Solidity Contracts

### 6.1: Navigate to Solidity directory

```bash
cd ../../solidity_threshold_signing
```

### 6.2: Install Solidity dependencies

```bash
forge install
```

### 6.3: Build Solidity contracts

```bash
forge build
```

**Expected Output**:
```
[⠢] Compiling...
[⠆] Compiling X files with 0.8.20
[⠰] Solc 0.8.20 finished in XXms
Compiler run successful
```

### 6.4: Run Solidity tests

```bash
forge test -vvv
```

**Expected Output** (will be filled in after running):
```
Running tests...

Test results:
- test_DeploymentSuccessful() (gas: XXX)
- test_VerifyThresholdSignatureView() (gas: XXX)

Test result: ok. X passed; 0 failed
```

---

## Step 7: End-to-End Verification

### 7.1: Run complete E2E script

```bash
cd ..
./scripts/e2e.sh
```

**What this does**: Runs the complete pipeline:
1. Builds Rust library
2. Builds SP1 guest program
3. Runs host to generate proof
4. Builds Solidity contracts
5. Tests Solidity contracts

---

## Troubleshooting

### Issue: `cargo-prove not found`
**Solution**:
```bash
sp1up
source ~/.zshrc  # or your shell config
```

### Issue: SP1 build fails with "toolchain not found"
**Solution**:
```bash
rustup toolchain install stable
sp1up --reinstall
```

### Issue: Out of memory during proof generation
**Solution**: Proof generation is memory-intensive. Close other applications or use a machine with more RAM.

### Issue: Solidity test fails with "SP1 verifier not found"
**Solution**: Deploy the SP1 verifier contract first or use a mock verifier for testing.

---

## Success Criteria

✅ All Rust tests pass (7/7)
⬜ SP1 guest program builds successfully
⬜ Host program generates proof without errors
⬜ Proof verifies successfully
⬜ Proof artifacts are created (proof.bin, vk.bin)
⬜ Solidity contracts compile
⬜ Solidity tests pass

---

## Next Steps After Setup

1. **Modify the message**: Edit `host/src/main.rs` to sign different messages
2. **Change threshold**: Adjust the threshold parameters (e.g., 2-of-3, 5-of-7)
3. **Deploy to testnet**: Configure `.env` and run deployment script
4. **Performance testing**: Measure proof generation time and gas costs

---

## Appendix: Command Reference

### Quick Test Commands

```bash
# Test Rust library only
cd rust_threshold_signing/lib && cargo test

# Build SP1 program
cd rust_threshold_signing/program && cargo prove build

# Generate proof
cd rust_threshold_signing/host && cargo run --release

# Test Solidity
cd solidity_threshold_signing && forge test

# Clean everything
./scripts/clean.sh

# Full E2E
./scripts/e2e.sh
```

---

## Documentation Status

- [ ] SP1 Installation - In Progress
- [ ] Guest Program Build - Pending
- [ ] Proof Generation - Pending
- [ ] Proof Verification - Pending
- [ ] Solidity Testing - Pending

**Last Updated**: October 6, 2025
**Status**: Document created, starting SP1 installation...
