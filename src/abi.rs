use alloy_primitives::{keccak256, Address, Bytes, U256};
use alloy_sol_types::{sol, EventTopic};
use alloy_sol_types::{sol_data::*, SolType};
use core::hash::Hash;
use std::str::FromStr;

use crate::types::{JsonGlobalAttributes};

sol! {


#[derive(Debug)]
enum ValidatorClassification {
    None,
    UnscopedValidator,
    ScopedValidator,
    Recovery,
    MultiPlexer
}

#[derive(Debug)]
enum ExecutorClassification {
    None,
    TriggerByAccount,
    TriggerByRelayer,
    DeterministicExecution
}

#[derive(Debug)]
enum FallbackClassification {
    None,
    CompatibiltyFallback,
    Callback
}

#[derive(Debug)]
enum HookClassification {
    None,
    AllowDefault,
    DenyDefault,
    AccessControl,
    ModuleControl,
    UserControl
}

#[derive(Debug)]
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

#[derive(Debug)]
struct ValidatorAttributes {
    ValidatorClassification[] classifications;
}

#[derive(Debug)]
struct ExecutorAttributes {
    bool handlesUserAssets;
    bool noDelegateCall;
    ExecutorClassification[] classifications;
}

#[derive(Debug)]
struct FallbackAttributes {
    bool usesERC2771AccessControl;
    bool calledWithStaticCall;
    FallbackClassification[] classifications;
}

#[derive(Debug)]
struct HookAttributes {
    HookClassification[] classifications;
}

#[derive(Debug)]
enum ERC7579ModuleType {
    None,
    Validator,
    Executor,
    Fallback,
    Hook
}

#[derive(Debug)]
struct ModuleTypeAttributes {
    ERC7579ModuleType moduleType;
    bytes encodedAttributes;
}

#[derive(Debug)]
struct ModuleAttributes {
    address moduleAddress;
    // packed(moduleOwnerCantRug, isUpgradeable, isPausable, isLicensedModule,
    // erc7562StorageCompliant, uninstallCleanUp)
    bytes packedAttributes;
    ModuleTypeAttributes[] typeAttributes;
}

#[derive(Debug)]
enum SignatureType {
    None,
    SECP256K1,
    ERC1271
}

#[derive(Debug)]
struct Auditor {
    string name;
    string uri;
    string[] authors;
}

#[derive(Debug)]
struct Signature {
    SignatureType sigType;
    address signer;
    bytes signatureData;
}

#[derive(Debug)]
struct AuditSummary {
    string title;
    Auditor auditor;
    ModuleAttributes moduleAttributes;
    Signature signature;
}

}

pub trait PackableAttributes {
    fn pack(&self) -> Bytes;
}


impl PackableAttributes for JsonGlobalAttributes {
    fn pack(&self) -> Bytes {
        let packed = Bytes::new();


        packed
    }
}

