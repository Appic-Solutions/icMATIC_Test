// SPDX-License-Identifier: Apache-2.0
pragma solidity 0.8.18;

contract IcMaticDepositHelper {
    address payable private immutable minter_address;

    event recievedMatic(
        address indexed from,
        uint256 value,
        bytes32 indexed principal
    );

    constructor(address icMAtic_minter_address) {
        minter_address = payable(icMAtic_minter_address);
    }

    function getMinterAddress() public view returns (address) {
        return minter_address;
    }

    function deposit(bytes32 _principal) public payable {
        minter_address.transfer(msg.value);
        emit recievedMatic(msg.sender, msg.value, _principal);
    }
}
