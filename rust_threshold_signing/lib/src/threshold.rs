use frost_ed25519 as frost;
use std::collections::BTreeMap;
use rand::thread_rng;
use sha2::Digest;

use crate::serialization::{SignerMessage, SignerResponse, CombinedSignature, serialize, deserialize};

// Store FROST signing packages for each signer during the signing process
pub struct ThresholdSigner {
    pub index: u16,
    pub key_package: frost::keys::KeyPackage,
    pub signing_nonces: Option<frost::round1::SigningNonces>,
    pub signing_commitments: Option<frost::round1::SigningCommitments>,
}

impl ThresholdSigner {
    pub fn new(index: u16, key_package: frost::keys::KeyPackage) -> Self {
        Self {
            index,
            key_package,
            signing_nonces: None,
            signing_commitments: None,
        }
    }

    /// Round 1: Generate nonce commitments for signing
    pub fn round1_generate_nonces(&mut self) -> frost::round1::SigningCommitments {
        let mut rng = thread_rng();
        let (nonces, commitments) = frost::round1::commit(
            self.key_package.signing_share(),
            &mut rng,
        );

        self.signing_nonces = Some(nonces);
        self.signing_commitments = Some(commitments.clone());

        commitments
    }

    /// Round 2: Generate signature share
    pub fn round2_sign(
        &self,
        _message: &[u8],
        signing_package: &frost::SigningPackage,
    ) -> Result<frost::round2::SignatureShare, String> {
        let nonces = self.signing_nonces.as_ref()
            .ok_or("No signing nonces available")?;

        frost::round2::sign(signing_package, nonces, &self.key_package)
            .map_err(|e| format!("Signing failed: {:?}", e))
    }

    /// Receive a serialized signing request and return a serialized response
    pub fn receive_serialized_signing_request(&mut self, serialized_msg: &[u8]) -> Vec<u8> {
        let msg: SignerMessage = deserialize(serialized_msg);

        // Generate nonce commitments
        let _commitments = self.round1_generate_nonces();

        // For demo purposes, we'll serialize the commitments as signature share
        // In a real implementation, this would be handled by the coordinator
        let response = SignerResponse {
            signer_index: self.index as u8,
            signature_share: msg.message_hash, // Placeholder
            nonce_share: msg.nonce_commitment,  // Placeholder
        };

        serialize(&response)
    }
}

pub struct ThresholdCoordinator {
    pub threshold: u16,
    pub signers: Vec<ThresholdSigner>,
    pub pubkey_package: frost::keys::PublicKeyPackage,
}

impl ThresholdCoordinator {
    pub fn new(
        threshold: u16,
        signers: Vec<ThresholdSigner>,
        pubkey_package: frost::keys::PublicKeyPackage,
    ) -> Self {
        Self {
            threshold,
            signers,
            pubkey_package,
        }
    }

    /// Send signing request to a specific signer
    pub fn send_to_signer(&mut self, signer_index: usize, message: &[u8]) -> Vec<u8> {
        let msg_hash = sha2::Sha256::digest(message);
        let mut msg_hash_bytes = [0u8; 32];
        msg_hash_bytes.copy_from_slice(&msg_hash);

        let signer_msg = SignerMessage {
            signer_index: signer_index as u8,
            message_hash: msg_hash_bytes,
            nonce_commitment: [0u8; 32], // Placeholder
        };

        let serialized_request = serialize(&signer_msg);

        // Simulate network call - in reality this would go over HTTP/gRPC
        self.signers[signer_index].receive_serialized_signing_request(&serialized_request)
    }

    /// Perform complete threshold signing process
    pub fn perform_threshold_signing(
        &mut self,
        message: &[u8],
        signer_indices: Vec<u16>,
    ) -> Result<CombinedSignature, String> {
        if signer_indices.len() < self.threshold as usize {
            return Err(format!(
                "Not enough signers: {} < {}",
                signer_indices.len(),
                self.threshold
            ));
        }

        // Round 1: Collect nonce commitments from all signers
        let mut commitments = BTreeMap::new();
        for &idx in &signer_indices {
            // idx is the signer's identifier (1-based), convert to 0-based for Vec indexing
            let signer = &mut self.signers[(idx - 1) as usize];
            let commitment = signer.round1_generate_nonces();
            let identifier = frost::Identifier::try_from(idx)
                .map_err(|e| format!("Invalid identifier: {:?}", e))?;
            commitments.insert(identifier, commitment);
        }

        // Create signing package
        let signing_package = frost::SigningPackage::new(commitments, message);

        // Round 2: Collect signature shares
        let mut signature_shares = BTreeMap::new();
        for &idx in &signer_indices {
            let identifier = frost::Identifier::try_from(idx)
                .map_err(|e| format!("Invalid identifier: {:?}", e))?;
            // idx is the signer's identifier (1-based), convert to 0-based for Vec indexing
            let signer = &self.signers[(idx - 1) as usize];
            let share = signer.round2_sign(message, &signing_package)?;
            signature_shares.insert(identifier, share);
        }

        // Aggregate signature shares into final signature
        let group_signature = frost::aggregate(&signing_package, &signature_shares, &self.pubkey_package)
            .map_err(|e| format!("Aggregation failed: {:?}", e))?;

        // Convert to ed25519-dalek format
        let sig_vec = group_signature.serialize()
            .map_err(|e| format!("Failed to serialize signature: {:?}", e))?;
        let signature_bytes: [u8; 64] = sig_vec
            .as_slice()
            .try_into()
            .expect("FROST signature should be 64 bytes");
        let vk_vec = self.pubkey_package.verifying_key().serialize()
            .map_err(|e| format!("Failed to serialize verifying key: {:?}", e))?;
        let verifying_key_bytes: [u8; 32] = vk_vec
            .as_slice()
            .try_into()
            .expect("Verifying key should be 32 bytes");

        Ok(CombinedSignature {
            signature: signature_bytes,
            public_key: verifying_key_bytes,
        })
    }

    /// Combine signature shares (simplified version for demonstration)
    pub fn combine_signatures(&self, _serialized_shares: Vec<Vec<u8>>) -> CombinedSignature {
        // This is a placeholder - real implementation uses perform_threshold_signing
        let vk_vec = self.pubkey_package.verifying_key().serialize()
            .expect("Failed to serialize verifying key");
        let verifying_key_bytes: [u8; 32] = vk_vec
            .as_slice()
            .try_into()
            .expect("Verifying key should be 32 bytes");
        CombinedSignature {
            signature: [0u8; 64],
            public_key: verifying_key_bytes,
        }
    }
}

/// Generate FROST key packages for threshold signing
///
/// Note: This uses the "trusted dealer" method for simplicity in this PoC.
/// For production, implement the full DKG protocol which doesn't require a trusted party.
/// The trusted dealer method still produces valid FROST threshold signatures.
pub fn generate_frost_keys(
    max_signers: u16,
    min_signers: u16,
) -> Result<(Vec<frost::keys::KeyPackage>, frost::keys::PublicKeyPackage), String> {
    let mut rng = thread_rng();

    // Use trusted dealer for key generation (simpler but requires trust)
    // For production DKG, use frost::keys::dkg module
    let (shares, pubkey_package) = frost::keys::generate_with_dealer(
        max_signers,
        min_signers,
        frost::keys::IdentifierList::Default,
        &mut rng,
    ).map_err(|e| format!("Trusted dealer keygen failed: {:?}", e))?;

    // Convert secret shares to key packages
    let key_packages: Vec<_> = shares
        .into_iter()
        .map(|(_, secret_share)| {
            frost::keys::KeyPackage::try_from(secret_share)
                .expect("Failed to create KeyPackage from SecretShare")
        })
        .collect();

    Ok((key_packages, pubkey_package))
}

/// Full DKG implementation (currently not working - keeping for future fix)
#[allow(dead_code)]
fn generate_frost_keys_dkg(
    max_signers: u16,
    min_signers: u16,
) -> Result<(Vec<frost::keys::KeyPackage>, frost::keys::PublicKeyPackage), String> {
    use frost::keys::dkg::{part1, part2, part3};

    let mut rng = thread_rng();
    let max_signers_usize = max_signers as usize;

    // Part 1: Each participant generates their secret polynomial
    let mut part1_packages = Vec::new();
    let mut part1_secret_packages = Vec::new();

    for i in 1..=max_signers {
        let identifier = frost::Identifier::try_from(i)
            .map_err(|e| format!("Invalid identifier: {:?}", e))?;

        let (secret_package, package) = part1(
            identifier,
            max_signers,
            min_signers,
            &mut rng,
        ).map_err(|e| format!("Part 1 failed: {:?}", e))?;

        part1_secret_packages.push(secret_package);
        part1_packages.push(package);
    }

    // Part 2: Each participant processes packages from others
    let mut part2_packages = Vec::new();
    let mut part2_secret_packages = Vec::new();

    for i in 0..max_signers_usize {
        let mut received_packages = BTreeMap::new();
        for (j, package) in part1_packages.iter().enumerate() {
            if i != j {
                let sender_id = frost::Identifier::try_from((j + 1) as u16)
                    .map_err(|e| format!("Invalid identifier: {:?}", e))?;
                received_packages.insert(sender_id, package.clone());
            }
        }

        let (secret_package, packages) = part2(
            part1_secret_packages[i].clone(),
            &received_packages,
        ).map_err(|e| format!("Part 2 failed for participant {}: {:?}", i + 1, e))?;

        // Debug: Verify part2 generated the right number of packages
        let expected_packages = max_signers_usize - 1; // Should create packages for all OTHER participants
        if packages.len() != expected_packages {
            return Err(format!(
                "Part 2 participant {} generated {} packages, expected {}",
                i + 1,
                packages.len(),
                expected_packages
            ));
        }

        part2_secret_packages.push(secret_package);
        part2_packages.push(packages);
    }

    // Part 3: Each participant creates their key package
    let mut key_packages = Vec::new();
    let mut pubkey_packages = Vec::new();

    // Convert part1_packages to BTreeMap for part3
    let part1_packages_map: BTreeMap<_, _> = part1_packages
        .iter()
        .enumerate()
        .map(|(j, pkg)| {
            let id = frost::Identifier::try_from((j + 1) as u16).unwrap();
            (id, pkg.clone())
        })
        .collect();

    for i in 0..max_signers_usize {
        let my_id = frost::Identifier::try_from((i + 1) as u16)
            .map_err(|e| format!("Invalid identifier: {:?}", e))?;

        // Collect Round 2 packages destined for this participant
        let mut received_packages = BTreeMap::new();
        for (j, packages) in part2_packages.iter().enumerate() {
            let sender_id = frost::Identifier::try_from((j + 1) as u16)
                .map_err(|e| format!("Invalid identifier: {:?}", e))?;

            // Don't include our own package
            if i != j {
                if let Some(package) = packages.get(&my_id) {
                    received_packages.insert(sender_id, package.clone());
                }
            }
        }

        // Debug: check we got the right number of packages
        let expected_r2_count = max_signers_usize - 1;
        if received_packages.len() != expected_r2_count {
            return Err(format!(
                "Participant {} expected {} round2 packages but got {}",
                i + 1,
                expected_r2_count,
                received_packages.len()
            ));
        }

        // part1_packages_map should have ALL participants (including self)
        if part1_packages_map.len() != max_signers_usize {
            return Err(format!(
                "Participant {} expected {} round1 packages but got {}",
                i + 1,
                max_signers_usize,
                part1_packages_map.len()
            ));
        }

        let (key_package, pubkey_package) = part3(
            &part2_secret_packages[i],
            &part1_packages_map,
            &received_packages,
        ).map_err(|e| format!("Part 3 failed for participant {}: {:?}. Round1 packages: {}, Round2 packages: {}",
            i + 1, e, part1_packages_map.len(), received_packages.len()))?;

        key_packages.push(key_package);
        pubkey_packages.push(pubkey_package);
    }

    // All participants should have the same public key package
    let pubkey_package = pubkey_packages[0].clone();

    Ok((key_packages, pubkey_package))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frost_key_generation() {
        let result = generate_frost_keys(5, 3);
        if let Err(e) = &result {
            eprintln!("Key generation error: {}", e);
        }
        assert!(result.is_ok());

        let (key_packages, pubkey_package) = result.unwrap();
        assert_eq!(key_packages.len(), 5);

        // All key packages should have the same group public key
        for kp in &key_packages {
            let kp_vec = kp.verifying_key().serialize().unwrap();
            let kp_bytes: [u8; 32] = kp_vec.as_slice().try_into().unwrap();
            let pkg_vec = pubkey_package.verifying_key().serialize().unwrap();
            let pkg_bytes: [u8; 32] = pkg_vec.as_slice().try_into().unwrap();
            assert_eq!(kp_bytes, pkg_bytes);
        }
    }

    #[test]
    fn test_threshold_signing() {
        let (key_packages, pubkey_package) = generate_frost_keys(5, 3).unwrap();

        let signers: Vec<ThresholdSigner> = key_packages
            .into_iter()
            .enumerate()
            .map(|(i, kp)| ThresholdSigner::new((i + 1) as u16, kp))
            .collect();

        let mut coordinator = ThresholdCoordinator::new(3, signers, pubkey_package);

        let message = b"Hello, threshold signatures!";
        let signer_indices = vec![1, 2, 3];

        let result = coordinator.perform_threshold_signing(message, signer_indices);
        if let Err(e) = &result {
            eprintln!("Threshold signing error: {}", e);
        }
        assert!(result.is_ok());

        let combined_sig = result.unwrap();

        // Verify the signature using ed25519-dalek
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};

        let verifying_key = VerifyingKey::from_bytes(&combined_sig.public_key).unwrap();
        let signature = Signature::from_bytes(&combined_sig.signature);

        assert!(verifying_key.verify(message, &signature).is_ok());
    }
}
