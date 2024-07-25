use alloy_primitives::{Address, Bytes};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
mod abi;
mod structs;
mod types;
use crate::abi::{AuditSummary, Auditor, ModuleAttributes, ERC7579ModuleType, ModuleTypeAttributes, Signature, SignatureType, PackableAttributes};
use crate::types::Input;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON input file
    #[arg(short, long)]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read the JSON file
    let json_input = fs::read_to_string(args.input)?;

    let input: Input = serde_json::from_str(&json_input)?;

    println!("{:#?}", input);

    let abi_audit_summary = AuditSummary {
        title: input.title,
        auditor: Auditor {
            name: input.auditor.name,
            uri: input.auditor.uri,
            authors: input.auditor.authors,
        },
        signature: Signature {
            sigType: SignatureType::SECP256K1,
            signer: Address::from_str("0x5c49Ff8088B92c6De258B286649FF85E2822451e")?,
            signatureData: Bytes::from(vec![]),
        },
        moduleAttributes: ModuleAttributes {
            moduleAddress: input.module_attributes.module_address,
            packedAttributes: input.module_attributes.global_attributes.pack(),
            typeAttributes: vec![ModuleTypeAttributes {
                moduleType: ERC7579ModuleType::None,
                encodedAttributes: Bytes::from(vec![]),
            }],
        },
    };
    println!("{:#?}", abi_audit_summary);

    // Map to ABI-compatible structs

    // println!("{:#?}", abi_audit_summary);
    //

    Ok(())
}
