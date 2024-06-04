// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.18;

import {Script} from "forge-std/Script.sol";
import {IcMaticDepositHelper} from "../src/maticHelper.sol";

contract DeployIcMATIC is Script {
    function run() public returns (IcMaticDepositHelper) {
        vm.startBroadcast(vm.envUint("PRIVATE_KEY"));
        IcMaticDepositHelper icMaticContract = new IcMaticDepositHelper(
            0xbAf59B045c6B53bCc849e2a487C14F234435cC51
        );
        vm.stopBroadcast();
        return icMaticContract;
    }
}
