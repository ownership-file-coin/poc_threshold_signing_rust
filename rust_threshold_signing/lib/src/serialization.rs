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
    #[serde(with = "serde_big_array::BigArray")]
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
