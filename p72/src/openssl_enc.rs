use getrandom::getrandom;
use openssl::symm::{encrypt_aead, Cipher};
use std::env;
use std::fs::{read, File};
use std::io::Write;

const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <input_file> <output_file> <key>", args[0]);
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let key = hex::decode(&args[3]).expect("Invalid hex key");

    if key.len() != 32 {
        eprintln!("Key must be 32 bytes (64 hex characters)");
        std::process::exit(1);
    }

    let input_data = read(input_file).expect("Failed to read input file");

    let mut nonce = [0u8; NONCE_LEN];
    getrandom(&mut nonce).expect("Failed to generate nonce");

    let cipher = Cipher::aes_256_gcm();
    let mut tag = [0u8; TAG_LEN];

    let ciphertext = encrypt_aead(cipher, &key, Some(&nonce), &[], &input_data, &mut tag)
        .expect("Encryption failed");

    let mut output = File::create(output_file).expect("Failed to create output file");
    output.write_all(&nonce).expect("Failed to write nonce");
    output.write_all(&tag).expect("Failed to write tag");
    output
        .write_all(&ciphertext)
        .expect("Failed to write ciphertext");
}
