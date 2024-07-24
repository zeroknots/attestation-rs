
use alloy_sol_types::{sol_data::*, SolType};
use alloy_primitives::{Address, Bytes, U256, keccak256};
use alloy_sol_types::{sol, EventTopic};
use core::hash::Hash;
use std::str::FromStr;

use crate::types::{Input, JsonAuditor, JsonModuleAttributes, JsonValidatorAttributes, JsonExecutorAttributes, JsonFallbackAttributes, JsonHookAttributes};
// ABI-compatible structs
sol! {

    #[derive(Debug)] 
    struct ValidatorAttributes {
        uint8[] classifications;
    }

    #[derive(Debug)] 
    struct ExecutorAttributes {
        bool handlesUserAssets;
        bool noDelegateCall;
        uint8[] classifications;
    }

    #[derive(Debug)] 
    struct FallbackAttributes {
        uint8[] classifications;
    }

    #[derive(Debug)] 
    struct HookAttributes {
        uint8[] classifications;
    }

    #[derive(Debug)] 
    struct ModuleTypeAttributes {
        uint8 moduleType;
        bytes encodedAttributes;
    }

    #[derive(Debug)] 
    struct ModuleAttributes {
        address moduleAddress;
        bytes packedAttributes;
        ModuleTypeAttributes[] typeAttributes;
    }


    #[derive(Debug)] 
    struct Auditor {
        string name;
        string uri;
        string[] authors;
    }

    #[derive(Debug)] 
    struct Signature {
        uint8 sigType;
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
pub trait ToSol {
    type SolType;
    fn to_sol(&self) -> Self::SolType;
}


trait FromSol {
    type SolType;
    fn from_sol(sol: &Self::SolType) -> Self;
}

impl ToSol for JsonValidatorAttributes {
    type SolType = ValidatorAttributes;
    fn to_sol(&self) -> Self::SolType {
        let classifications = [self.classifications.unscoped_validator, self.classifications.recovery_module, self.classifications.multiplexer];
        ValidatorAttributes {
            classifications: classifications.iter().map(|x| *x as u8).collect(),
        }
    }
}

impl ToSol for JsonExecutorAttributes {
    type SolType = ExecutorAttributes;
    fn to_sol(&self) -> Self::SolType {
        let classifications = [
            self.classifications.triggered_by_account,
            self.classifications.triggered_by_relayer,
            self.classifications.deterministic_execution,
        ];
        ExecutorAttributes {
            handlesUserAssets: self.handles_user_assets,
            noDelegateCall: self.no_delegate_call,
            classifications: classifications.iter().map(|x| *x as u8).collect(),
        }
    }
}

impl ToSol for JsonFallbackAttributes {
    type SolType = FallbackAttributes;
    fn to_sol(&self) -> Self::SolType {
        let classifications = [
            self.classifications.compatibility_feature,
            self.classifications.callbacks,
        ];
        FallbackAttributes {
            classifications: classifications.iter().map(|x| *x as u8).collect(),
        }
    }
}

impl ToSol for JsonHookAttributes {
    type SolType = HookAttributes;
    fn to_sol(&self) -> Self::SolType {
        let classifications = [
            self.classifications.default_allow,
            self.classifications.default_deny,
            self.classifications.access_control,
            self.classifications.module_control,
            self.classifications.user_control,
        ];
        HookAttributes {
            classifications: classifications.iter().map(|x| *x as u8).collect(),
        }
    }
}




impl ToSol for JsonModuleAttributes {
    type SolType = ModuleAttributes;
    fn to_sol(&self) -> Self::SolType {
        let mut packed_attributes = Vec::new();

        // Global attributes
        packed_attributes.extend_from_slice(&[
            self.global_attributes.reentrancy_protection as u8,
            self.global_attributes.important_data_validation as u8,
            self.global_attributes.input_manipulation_protection as u8,
            self.global_attributes.emits_events as u8,
            self.global_attributes.module_owner_cant_rug as u8,
            self.global_attributes.upgradeable as u8,
            self.global_attributes.pausable as u8,
            self.global_attributes.licensed_module as u8,
            self.global_attributes.erc7562_storage_compliant as u8,
            self.global_attributes.uninstall_clean_up as u8,
        ]);

        // Validator attributes
        packed_attributes.extend_from_slice(&[
            self.validator_attributes.classifications.unscoped_validator as u8,
            self.validator_attributes.classifications.recovery_module as u8,
            self.validator_attributes.classifications.multiplexer as u8,
        ]);

        // Executor attributes
        packed_attributes.extend_from_slice(&[
            self.executor_attributes.handles_user_assets as u8,
            self.executor_attributes.no_delegate_call as u8,
            self.executor_attributes.classifications.triggered_by_account as u8,
            self.executor_attributes.classifications.triggered_by_relayer as u8,
            self.executor_attributes.classifications.deterministic_execution as u8,
        ]);

        // Fallback attributes
        packed_attributes.extend_from_slice(&[
            self.fallback_attributes.classifications.compatibility_feature as u8,
            self.fallback_attributes.classifications.callbacks as u8,
        ]);

        // Hook attributes
        packed_attributes.extend_from_slice(&[
            self.hook_attributes.classifications.default_allow as u8,
            self.hook_attributes.classifications.default_deny as u8,
            self.hook_attributes.classifications.access_control as u8,
            self.hook_attributes.classifications.module_control as u8,
            self.hook_attributes.classifications.user_control as u8,
        ]);

        // Create ModuleTypeAttributes
        let type_attributes = vec![
            ModuleTypeAttributes {
                moduleType: 1,
                encodedAttributes: packed_attributes[10..13].to_vec(),
            },
            ModuleTypeAttributes {
                moduleType: 2,
                encodedAttributes: packed_attributes[13..18].to_vec(),
            },
            ModuleTypeAttributes {
                moduleType: 3,
                encodedAttributes: packed_attributes[18..20].to_vec(),
            },
            ModuleTypeAttributes {
                moduleType: 4,
                encodedAttributes: packed_attributes[20..25].to_vec(),
            },
        ];

        ModuleAttributes {
            moduleAddress: self.module_address,
            packedAttributes: packed_attributes,
            typeAttributes: type_attributes,
        }
    }
}






impl ToSol for JsonAuditor {
    type SolType = Auditor;
    fn to_sol(&self) -> Self::SolType {
        Auditor {
            name: self.name.clone(),
            uri: self.uri.clone(),
            authors: self.authors.clone(),
        }
    }
}

// You might also want to implement ToSol for the entire Input struct
impl ToSol for Input {
    type SolType = AuditSummary;
    fn to_sol(&self) -> Self::SolType {
        AuditSummary {
            title: self.title.clone(),
            auditor: self.auditor.to_sol(),
            moduleAttributes: self.module_attributes.to_sol(),
            signature: Signature { // This is a placeholder, adjust as needed
                sigType: 0,
                signer: Address::ZERO,
                signatureData: vec![].into(),
            },
        }
    }
}



