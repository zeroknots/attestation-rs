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

//string foobar = "(enum ERC7579ModuleType (None,Validator,Executor,Fallback,Hook),struct ModuleTypeAttributes (ERC7579ModuleType moduleType,bytes encodedAttributes),struct ModuleAttributes (address moduleAddress,bytes packedAttributes,ModuleTypeAttributes[] typeAttributes,bytes packedExternalDependency),enum SignatureType (None,SECP256K1,ERC1271),struct Auditor (string name,string uri,string[] authors),struct Signature (SignatureType sigType,address signer,bytes signatureData,bytes32 hash),struct AuditSummary (string title,Auditor auditor,ModuleAttributes moduleAttributes,Signature signature))");

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
        if (summary.signature.sigType == SignatureType.SECP256K1) {
            address recover =
                ECDSA.recover(ECDSA.toEthSignedMessageHash(summary.signature.hash), summary.signature.signatureData);
            return recover == summary.signature.signer;
        }
    }
}
