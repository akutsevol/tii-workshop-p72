use ring::aead::{self, Aad, LessSafeKey, UnboundKey, Nonce};
use hex;
use std::fs;

const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: ring_dec <input_file> <output_file> <key_hex>");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let key_hex = &args[3];

    let key_bytes = hex::decode(key_hex).expect("Invalid hex key");
    if key_bytes.len() != 32 {
        eprintln!("Key must be 32 bytes (64 hex characters) long");
        std::process::exit(1);
    }

    let input_data = fs::read(input_file).expect("Failed to read input file");
    if input_data.len() < NONCE_LEN + TAG_LEN {
        eprintln!("Invalid input file");
        std::process::exit(1);
    }

    let nonce = &input_data[..NONCE_LEN];
    let ciphertext = &input_data[NONCE_LEN..];

    let key = UnboundKey::new(&aead::AES_256_GCM, &key_bytes).expect("Failed to create key");
    let key = LessSafeKey::new(key);

    let nonce = Nonce::assume_unique_for_key(nonce.try_into().unwrap());
    let mut in_out = ciphertext.to_vec();

    let decrypted_data = key
        .open_in_place(nonce, Aad::empty(), &mut in_out)
        .expect("Decryption failed");

    fs::write(output_file, decrypted_data).expect("Failed to write output file");
}
