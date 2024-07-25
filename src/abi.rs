use alloy_primitives::Bytes;
use alloy_sol_types::sol;

use crate::types::{
    JsonExecutorAttributes, JsonFallbackAttributes, JsonGlobalAttributes, JsonHookAttributes,
    JsonModuleAttributes, JsonValidatorAttributes, JsonExternalDependency
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
    HandlesUserAssets,
    NoDelegateCall,
    TriggerByAccount,
    TriggerByRelayer,
    DeterministicExecution
}

#[derive(Debug)]
enum FallbackClassification {
    None,
    usesERC2771AccessControl,
    CalledWithStaticCall,
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

#[derive(Debug, PartialEq)]
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


#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, Bytes};

    #[test]
    fn test_pack_global_attributes() {
        let global_attrs = JsonGlobalAttributes {
            reentrancy_protection: true,
            important_data_validation: false,
            input_manipulation_protection: true,
            emits_events: true,
            module_owner_cant_rug: false,
            upgradeable: true,
            pausable: false,
            licensed_module: true,
            erc7562_storage_compliant: false,
            uninstall_clean_up: true,
        };

        let packed = global_attrs.pack();
        assert_eq!(packed, Bytes::from(vec![1, 0, 1, 1, 0, 1, 0, 1, 0, 1]));
    }

    #[test]
    fn test_pack_validator_attributes() {
        let validator_attrs = JsonValidatorAttributes {
            unscoped_validator: true,
            recovery_module: false,
            multiplexer: true,
        };

        let packed = validator_attrs.pack();
        assert_eq!(packed, Bytes::from(vec![1, 0, 1]));
    }

    #[test]
    fn test_pack_executor_attributes() {
        let executor_attrs = JsonExecutorAttributes {
            handles_user_assets: true,
            no_delegate_call: false,
            triggered_by_account: true,
            triggered_by_relayer: false,
            deterministic_execution: true,
        };

        let packed = executor_attrs.pack();
        assert_eq!(packed, Bytes::from(vec![1, 0, 1]));
    }

    #[test]
    fn test_pack_fallback_attributes() {
        let fallback_attrs = JsonFallbackAttributes {
            compatibility_feature: true,
            callbacks: false,
        };

        let packed = fallback_attrs.pack();
        assert_eq!(packed, Bytes::from(vec![1, 0]));
    }

    #[test]
    fn test_pack_hook_attributes() {
        let hook_attrs = JsonHookAttributes {
            default_allow: true,
            default_deny: false,
            access_control: true,
            module_control: false,
            user_control: true,
        };

        let packed = hook_attrs.pack();
        assert_eq!(packed, Bytes::from(vec![1, 0, 1, 0, 1]));
    }

    #[test]
    fn test_parse_module_attributes() {
        let module_attrs = JsonModuleAttributes {
            module_address: Address::from([0x42; 20]),
            global_attributes: JsonGlobalAttributes {
                reentrancy_protection: true,
                important_data_validation: false,
                input_manipulation_protection: true,
                emits_events: true,
                module_owner_cant_rug: false,
                upgradeable: true,
                pausable: false,
                licensed_module: true,
                erc7562_storage_compliant: false,
                uninstall_clean_up: true,
            },
            validator_attributes: JsonValidatorAttributes {
                unscoped_validator: true,
                recovery_module: false,
                multiplexer: true,
            },
            executor_attributes: JsonExecutorAttributes {
                handles_user_assets: true,
                no_delegate_call: false,
                triggered_by_account: true,
                triggered_by_relayer: false,
                deterministic_execution: true,
            },
            fallback_attributes: JsonFallbackAttributes {
                compatibility_feature: true,
                callbacks: false,
            },
            hook_attributes: JsonHookAttributes {
                default_allow: true,
                default_deny: false,
                access_control: true,
                module_control: false,
                user_control: true,
            },
            external_dependency: JsonExternalDependency {
                oracle: false,
                bridges: false,
                dexs: false,
                vaults: false,
                registry: false,
                lending: false,
                liquidity_provision: false,
                governance: false,
                privacy: false,
                zk_provers: false,
                erc_deps: vec![],
            },
        };

        let parsed = module_attrs.parse();

        assert_eq!(parsed.moduleAddress, Address::from([0x42; 20]));
        assert_eq!(parsed.packedAttributes, Bytes::from(vec![1, 0, 1, 1, 0, 1, 0, 1, 0, 1]));
        assert_eq!(parsed.typeAttributes.len(), 4);

        assert_eq!(parsed.typeAttributes[0].moduleType, ERC7579ModuleType::Validator);
        assert_eq!(parsed.typeAttributes[0].encodedAttributes, Bytes::from(vec![1, 0, 1]));

        assert_eq!(parsed.typeAttributes[1].moduleType, ERC7579ModuleType::Executor);
        assert_eq!(parsed.typeAttributes[1].encodedAttributes, Bytes::from(vec![1, 0, 1]));

        assert_eq!(parsed.typeAttributes[2].moduleType, ERC7579ModuleType::Fallback);
        assert_eq!(parsed.typeAttributes[2].encodedAttributes, Bytes::from(vec![1, 0]));

        assert_eq!(parsed.typeAttributes[3].moduleType, ERC7579ModuleType::Hook);
        assert_eq!(parsed.typeAttributes[3].encodedAttributes, Bytes::from(vec![1, 0, 1, 0, 1]));
    }
}

