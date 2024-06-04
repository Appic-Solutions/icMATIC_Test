// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import {Script, console2} from "forge-std/Script.sol";

import {IcMaticDepositHelper} from "../src/maticHelper.sol";

contract InteractWithContract is Script {
    function run() public {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
        IcMaticDepositHelper contractInterface = IcMaticDepositHelper(
            0x0E2e8F489927b62725Ae65EcB2C3ED410701A337
        );
        contractInterface.deposit{value: 1e9}(0x1d4cc29325adc1e82de0dc2e87dce6e8feb31c0b4ae578881172cf35f9020000); 
        vm.stopBroadcast();
    }
}


