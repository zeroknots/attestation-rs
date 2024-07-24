
use alloy_primitives::{Address, Bytes};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
mod types;
mod abi;
use crate::types::Input;
use crate::abi::ToSol;

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
    let abi_audit_summary = input.to_sol();
    println!("{:#?}", abi_audit_summary);
    
    // Map to ABI-compatible structs
    
    // println!("{:#?}", abi_audit_summary);
    //

    Ok(())
}
