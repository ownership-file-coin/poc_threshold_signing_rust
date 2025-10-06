// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {Script} from "forge-std/Script.sol";
import {console} from "forge-std/console.sol";
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
