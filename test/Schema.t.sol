// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Schema, AuditSummary} from "../src/Schema.sol";

contract SchemaTest is Test {
    Schema public schema;

    Account signer;

    function setUp() public {
        schema = new Schema();

        signer = makeAccount("signer");
    }

    function test_decode() public {
        string[] memory inputs = new string[](7);
        inputs[0] = "cargo";
        inputs[1] = "run";
        inputs[2] = "--";
        inputs[3] = "--input";
        inputs[4] = "./attestation/example2.json";
        inputs[5] = "--private-key";
        inputs[6] = "0x7f28531d8798eb4b4488bc51cf7cec1941c20fc7ce1a3f754e67f89759e6401d";

        bytes memory data = vm.ffi(inputs);

        AuditSummary memory summary = schema.decode(data);

        bool reentrancyProtection = (summary.moduleAttributes.packedAttributes[0] == 0x01);
        bool ownerCantRug = (summary.moduleAttributes.packedAttributes[4] == 0x01);
        assertTrue(reentrancyProtection, "reentrancyProtection");
        assertFalse(ownerCantRug, "ownerCantRug");
    }
}
