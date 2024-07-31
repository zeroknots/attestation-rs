use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use alloy_sol_types::{abi, sol};
use alloy_sol_types::{sol_data::*, SolValue};

use crate::types::{
    Input, JsonExecutorAttributes, JsonExternalDependency, JsonFallbackAttributes,
    JsonGlobalAttributes, JsonHookAttributes, JsonModuleAttributes, JsonValidatorAttributes,
};
use std::error::Error;

sol! {

#[derive(Debug)]
struct PackedSig {
bytes32 r;
bytes32 s;
uint8 v;
}
}

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
    bytes packedExternalDependency;
    uint16[] ercDeps;
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
    bytes32 hash;
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
    fn unpack(bytes: &Bytes) -> Result<Box<Self>, Box<dyn Error>>;
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
    fn unpack(bytes: &Bytes) -> Result<Box<Self>, Box<dyn Error>> {
        if bytes.len() != 10 {
            return Err("Invalid byte length for JsonGlobalAttributes".into());
        }
        Ok(Box::new(JsonGlobalAttributes {
            reentrancy_protection: bytes[0] != 0,
            important_data_validation: bytes[1] != 0,
            input_manipulation_protection: bytes[2] != 0,
            emits_events: bytes[3] != 0,
            module_owner_cant_rug: bytes[4] != 0,
            upgradeable: bytes[5] != 0,
            pausable: bytes[6] != 0,
            licensed_module: bytes[7] != 0,
            erc7562_storage_compliant: bytes[8] != 0,
            uninstall_clean_up: bytes[9] != 0,
        }))
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
    fn unpack(bytes: &Bytes) -> Result<Box<Self>, Box<dyn Error>> {
        if bytes.len() != 3 {
            return Err("Invalid byte length for JsonValidatorAttributes".into());
        }
        Ok(Box::new(JsonValidatorAttributes {
            unscoped_validator: bytes[0] != 0,
            recovery_module: bytes[1] != 0,
            multiplexer: bytes[2] != 0,
        }))
    }
}

impl PackableAttributes for JsonExecutorAttributes {
    fn pack(&self) -> Bytes {
        let list = vec![
            self.handles_user_assets as u8,
            self.delegate_call as u8,
            self.triggered_by_account as u8,
            self.triggered_by_relayer as u8,
            self.deterministic_execution as u8,
        ];

        Bytes::from(list)
    }
    fn unpack(bytes: &Bytes) -> Result<Box<Self>, Box<dyn Error>> {
        if bytes.len() != 5 {
            // Changed from 3 to 5
            return Err("Invalid byte length for JsonExecutorAttributes".into());
        }
        Ok(Box::new(JsonExecutorAttributes {
            handles_user_assets: bytes[0] != 0,
            delegate_call: bytes[1] != 0,
            triggered_by_account: bytes[2] != 0,
            triggered_by_relayer: bytes[3] != 0,
            deterministic_execution: bytes[4] != 0,
        }))
    }
}

impl PackableAttributes for JsonFallbackAttributes {
    fn pack(&self) -> Bytes {
        let list = vec![self.compatibility_feature as u8, self.callbacks as u8];

        Bytes::from(list)
    }
    fn unpack(bytes: &Bytes) -> Result<Box<Self>, Box<dyn Error>> {
        if bytes.len() != 2 {
            return Err("Invalid byte length for JsonFallbackAttributes".into());
        }
        Ok(Box::new(JsonFallbackAttributes {
            compatibility_feature: bytes[0] != 0,
            callbacks: bytes[1] != 0,
        }))
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
    fn unpack(bytes: &Bytes) -> Result<Box<Self>, Box<dyn Error>> {
        if bytes.len() != 5 {
            return Err("Invalid byte length for JsonHookAttributes".into());
        }
        Ok(Box::new(JsonHookAttributes {
            default_allow: bytes[0] != 0,
            default_deny: bytes[1] != 0,
            access_control: bytes[2] != 0,
            module_control: bytes[3] != 0,
            user_control: bytes[4] != 0,
        }))
    }
}

impl PackableAttributes for JsonExternalDependency {
    fn pack(&self) -> Bytes {
        let list = vec![
            self.oracle as u8,
            self.bridges as u8,
            self.dexs as u8,
            self.vaults as u8,
            self.registry as u8,
            self.lending as u8,
            self.liquidity_provision as u8,
            self.governance as u8,
            self.privacy as u8,
            self.zk_provers as u8,
        ];

        Bytes::from(list)
    }

    fn unpack(bytes: &Bytes) -> Result<Box<Self>, Box<dyn Error>> {
        if bytes.len() != 10 {
            return Err("Invalid byte length for JsonExternalDependency".into());
        }
        Ok(Box::new(JsonExternalDependency {
            oracle: bytes[0] != 0,
            bridges: bytes[1] != 0,
            dexs: bytes[2] != 0,
            vaults: bytes[3] != 0,
            registry: bytes[4] != 0,
            lending: bytes[5] != 0,
            liquidity_provision: bytes[6] != 0,
            governance: bytes[7] != 0,
            privacy: bytes[8] != 0,
            zk_provers: bytes[9] != 0,
            erc_deps: vec![],
        }))
    }
}

pub trait ParseAttributes {
    fn encode(&self) -> ModuleAttributes;
}
impl ParseAttributes for JsonModuleAttributes {
    fn encode(&self) -> ModuleAttributes {
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
            packedExternalDependency: self.external_dependency.pack(),
            ercDeps: self.external_dependency.erc_deps.clone(),
        };

        module_attributes
    }
}

pub trait SignAttestation {
    fn encode(&self, sig_type: SignatureType, signer: Address) -> AuditSummary;
}

impl SignAttestation for Input {
    fn encode(&self, sig_type: SignatureType, signer: Address) -> AuditSummary {
        let mut hash: U256 = "42".parse().unwrap();
        let mut summary = AuditSummary {
            title: self.title.clone(),
            auditor: Auditor {
                name: self.auditor.name.clone(),
                uri: self.auditor.uri.clone(),
                authors: self.auditor.authors.clone(),
            },
            moduleAttributes: self.module_attributes.encode(),
            signature: Signature {
                sigType: sig_type,
                signer:self.signer,
                signatureData: Bytes::default(), // You might want to set this to actual signature data
                hash: hash.into(),
            },
        };

        let actual_hash = summary.digest();

        summary.signature.hash = actual_hash.into();

        summary
    }
}

sol! {
    #[derive(Debug)]
    struct Digest {
        string title;
        Auditor auditor;
        ModuleAttributes moduleAttributes;
    }

}

pub trait HashAuditSummary {
    fn digest(&self) -> B256;
    fn encode(&self) -> Bytes;
}

impl HashAuditSummary for AuditSummary {
    fn encode(&self) -> Bytes {
        let data: Digest = Digest {
            title: self.title.clone(),
            auditor: self.auditor.clone(),
            moduleAttributes: self.moduleAttributes.clone(),
        };

        let encoded: Vec<u8> = Digest::abi_encode(&data);
        Bytes::from(encoded)
    }
    fn digest(&self) -> B256 {
        // First, we need to ABI encode the AuditSummary
        let encoded = self.encode();

        // Then, we compute the Keccak-256 hash
        keccak256(&encoded)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, Bytes};

    #[test]
    fn test_global_attributes_pack_unpack() {
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
        let unpacked = JsonGlobalAttributes::unpack(&packed).unwrap();

        assert_eq!(*unpacked, global_attrs);
    }

    #[test]
    fn test_validator_attributes_pack_unpack() {
        let validator_attrs = JsonValidatorAttributes {
            unscoped_validator: true,
            recovery_module: false,
            multiplexer: true,
        };

        let packed = validator_attrs.pack();
        let unpacked = JsonValidatorAttributes::unpack(&packed).unwrap();

        assert_eq!(*unpacked, validator_attrs);
    }

    #[test]
    fn test_executor_attributes_pack_unpack() {
        let executor_attrs = JsonExecutorAttributes {
            handles_user_assets: true,
            no_delegate_call: false,
            triggered_by_account: true,
            triggered_by_relayer: false,
            deterministic_execution: true,
        };

        let packed = executor_attrs.pack();
        let unpacked = JsonExecutorAttributes::unpack(&packed).unwrap();

        assert_eq!(*unpacked, executor_attrs);
    }

    #[test]
    fn test_fallback_attributes_pack_unpack() {
        let fallback_attrs = JsonFallbackAttributes {
            compatibility_feature: true,
            callbacks: false,
        };

        let packed = fallback_attrs.pack();
        let unpacked = JsonFallbackAttributes::unpack(&packed).unwrap();

        assert_eq!(*unpacked, fallback_attrs);
    }

    #[test]
    fn test_hook_attributes_pack_unpack() {
        let hook_attrs = JsonHookAttributes {
            default_allow: true,
            default_deny: false,
            access_control: true,
            module_control: false,
            user_control: true,
        };

        let packed = hook_attrs.pack();
        let unpacked = JsonHookAttributes::unpack(&packed).unwrap();

        assert_eq!(*unpacked, hook_attrs);
    }

    #[test]
    fn test_external_dependency_pack_unpack() {
        let external_dependency = JsonExternalDependency {
            oracle: true,
            bridges: false,
            dexs: true,
            vaults: false,
            registry: true,
            lending: false,
            liquidity_provision: true,
            governance: false,
            privacy: true,
            zk_provers: false,
            erc_deps: vec![],
        };

        let packed = external_dependency.pack();
        let unpacked = JsonExternalDependency::unpack(&packed).unwrap();

        assert_eq!(*unpacked, external_dependency);
    }

    #[test]
    fn test_module_attributes_encode() {
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
                delegate_call: false,
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
                oracle: true,
                bridges: false,
                dexs: true,
                vaults: false,
                registry: true,
                lending: false,
                liquidity_provision: true,
                governance: false,
                privacy: true,
                zk_provers: false,
                erc_deps: vec![],
            },
        };

        let encoded = module_attrs.encode();

        assert_eq!(encoded.moduleAddress, module_attrs.module_address);
        assert_eq!(
            encoded.packedAttributes,
            module_attrs.global_attributes.pack()
        );
        assert_eq!(
            encoded.packedExternalDependency,
            module_attrs.external_dependency.pack()
        );
        assert_eq!(encoded.typeAttributes.len(), 4);

        assert_eq!(
            encoded.typeAttributes[0].moduleType,
            ERC7579ModuleType::Validator
        );
        assert_eq!(
            encoded.typeAttributes[0].encodedAttributes,
            module_attrs.validator_attributes.pack()
        );

        assert_eq!(
            encoded.typeAttributes[1].moduleType,
            ERC7579ModuleType::Executor
        );
        assert_eq!(
            encoded.typeAttributes[1].encodedAttributes,
            module_attrs.executor_attributes.pack()
        );

        assert_eq!(
            encoded.typeAttributes[2].moduleType,
            ERC7579ModuleType::Fallback
        );
        assert_eq!(
            encoded.typeAttributes[2].encodedAttributes,
            module_attrs.fallback_attributes.pack()
        );

        assert_eq!(
            encoded.typeAttributes[3].moduleType,
            ERC7579ModuleType::Hook
        );
        assert_eq!(
            encoded.typeAttributes[3].encodedAttributes,
            module_attrs.hook_attributes.pack()
        );
    }

    #[test]
    fn test_invalid_byte_length() {
        // Invalid length for JsonGlobalAttributes (expects 10 bytes)
        let invalid_global = Bytes::from(vec![0; 9]);
        assert!(JsonGlobalAttributes::unpack(&invalid_global).is_err());

        // Invalid length for JsonValidatorAttributes (expects 3 bytes)
        let invalid_validator = Bytes::from(vec![0; 2]);
        assert!(JsonValidatorAttributes::unpack(&invalid_validator).is_err());

        // Invalid length for JsonExecutorAttributes (expects 5 bytes)
        let invalid_executor = Bytes::from(vec![0; 4]);
        assert!(JsonExecutorAttributes::unpack(&invalid_executor).is_err());

        // Invalid length for JsonFallbackAttributes (expects 2 bytes)
        let invalid_fallback = Bytes::from(vec![0; 1]);
        assert!(JsonFallbackAttributes::unpack(&invalid_fallback).is_err());

        // Invalid length for JsonHookAttributes (expects 5 bytes)
        let invalid_hook = Bytes::from(vec![0; 4]);
        assert!(JsonHookAttributes::unpack(&invalid_hook).is_err());

        // Invalid length for JsonExternalDependency (expects 10 bytes)
        let invalid_external = Bytes::from(vec![0; 9]);
        assert!(JsonExternalDependency::unpack(&invalid_external).is_err());
    }
}
