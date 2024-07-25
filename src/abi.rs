use alloy_primitives::Bytes;
use alloy_sol_types::sol;

use crate::types::{
    JsonExecutorAttributes, JsonFallbackAttributes, JsonGlobalAttributes, JsonHookAttributes,
    JsonModuleAttributes, JsonValidatorAttributes,
};

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
        let list = vec![
            self.reentrancy_protection as u8,
            self.important_data_validation as u8,
            self.input_manipulation_protection as u8,
            self.emits_events as u8,
            self.module_owner_cant_rug as u8,
            self.upgradeable as u8,
            self.pausable as u8,
            self.licensed_module as u8,
            self.erc7562_storage_compliant as u8,
            self.uninstall_clean_up as u8,
        ];
        Bytes::from(list)
    }
}

impl PackableAttributes for JsonValidatorAttributes {
    fn pack(&self) -> Bytes {
        let list = vec![
            self.unscoped_validator as u8,
            self.recovery_module as u8,
            self.multiplexer as u8,
        ];

        Bytes::from(list)
    }
}

impl PackableAttributes for JsonExecutorAttributes {
    fn pack(&self) -> Bytes {
        let list = vec![
            self.triggered_by_account as u8,
            self.triggered_by_relayer as u8,
            self.deterministic_execution as u8,
        ];

        Bytes::from(list)
    }
}

impl PackableAttributes for JsonFallbackAttributes {
    fn pack(&self) -> Bytes {
        let list = vec![
            self.compatibility_feature as u8,
            self.callbacks as u8];

        Bytes::from(list)
    }
}

impl PackableAttributes for JsonHookAttributes {
    fn pack(&self) -> Bytes {
        let list = vec![
            self.default_allow as u8,
            self.default_deny as u8,
            self.access_control as u8,
            self.module_control as u8,
            self.user_control as u8,
        ];

        Bytes::from(list)
    }
}

pub trait ParseAttributes {
    fn parse(&self) -> ModuleAttributes;
}
impl ParseAttributes for JsonModuleAttributes {
    fn parse(&self) -> ModuleAttributes {
        let module_attributes = ModuleAttributes {
            moduleAddress: self.module_address,
            packedAttributes: self.global_attributes.pack(),
            typeAttributes: vec![
                ModuleTypeAttributes {
                    moduleType: ERC7579ModuleType::Validator,
                    encodedAttributes: self.validator_attributes.pack(),
                },

                ModuleTypeAttributes {
                    moduleType: ERC7579ModuleType::Executor,
                    encodedAttributes: self.executor_attributes.pack(),
                },
                ModuleTypeAttributes {
                    moduleType: ERC7579ModuleType::Fallback,
                    encodedAttributes: self.fallback_attributes.pack(),
                },
                ModuleTypeAttributes {
                    moduleType: ERC7579ModuleType::Hook,
                    encodedAttributes: self.hook_attributes.pack(),
                },
            ],
        };
        module_attributes
    }
}
