use clap::Parser;
use std::fs;
use std::path::PathBuf;
mod abi;
mod structs;
mod types;
use crate::abi::ParseAttributes;
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
    let abi_audit_summary = input.module_attributes.parse();
    println!("{:#?}", abi_audit_summary);


    Ok(())
}
