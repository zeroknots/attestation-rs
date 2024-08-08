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
        string[] memory inputs = new string[](10);
        inputs[0] = "cargo";
        inputs[1] = "run";
        inputs[2] = "--";
        inputs[3] = "--input";
        inputs[4] = "./attestation/example.json";
        inputs[5] = "--mode";
        inputs[6] = "cast";
        inputs[7] = "private-key";
        inputs[8] = "--private-key";
        inputs[9] = "0x5ed8e65ce779308a499dfcb4b2d37a267847e084d80d1b51b09f0bf97e5b8319";
        vm.ffi(inputs);

        string[] memory print = new string[](7);
        print[0] = "cargo";
        print[1] = "run";
        print[2] = "--";
        print[3] = "--input";
        print[4] = "./attestation/example.signed.json";
        print[5] = "--mode";
        print[6] = "bytes";

        bytes memory data = vm.ffi(print);

        AuditSummary memory summary = schema.decode(data);

        bool reentrancyProtection = (summary.moduleAttributes.packedAttributes[0] == 0x01);
        bool ownerCantRug = (summary.moduleAttributes.packedAttributes[4] == 0x01);
        assertTrue(reentrancyProtection, "reentrancyProtection");
        assertFalse(ownerCantRug, "ownerCantRug");

        bytes32 hash_from_rust = summary.signature.hash;


        // // bytes32 hash = schema.digest(summary);
        // // assertEq(hash, hash_from_rust, "hash");
        //
        // summary.signature.signer = 0xD1dcdD8e6Fe04c338aC3f76f7D7105bEcab74F77;
        // summary.signature.hash = 0xcd27fc48f1f4d6e605817b0c9b5a8a52f2d1ae87b31c65edb64846a047cdad2a;
        // summary.signature.signatureData =
        //     hex"d9a389b4a84991f18fa48ae3190838df031236215967489d5baeff27838d085768a0ab4bc56d0158f575ca4ba359b7b0eac2bf6dfc7efeff740981af20f85ba11b";
        console.logBytes(summary.signature.signatureData);

        bool validSig = schema.validateSignature(summary);

        assertTrue(validSig, "validSig");
    }
}
