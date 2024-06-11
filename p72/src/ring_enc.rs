use ring::aead::{self, Aad, LessSafeKey, UnboundKey, Nonce};
use ring::rand::{SecureRandom, SystemRandom};
use hex;
use std::fs;

const NONCE_LEN: usize = 12;
const TAG_LEN: usize = 16;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: ring_enc <input_file> <output_file> <key_hex>");
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
    let mut output_data = vec![0u8; NONCE_LEN + input_data.len() + TAG_LEN];

    let mut nonce = [0u8; NONCE_LEN];
    SystemRandom::new().fill(&mut nonce).expect("Failed to generate nonce");

    output_data[..NONCE_LEN].copy_from_slice(&nonce);

    let key = UnboundKey::new(&aead::AES_256_GCM, &key_bytes).expect("Failed to create key");
    let key = LessSafeKey::new(key);

    let nonce = Nonce::assume_unique_for_key(nonce);
    let mut in_out = input_data.to_vec();

    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .expect("Encryption failed");

    output_data[NONCE_LEN..].copy_from_slice(&in_out);

    fs::write(output_file, &output_data).expect("Failed to write output file");
}
