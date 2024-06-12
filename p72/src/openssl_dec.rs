use openssl::symm::{decrypt_aead, Cipher};
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

    if input_data.len() < NONCE_LEN + TAG_LEN {
        eprintln!("Invalid input file: too short");
        std::process::exit(1);
    }

    let nonce = &input_data[..NONCE_LEN];
    let tag = &input_data[NONCE_LEN..NONCE_LEN + TAG_LEN];
    let ciphertext = &input_data[NONCE_LEN + TAG_LEN..];

    let cipher = Cipher::aes_256_gcm();

    let plaintext =
        decrypt_aead(cipher, &key, Some(nonce), &[], ciphertext, tag).expect("Decryption failed");

    let mut output = File::create(output_file).expect("Failed to create output file");
    output
        .write_all(&plaintext)
        .expect("Failed to write plaintext");
}
