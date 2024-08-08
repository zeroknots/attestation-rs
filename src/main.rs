use alloy_primitives::{Address, Bytes};
use alloy_sol_types::SolValue;
use clap::Parser;

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
use std::process::Command;

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
    #[arg(short, long)]
    signer_type: Option<String>,
}
fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
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
            println!("{:?}", Bytes::from(digest.clone()));

            let signer = onchain_data.signature.signer.to_string();

            let signer_type = args.signer_type.unwrap();
            let cast_type = "--".to_owned();

            let param = cast_type + &signer_type;

            println!("{:?}", param);

            let command = "cast";
            let output = Command::new(command)
                .arg("wallet")
                .arg("sign")
                .arg("--from")
                .arg(signer)
                .arg(to_hex_string(&digest))
                .arg(param)
                .output()
                .expect("Failed to execute command");

            println!("Status: {}", output.status);
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            let signature = String::from_utf8_lossy(&output.stdout);

            let mut output = input;
            output.signature = Some(JsonSignature {
                hash: to_hex_string(&digest),
                signature: signature.to_string().trim().to_owned(),
            });

            println!("{:?}", output);

            // Create the outfile name and keep it in a variable
            let outfile_name_string = to_outfile(&input_path);
            let outfile_name = Path::new(&outfile_name_string);
            println!("{:?}", outfile_name);

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
