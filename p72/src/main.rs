fn main() {
    println!("Hello, world!");
}

// Dependencies
// Add the following dependencies to your Cargo.toml:
// 
// [dependencies]
// openssl = "0.10"
// getrandom = "0.2"
// hex = "0.4"
 
// Building and Running the Binaries
// To build the binaries, add the following to your Cargo.toml:

// [[bin]]
// name = "openssl_enc"
// path = "bin/openssl_enc.rs"

// [[bin]]
// name = "openssl_dec"
// path = "bin/openssl_dec.rs"

// cargo build --release
//
// # Encrypt a file
// ./target/release/openssl_enc input.txt encrypted.bin <hex_key>

// # Decrypt the file
// ./target/release/openssl_dec encrypted.bin decrypted.txt <hex_key>

// Replace <hex_key> with a 64-character hex string representing a 32-byte key.

// hexdump -vn32 -e'4/4 "%08X" 1 "\n"' /dev/urandom | tr -d '\n'
// uuidgen | tr -d '-'
// openssl rand -hex 32

// This implementation ensures that the data is securely encrypted and decrypted using AES-256-GCM, and it appends the nonce to the encrypted file for later use during decryption.