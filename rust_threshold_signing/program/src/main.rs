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
