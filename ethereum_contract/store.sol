// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract IPFS_CID_Store {
    string cid;

    function set(string calldata value) public {
        cid = value;
    }

    // Reads the last stored value
    function get() public view returns (string memory) {
        return cid;
    }
}