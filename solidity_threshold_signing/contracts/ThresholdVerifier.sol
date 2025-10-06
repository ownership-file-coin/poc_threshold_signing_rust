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
