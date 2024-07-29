// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "solady/utils/SignatureCheckerLib.sol";
import "solady/utils/ECDSA.sol";

import "forge-std/console2.sol";

enum ValidatorClassification {
    None,
    UnscopedValidator,
    ScopedValidator,
    Recovery,
    MultiPlexer
}

enum ExecutorClassification {
    None,
    HandlesUserAssets,
    NoDelegateCall,
    TriggerByAccount,
    TriggerByRelayer,
    DeterministicExecution
}

enum FallbackClassification {
    None,
    usesERC2771AccessControl,
    CalledWithStaticCall,
    CompatibiltyFallback,
    Callback
}

enum HookClassification {
    None,
    AllowDefault,
    DenyDefault,
    AccessControl,
    ModuleControl,
    UserControl
}

enum ExternalDependencyClassification {
    None,
    Oracles,
    Bridges,
    DEXs,
    Vaults,
    Registry,
    Lending,
    LiquidityProvision,
    Governance,
    Privacy,
    ZKProvers
}

enum ERC7579ModuleType {
    None,
    Validator,
    Executor,
    Fallback,
    Hook
}

struct ModuleTypeAttributes {
    ERC7579ModuleType moduleType;
    bytes encodedAttributes;
}

struct ModuleAttributes {
    address moduleAddress;
    bytes packedAttributes;
    ModuleTypeAttributes[] typeAttributes;
    bytes packedExternalDependency;
}
// uint16 ercDeps;

enum SignatureType {
    None,
    SECP256K1,
    ERC1271
}

struct Auditor {
    string name;
    string uri;
    string[] authors;
}

struct Signature {
    SignatureType sigType;
    address signer;
    bytes signatureData;
    bytes32 hash;
}

struct AuditSummary {
    string title;
    Auditor auditor;
    ModuleAttributes moduleAttributes;
    Signature signature;
}

struct Digest {
    string title;
    Auditor auditor;
    ModuleAttributes moduleAttributes;
}

contract Schema {
    using SignatureCheckerLib for address;

    function decode(bytes memory data) public pure returns (AuditSummary memory summary) {
        summary = abi.decode(data, (AuditSummary));
    }

    function digest(AuditSummary memory summary) public pure returns (bytes32) {
        bytes memory data = abi.encode(
            Digest({title: summary.title, auditor: summary.auditor, moduleAttributes: summary.moduleAttributes})
        );
        return keccak256(data);
    }

    function validateSignature(AuditSummary memory summary) public view returns (bool) {
        // if (summary.signature.signer == address(0)) {
        //     return false;
        // }
        //
        // if (summary.signature.sigType == SignatureType.None) {
        //     return false;
        // }
        // if (summary.signature.hash == bytes32(0)) {
        //     return false;
        // }
        //
        // if (summary.signature.signatureData.length == 0) {
        //     return false;
        // }

        if (summary.signature.sigType == SignatureType.SECP256K1) {
            // console2.log("SECP256K1");
            // console2.log(summary.signature.signer);
            // console2.logBytes(summary.signature.signatureData);
            // bytes32 _digest = digest(summary);
            // console2.log("hash same", _digest == summary.signature.hash);
            // bytes32 _r = bytes32(uint256(91181765093448877791072672153346532075250457633545348239646477672764849628088));
            // bytes32 _s = bytes32(uint256(7343553004683555850456342638978589027025187298616586585858472434401785059));
            // uint8 _v = uint8(0);
            //
            // address recover = ecrecover(_digest, _v, _r, _s);
            console2.log(summary.signature.signer);

            console2.logBytes32(summary.signature.hash);
            console2.logBytes(summary.signature.signatureData);
            address recover = ECDSA.recover(ECDSA.toEthSignedMessageHash(summary.signature.hash), summary.signature.signatureData);
            console2.log(recover);
            return recover == summary.signature.signer;
        }
    }
}
