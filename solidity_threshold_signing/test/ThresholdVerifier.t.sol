// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Test} from "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
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
        assertEq(verifier.programVKey(), bytes32(uint256(1)));
    }

    function test_VerifyThresholdSignatureView() public {
        // Create test data
        bool expectedIsValid = true;
        bytes32 expectedPublicKey = bytes32(uint256(0x123));
        bytes memory expectedMessage = "test message";

        // Encode the data
        bytes memory publicValues = abi.encode(
            expectedIsValid,
            expectedPublicKey,
            expectedMessage
        );

        // Call the view function
        (bool isValid, bytes32 publicKey, bytes memory message) =
            verifier.verifyThresholdSignatureView(publicValues);

        // Verify the decoded values
        assertEq(isValid, expectedIsValid);
        assertEq(publicKey, expectedPublicKey);
        assertEq(message, expectedMessage);
    }
}
