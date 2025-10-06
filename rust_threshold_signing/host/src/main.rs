use sp1_sdk::{ProverClient, SP1Stdin};
use threshold_signing_lib::{ThresholdSigner, ThresholdCoordinator, generate_frost_keys, serialize};

fn main() {
    println!("=== Threshold Signature SP1 zkVM Demo ===\n");

    // Step 1: Setup threshold signing configuration
    let threshold = 3;
    let total_signers = 5;
    let message = b"Hello, threshold signatures in zkVM!";

    println!("Configuration:");
    println!("  Threshold: {}/{}", threshold, total_signers);
    println!("  Message: {:?}\n", String::from_utf8_lossy(message));

    // Step 2: Generate FROST keys using distributed key generation
    println!("Generating FROST threshold keys...");
    let (key_packages, pubkey_package) = generate_frost_keys(total_signers, threshold)
        .expect("Failed to generate FROST keys");
    println!("Keys generated successfully\n");

    // Create signers from key packages
    let signers: Vec<ThresholdSigner> = key_packages
        .into_iter()
        .enumerate()
        .map(|(i, kp)| ThresholdSigner::new((i + 1) as u16, kp))
        .collect();

    let mut coordinator = ThresholdCoordinator::new(threshold, signers, pubkey_package);

    // Step 3: Perform threshold signing
    println!("Performing threshold signing...");
    let signer_indices = vec![1, 2, 3]; // Use first 3 signers (meets threshold)

    println!("  Using signers: {:?}", signer_indices);

    let combined_signature = coordinator
        .perform_threshold_signing(message, signer_indices)
        .expect("Threshold signing failed");

    println!("Threshold signature created\n");

    // Verify signature locally before generating proof
    println!("Verifying signature locally...");
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let verifying_key = VerifyingKey::from_bytes(&combined_signature.public_key)
        .expect("Invalid public key");
    let signature = Signature::from_bytes(&combined_signature.signature);

    verifying_key
        .verify(message, &signature)
        .expect("Local signature verification failed");
    println!("Local verification successful\n");

    // Step 4: Serialize for zkVM
    let combined_sig_serialized = serialize(&combined_signature);

    // Step 5: Generate zkVM proof
    println!("Generating SP1 proof...");
    let client = ProverClient::new();
    let elf = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

    let mut stdin = SP1Stdin::new();
    stdin.write(&message.to_vec());
    stdin.write(&combined_sig_serialized);

    let (pk, vk) = client.setup(elf);

    println!("Proving (this may take a few minutes)...");
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
    println!("Public key: {}", hex::encode(public_key));
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
    println!("\n=== Demo Complete ===");
}
