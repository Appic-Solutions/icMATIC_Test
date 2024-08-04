// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.20;

import "forge-std/Script.sol";
import "../src/TokenLock.sol";

contract DeployTokenLock is Script {
    function run() external {
        // Start broadcasting transactions
        vm.startBroadcast();

        // Deploy the TokenLock contract
        TokenLock tokenLock = new TokenLock();

        // Log the deployed contract address
        console.log("TokenLock deployed at:", address(tokenLock));

        // Stop broadcasting transactions
        vm.stopBroadcast();
    }
}
