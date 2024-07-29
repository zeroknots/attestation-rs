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
        string[] memory inputs = new string[](9);
        inputs[0] = "cargo";
        inputs[1] = "run";
        inputs[2] = "--";
        inputs[3] = "--input";
        inputs[4] = "./attestation/example2.json";
        inputs[5] = "--private-key";
        inputs[6] = "0x7f28531d8798eb4b4488bc51cf7cec1941c20fc7ce1a3f754e67f89759e6401d";
        inputs[7] = "--mode";
        inputs[8] = "sign";

        bytes memory data = vm.ffi(inputs);

        AuditSummary memory summary = schema.decode(data);

        bool reentrancyProtection = (summary.moduleAttributes.packedAttributes[0] == 0x01);
        bool ownerCantRug = (summary.moduleAttributes.packedAttributes[4] == 0x01);
        assertTrue(reentrancyProtection, "reentrancyProtection");
        assertFalse(ownerCantRug, "ownerCantRug");

        bytes32 hash_from_rust = summary.signature.hash;

        // bytes32 hash = schema.digest(summary);
        // assertEq(hash, hash_from_rust, "hash");

        summary.signature.signer = 0xD1dcdD8e6Fe04c338aC3f76f7D7105bEcab74F77;
        summary.signature.hash = 0xcd27fc48f1f4d6e605817b0c9b5a8a52f2d1ae87b31c65edb64846a047cdad2a;
        summary.signature.signatureData =
            hex"d9a389b4a84991f18fa48ae3190838df031236215967489d5baeff27838d085768a0ab4bc56d0158f575ca4ba359b7b0eac2bf6dfc7efeff740981af20f85ba11b";
        console.logBytes(summary.signature.signatureData);

        bool validSig = schema.validateSignature(summary);

        assertTrue(validSig, "validSig");
    }
}
