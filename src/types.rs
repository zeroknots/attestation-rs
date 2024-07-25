use alloy_primitives::{Address, Bytes};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;


// JSON input structs
#[derive(Debug, Serialize, Deserialize)]
pub struct Input {
    pub title: String,
    pub auditor: JsonAuditor,
    #[serde(rename = "reportUrl")]
    pub report_url: String,
    #[serde(rename = "moduleAttributes")]
    pub module_attributes: JsonModuleAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonAuditor {
    pub name: String,
    pub uri: String,
    pub authors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonModuleAttributes {
    #[serde(rename = "moduleAddress")]
    pub module_address: Address,
    #[serde(rename = "globalAttributes")]
    pub global_attributes: JsonGlobalAttributes,
    #[serde(rename = "validatorAttributes")]
    pub validator_attributes: JsonValidatorAttributes,
    #[serde(rename = "executorAttributes")]
    pub executor_attributes: JsonExecutorAttributes,
    #[serde(rename = "fallbackAttributes")]
    pub fallback_attributes: JsonFallbackAttributes,
    #[serde(rename = "hookAttributes")]
    pub hook_attributes: JsonHookAttributes,
    #[serde(rename = "externalDependency")]
    pub external_dependency: JsonExternalDependency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonGlobalAttributes {
    #[serde(rename = "reentrancyProtection")]
    pub reentrancy_protection: bool,
    #[serde(rename = "importantDataValidation")]
    pub important_data_validation: bool,
    #[serde(rename = "inputManipulationProtection")]
    pub input_manipulation_protection: bool,
    #[serde(rename = "emitsEvents")]
    pub emits_events: bool,
    #[serde(rename = "moduleOwnerCantRug")]
    pub module_owner_cant_rug: bool,
    pub upgradeable: bool,
    pub pausable: bool,
    #[serde(rename = "licensedModule")]
    pub licensed_module: bool,
    #[serde(rename = "erc7562StorageCompliant")]
    pub erc7562_storage_compliant: bool,
    #[serde(rename = "uninstallCleanUp")]
    pub uninstall_clean_up: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonValidatorAttributes {
    pub classifications: JsonValidatorClassifications,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonValidatorClassifications {
    #[serde(rename = "unscopedValidator")]
    pub unscoped_validator: bool,
    #[serde(rename = "recoveryModule")]
    pub recovery_module: bool,
    pub multiplexer: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonExecutorAttributes {
    #[serde(rename = "handlesUserAssets")]
    pub handles_user_assets: bool,
    #[serde(rename = "noDelegateCall")]
    pub no_delegate_call: bool,
    pub classifications: JsonExecutorClassifications,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonExecutorClassifications {
    #[serde(rename = "triggeredByAccount")]
    pub triggered_by_account: bool,
    #[serde(rename = "triggeredByRelayer")]
    pub triggered_by_relayer: bool,
    #[serde(rename = "deterministicExecution")]
    pub deterministic_execution: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFallbackAttributes {
    pub classifications: JsonFallbackClassifications,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonFallbackClassifications {
    #[serde(rename = "compatibilityFeature")]
    pub compatibility_feature: bool,
    pub callbacks: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonHookAttributes {
    pub classifications: JsonHookClassifications,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonHookClassifications {
    #[serde(rename = "defaultAllow")]
    pub default_allow: bool,
    #[serde(rename = "defaultDeny")]
    pub default_deny: bool,
    #[serde(rename = "accessControl")]
    pub access_control: bool,
    #[serde(rename = "moduleControl")]
    pub module_control: bool,
    #[serde(rename = "userControl")]
    pub user_control: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonExternalDependency {
    pub oracle: bool,
    pub bridges: bool,
    pub dexs: bool,
    pub vaults: bool,
    pub registry: bool,
    pub lending: bool,
    #[serde(rename = "liquidityProvision")]
    pub liquidity_provision: bool,
    pub governance: bool,
    pub privacy: bool,
    #[serde(rename = "zkProvers")]
    pub zk_provers: bool,
    #[serde(rename = "ercDeps")]
    pub erc_deps: Vec<u16>,
}


