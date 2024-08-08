use alloy_primitives::{Address, Bytes};
use alloy_sol_types::SolValue;
use clap::{Parser, Subcommand};

use std::path::PathBuf;
use std::{fs, str::FromStr};
mod abi;
mod types;
use crate::abi::{HashAuditSummary, PackedSig, ParseAttributes, SignAttestation, SignatureType};
use crate::types::{Input, JsonSignature};
use alloy_signer::{Signer, SignerSync};
use alloy_signer_local::PrivateKeySigner;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};


#[derive(Subcommand, Debug)]
#[command(author, version, about, long_about = None)]
enum SignerType {
    /// Interactive mode
    Interactive,

    /// Private key mode
    PrivateKey {
        #[arg(short, long)]
        private_key: String,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the JSON input file
    #[arg(short, long)]
    input: PathBuf,
    #[arg(short, long)]
    private_key: Option<String>,
    #[arg(short, long)]
    mode: String,
    #[command(subcommand)]
    signer_type: Option<SignerType>,
}
fn to_hex_string(bytes: &[u8]) -> String {
    let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    format!("0x{}", hex)
}
fn from_hex_string(hex_string: &str) -> Result<Vec<u8>, hex::FromHexError> {
    // Remove "0x" prefix if present
    let cleaned_hex = hex_string.strip_prefix("0x").unwrap_or(hex_string);
    
    // Use the hex crate to decode the string
    hex::decode(cleaned_hex)
}

fn to_outfile(path_buf: &PathBuf) -> PathBuf {
    let file_stem = path_buf.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let extension = path_buf.extension().and_then(|s| s.to_str()).unwrap_or("");

    let new_file_stem = format!("{}.signed", file_stem);

    if extension.is_empty() {
        path_buf.with_file_name(new_file_stem)
    } else {
        path_buf.with_file_name(format!("{}.{}", new_file_stem, extension))
    }
}

fn add_signed_to_filename(path: &Path) -> PathBuf {
    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let new_file_stem = format!("{}.signed", file_stem);

    let mut new_path = path.with_file_name(new_file_stem);
    if !extension.is_empty() {
        new_path.set_extension(extension);
    }

    new_path
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input_path = args.input.clone();
    // Read the JSON file
    let json_input = fs::read_to_string(args.input)?;
    // let private_key = args.private_key;

    let input: Input = serde_json::from_str(&json_input)?;
    // let signer = PrivateKeySigner::from_str(&private_key)?;
    let mut onchain_data = input.encode(SignatureType::SECP256K1, Address::default());

    match args.mode.as_str() {
        "print" => {
            println!("{:#?}", onchain_data);
            let data_bytes = Bytes::from(onchain_data.abi_encode());
            println!("{:?}", data_bytes);
        }
        "bytes" => {
            // println!("{:#?}", onchain_data);
            let data_bytes = Bytes::from(onchain_data.abi_encode());
            println!("{:?}", data_bytes);
        }
        "hash" => {
            let digest = onchain_data.signature.hash.to_vec();
            println!("{:?}", Bytes::from(digest.clone()));
        }
        "cast" => {

            let digest = onchain_data.signature.hash.to_vec();
            let digest_bytes = Bytes::from(digest.clone());
            println!("signing hash:  {:?}", digest_bytes);


            let foo = to_hex_string(&digest);

            let output;

            match args.signer_type{
                Some(SignerType::PrivateKey{private_key}) => {
                     output = Command::new("cast")
                        .arg("wallet")
                        .arg("sign")
                        .arg(foo)
                        .arg("--private-key")
                        .arg(private_key)
                        .output()
                        .expect("Failed to execute command");
                },
                Some(SignerType::Interactive) => {
                     output = Command::new("cast")
                        .arg("wallet")
                        .arg("sign")
                        .arg(foo)
                        .arg("--interactive")
                        .output()
                        .expect("Failed to execute command");
                },
                _ => {
                    println!("Invalid mode");
                    exit(-1);
                }
            }


            let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
            let signature = stdout.trim().to_string(); // R

            let mut output = input;
            output.signature = Some(JsonSignature {
                hash: to_hex_string(&digest),
                signature: Bytes::from(from_hex_string(&signature).unwrap()),
            });

            // Create the outfile name and keep it in a variable
            let outfile_name_string = to_outfile(&input_path);
            let outfile_name = Path::new(&outfile_name_string);
            println!("written to: {:?}", outfile_name);

            // Create the file
            let mut file = File::create(outfile_name)?;

            // Serialize the struct to JSON
            let json_string = serde_json::to_string(&output)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

            // Write the JSON to the file
            file.write_all(json_string.as_bytes())?;
        }
        "sign" => {
            // let private_key = args.private_key;
            //
            // let input: Input = serde_json::from_str(&json_input)?;
            // let signer = PrivateKeySigner::from_str(&private_key.unwrap())?;
            // onchain_data = input.encode(SignatureType::SECP256K1, signer.address());
            //
            // // let data_bytes = Bytes::from(onchain_data.abi_encode());
            //
            // let digest = onchain_data.signature.hash.to_vec();
            // // println!("{:?}", Bytes::from(digest.clone()));
            //
            // // let signer = PrivateKeySigner::from_str(&private_key.unwrap())?;
            // let sig = signer.sign_message_sync(&digest)?;
            // // println!("{:?}", sig);
            //
            // let packed_sig: PackedSig = PackedSig {
            //     r: sig.r().into(),
            //     s: sig.s().into(),
            //     v: sig.v().to_u64() as u8,
            // };
            //
            // let packed = PackedSig::abi_encode_packed(&packed_sig);
            // // println!("{:?}", packed);
            // let packed_bytes = Bytes::from(packed);
            // onchain_data.signature.signatureData = packed_bytes;
            // // onchain_data.signature.signatureData = signer.sign_message_sync(&digest)?;
            // let data_bytes = Bytes::from(onchain_data.abi_encode());
            // println!("{:?}", data_bytes);
        }
        _ => {
            println!("Invalid mode");
        }
    }

    Ok(())
}
