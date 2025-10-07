# Solidity Contract Deployment Guide

Complete guide for deploying and testing the threshold signature verifier on Ethereum testnets.

---

## Understanding What Gets Sent to Solidity

### One-Time (Contract Deployment):

**Verification Key** (`vk.bin` → `bytes32 programVKey`)
- 256 bytes converted to bytes32
- Stored in the contract as `bytes32 public immutable programVKey`
- Passed to constructor at deployment
- **Cost**: Stored once on-chain (cheap since it's only 256 bytes)

### Every Verification Transaction:

**Two parameters sent to `verifyThresholdSignature()` function:**

1. **`bytes calldata proof`** - The STARK proof from `proof.bin`
   - Size: 7.4 MB (large!)
   - **NOT stored on-chain** - only passed as calldata
   - Contains the cryptographic proof
   - **Cost**: High gas for calldata (but not stored permanently)

2. **`bytes calldata publicValues`** - Public outputs from zkVM
   - Contains: `(bool isValid, bytes32 publicKey, bytes message)`
   - These are the values that were "committed" in the guest program:
     ```rust
     sp1_zkvm::io::commit(&is_valid);              // → bool
     sp1_zkvm::io::commit(&combined_sig.public_key);  // → bytes32
     sp1_zkvm::io::commit(&message);                // → bytes
     ```

### What Happens On-Chain:

```solidity
verifier.verifyProof(programVKey, publicValues, proof);
```

The SP1 verifier contract checks:
1. ✅ Does this proof correspond to this programVKey?
2. ✅ Does the proof correctly prove the publicValues?
3. ✅ Is the proof cryptographically valid?

If all checks pass, the transaction succeeds and emits:
```solidity
emit SignatureVerified(isValid, publicKey, message);
```

**Summary**:
- **Deploy once**: vk.bin embedded in contract
- **Each call**: proof.bin + publicValues sent as calldata (not stored)

---

## Prerequisites

### 1. Install Foundry

```bash
# Install Foundry
curl -L https://foundry.paradigm.xyz | bash

# Activate Foundry in current shell
source ~/.bashrc  # or ~/.zshrc

# Install the latest version
foundryup
```

**Verify installation**:
```bash
forge --version
cast --version
```

**Expected**: `forge 0.2.0` or higher

### 2. Create or Import Wallet

**Option A: Create New Wallet**
```bash
cast wallet new
```

**Save the output** (private key and address) in a secure location!

**Option B: Use Existing Wallet**

You'll need your private key. Export it from:
- MetaMask: Account Details → Export Private Key
- Hardware wallet: Not recommended for testnet

⚠️ **IMPORTANT**: Never use mainnet wallets with real funds for testing!

---

## Step 1: Get Sepolia Testnet ETH

You need Sepolia ETH to deploy contracts and pay for gas.

### Recommended Faucets:

**1. Alchemy Sepolia Faucet** (Most Reliable)
- URL: https://sepoliafaucet.com/
- Requirements: Alchemy account (free)
- Amount: 0.5 SepoliaETH per day
- Instructions:
  1. Sign up at https://www.alchemy.com/
  2. Go to https://sepoliafaucet.com/
  3. Login with Alchemy account
  4. Enter your wallet address
  5. Click "Send Me ETH"

**2. QuickNode Faucet**
- URL: https://faucet.quicknode.com/ethereum/sepolia
- Requirements: QuickNode account (free)
- Amount: 0.05 SepoliaETH per day

**3. Infura Faucet**
- URL: https://www.infura.io/faucet/sepolia
- Requirements: Infura account (free)
- Amount: 0.5 SepoliaETH per day

**4. Chainlink Faucet**
- URL: https://faucets.chain.link/sepolia
- Requirements: GitHub/Google account
- Amount: 0.1 SepoliaETH per request

### Verify You Received Funds:

```bash
# Replace with your wallet address
cast balance 0xYourAddressHere --rpc-url https://eth-sepolia.g.alchemy.com/v2/demo
```

**Expected**: Should show balance > 0 (in wei, 1 ETH = 10^18 wei)

---

## Step 2: Get RPC URL

You need an RPC endpoint to interact with Sepolia testnet.

### Recommended Providers (All Free):

**Option A: Alchemy** (Recommended)
1. Sign up at https://www.alchemy.com/
2. Create new app → Select "Ethereum" → Select "Sepolia"
3. Copy the HTTPS URL (looks like: `https://eth-sepolia.g.alchemy.com/v2/YOUR_API_KEY`)

**Option B: Infura**
1. Sign up at https://infura.io/
2. Create new project
3. Select "Sepolia" from network dropdown
4. Copy the HTTPS endpoint

**Option C: Public RPC** (Rate Limited)
- `https://rpc.sepolia.org/` (may be slow/unreliable)

---

## Step 3: Configure Environment

### 3.1: Create .env file

```bash
cd solidity_threshold_signing
cp .env.example .env
```

### 3.2: Edit .env file

Open `.env` in your editor and fill in:

```bash
# Your wallet private key (WITHOUT 0x prefix)
PRIVATE_KEY=your_private_key_here_without_0x

# Sepolia RPC URL
SEPOLIA_RPC_URL=https://eth-sepolia.g.alchemy.com/v2/YOUR_API_KEY

# Etherscan API key for contract verification (optional but recommended)
ETHERSCAN_API_KEY=your_etherscan_api_key_here

# SP1 Verifier address on Sepolia (provided by SP1)
# Check latest at: https://docs.succinct.xyz/
SP1_VERIFIER_ADDRESS=0x...  # Get from SP1 docs
```

### 3.3: Get Etherscan API Key (Optional - for Contract Verification)

1. Go to https://etherscan.io/
2. Sign up for free account
3. Go to API Keys → Create API Key
4. Copy the key to `.env`

This allows you to verify your contract source code on Etherscan.

---

## Step 4: Generate Proof Artifacts

Before deploying, you need to generate the proof and verification key.

```bash
cd ../rust_threshold_signing/host
export PATH="$HOME/.sp1/bin:$PATH"
cargo run --release
```

**This creates**:
- `../solidity_threshold_signing/proof.bin` - 7.4 MB proof
- `../solidity_threshold_signing/vk.bin` - 256 byte verification key

**Verify artifacts exist**:
```bash
ls -lh ../solidity_threshold_signing/*.bin
```

**Expected output**:
```
-rw-r--r--  1 user  staff   7.4M Oct  6 14:23 ../solidity_threshold_signing/proof.bin
-rw-r--r--  1 user  staff   256B Oct  6 14:23 ../solidity_threshold_signing/vk.bin
```

---

## Step 5: Deploy Contract to Sepolia

### 5.1: Check the SP1 Verifier Address

The SP1 team deploys a verifier contract on each network. Check the current address:

**Official SP1 Docs**: https://docs.succinct.xyz/onchain-verification/contract-addresses.html

**Sepolia Testnet**: Usually at `0x3B6041173B80E77f038f3F2C0f9744f04837185e` (verify on docs!)

Update your `.env` file with the correct address.

### 5.2: Deploy your contract

```bash
cd ../solidity_threshold_signing

# Load environment variables
source .env

# Deploy contract
forge create contracts/ThresholdVerifier.sol:ThresholdSignatureVerifier \
  --rpc-url $SEPOLIA_RPC_URL \
  --private-key $PRIVATE_KEY \
  --constructor-args $SP1_VERIFIER_ADDRESS $(cast abi-encode "f(bytes32)" $(xxd -p -c 256 vk.bin)) \
  --verify \
  --etherscan-api-key $ETHERSCAN_API_KEY
```

**If deployment succeeds**, you'll see:
```
Deployer: 0xYourAddress
Deployed to: 0xContractAddress
Transaction hash: 0x...
```

**Save the contract address!** You'll need it for verification.

### 5.3: Troubleshooting Deployment

**Error: "insufficient funds"**
- Get more Sepolia ETH from faucets (see Step 1)

**Error: "invalid API key"**
- Check your RPC URL is correct
- Verify API key in Alchemy/Infura dashboard

**Error: "Verification failed"**
- Contract still deployed successfully
- You can verify later with:
  ```bash
  forge verify-contract <CONTRACT_ADDRESS> \
    contracts/ThresholdVerifier.sol:ThresholdSignatureVerifier \
    --constructor-args $(cast abi-encode "constructor(address,bytes32)" $SP1_VERIFIER_ADDRESS $(xxd -p -c 256 vk.bin)) \
    --etherscan-api-key $ETHERSCAN_API_KEY \
    --chain sepolia
  ```

---

## Step 6: Verify Proof On-Chain

Now submit the proof to your deployed contract!

### 6.1: Prepare proof data

```bash
# Get proof as hex (without 0x prefix)
PROOF_HEX=$(xxd -p -c 1000000 proof.bin | tr -d '\n')

# Your deployed contract address
CONTRACT_ADDRESS=0xYourContractAddressFromStep5

# Encode the public values (from your proof generation output)
# You need: isValid (bool), publicKey (bytes32), message (bytes)
PUBLIC_VALUES=$(cast abi-encode "f(bool,bytes32,bytes)" true 0xYourPublicKeyHex 0xYourMessageHex)
```

### 6.2: Call the verifier

```bash
cast send $CONTRACT_ADDRESS \
  "verifyThresholdSignature(bytes,bytes)" \
  0x$PROOF_HEX \
  $PUBLIC_VALUES \
  --rpc-url $SEPOLIA_RPC_URL \
  --private-key $PRIVATE_KEY
```

**If successful**:
```
blockHash               0x...
blockNumber             12345678
...
status                  1 (success)
```

### 6.3: View the event

```bash
cast logs \
  --address $CONTRACT_ADDRESS \
  --rpc-url $SEPOLIA_RPC_URL
```

You should see the `SignatureVerified` event with your data!

---

## Step 7: View on Etherscan

1. Go to https://sepolia.etherscan.io/
2. Search for your contract address
3. You should see:
   - Contract deployment transaction
   - Verification transaction
   - Events emitted

---

## Alternative: Deploy Using Deployment Script

The project includes a Forge deployment script for easier deployment.

### 7.1: Review the script

```bash
cat script/Deploy.s.sol
```

### 7.2: Deploy using script

```bash
forge script script/Deploy.s.sol:DeployThresholdVerifier \
  --rpc-url $SEPOLIA_RPC_URL \
  --private-key $PRIVATE_KEY \
  --broadcast \
  --verify \
  --etherscan-api-key $ETHERSCAN_API_KEY
```

This will:
1. Read `vk.bin` automatically
2. Deploy the contract
3. Verify on Etherscan
4. Save deployment artifacts to `broadcast/` directory

---

## Gas Costs Estimation

**Deployment** (one-time):
- ~500,000 gas
- At 20 gwei: ~0.01 ETH
- At current Sepolia (usually 1-2 gwei): ~0.001-0.002 SepoliaETH

**Verification** (per proof):
- ~2,000,000 gas (depends on proof size)
- At 20 gwei: ~0.04 ETH
- At current Sepolia: ~0.002-0.004 SepoliaETH

**Note**: Mainnet costs would be significantly higher. Consider using L2s (Optimism, Arbitrum, Base) for production.

---

## Testing Locally First (Recommended)

Before deploying to testnet, test locally:

```bash
# Start local Ethereum node
anvil

# In another terminal, run tests
cd solidity_threshold_signing
forge test -vvv
```

This runs tests against a local blockchain with no cost.

---

## Production Deployment Checklist

Before deploying to mainnet:

- [ ] **Security audit** of Solidity contracts
- [ ] **Test on multiple testnets** (Sepolia, Goerli)
- [ ] **Verify gas costs** are acceptable
- [ ] **Consider L2 deployment** (Optimism, Arbitrum, Base, etc.)
- [ ] **Multi-sig wallet** for contract ownership
- [ ] **Monitor contract** with alerts
- [ ] **Upgrade path** if using proxy pattern
- [ ] **Emergency pause mechanism**

---

## Troubleshooting

### Contract deployment fails with "insufficient funds"
**Solution**: Get more testnet ETH from faucets (minimum ~0.01 SepoliaETH recommended)

### "Failed to get EIP-1559 fees"
**Solution**: Try adding `--legacy` flag to use legacy transactions:
```bash
forge create ... --legacy
```

### Proof verification fails on-chain
**Possible causes**:
1. Wrong verification key (regenerate proof.bin and vk.bin)
2. Wrong SP1 verifier address in constructor
3. Corrupted proof file
4. Proof generated for different program version

**Solution**: Regenerate proof with latest code and redeploy

### "Could not find artifact"
**Solution**: Build contracts first:
```bash
forge build
```

---

## Useful Commands

```bash
# Check wallet balance
cast balance <ADDRESS> --rpc-url $SEPOLIA_RPC_URL

# Get current gas price
cast gas-price --rpc-url $SEPOLIA_RPC_URL

# Estimate gas for function call
cast estimate <CONTRACT_ADDRESS> "verifyThresholdSignature(bytes,bytes)" <ARGS> --rpc-url $SEPOLIA_RPC_URL

# Read contract state
cast call <CONTRACT_ADDRESS> "programVKey()(bytes32)" --rpc-url $SEPOLIA_RPC_URL

# Get transaction receipt
cast receipt <TX_HASH> --rpc-url $SEPOLIA_RPC_URL
```

---

## Resources

- **Foundry Book**: https://book.getfoundry.sh/
- **SP1 Docs**: https://docs.succinct.xyz/
- **Sepolia Etherscan**: https://sepolia.etherscan.io/
- **Sepolia Faucets**:
  - https://sepoliafaucet.com/
  - https://faucet.quicknode.com/ethereum/sepolia
- **Gas Tracker**: https://etherscan.io/gastracker

---

## Next Steps

1. Test multiple proof submissions
2. Integrate with frontend dApp
3. Monitor gas costs and optimize
4. Plan mainnet or L2 deployment
5. Implement batch verification for multiple proofs
6. Add access control if needed
7. Set up monitoring and alerts

---

**Last Updated**: October 7, 2025
**Status**: Ready for testnet deployment
