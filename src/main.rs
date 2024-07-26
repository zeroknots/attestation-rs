use alloy_primitives::Bytes;
use alloy_sol_types::SolValue;
use clap::Parser;
use std::path::PathBuf;
use std::{fs, str::FromStr};
mod abi;
mod types;
use crate::abi::{SignatureType, ParseAttributes, SignAttestation, HashAuditSummary};
use crate::types::Input;
use alloy_signer::{Signer, SignerSync};
use alloy_signer_local::PrivateKeySigner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON input file
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    private_key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read the JSON file
    let json_input = fs::read_to_string(args.input)?;
    let private_key = args.private_key;
    // // Instantiate a signer.
    let signer = PrivateKeySigner::from_str(&private_key)?;

    let input: Input = serde_json::from_str(&json_input)?;

    let onchain_data = input.encode(SignatureType::SECP256K1, signer.address());
    println!("{:#?}", onchain_data);
    let data_bytes = Bytes::from(onchain_data.abi_encode());
    println!("{:?}", data_bytes);

    let digest = onchain_data.digest();
    println!("{:?}", digest);
    //
    // println!("{:#?}", input);
    // let abi_audit_summary = input.module_attributes.encode();
    // println!("{:#?}", abi_audit_summary);
    //
    // let data_bytes = Bytes::from(abi_audit_summary.abi_encode());
    // println!("{:?}", data_bytes);
    //
    // // Instantiate a signer.
    // let signer = PrivateKeySigner::from_str(&private_key)?;
    // println!("{:?}", signer.address());
    //
    // let signature = signer.sign_message_sync(&digest)?;
    // println!("{:?}", signature);

    Ok(())
}
