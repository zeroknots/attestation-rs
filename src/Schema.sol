// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

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
}

struct AuditSummary {
    string title;
    Auditor auditor;
    ModuleAttributes moduleAttributes;
    Signature signature;
}

contract Schema {
    function decode(bytes memory data) public pure returns (AuditSummary memory summary) {
        summary = abi.decode(data, (AuditSummary));
    }
}
