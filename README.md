# Dependencies

Add the following dependencies to your *Cargo.toml*:

```
[dependencies]
ring = "0.16.20"
getrandom = "0.2"
hex = "0.4"
```

# Building and Running the Binaries

To build the binaries, add the following to your *Cargo.toml*:

```
[[bin]]
name = "ring_enc"
path = "src/ring_enc.rs"

[[bin]]
name = "ring_dec"
path = "src/ring_dec.rs"
```

```
cargo clean
cargo build --release
```

# Encrypt a file

```
./target/release/ring_enc input.txt encrypted.bin <hex_key>
```

# Decrypt the file
```
./target/release/ring_dec encrypted.bin decrypted.txt <hex_key>
```

Replace <hex_key> with a 64-character hex string representing a 32-byte key.

```
hexdump -vn32 -e'4/4 "%08X" 1 "\n"' /dev/urandom | tr -d '\n'
openssl rand -hex 32
echo $(uuidgen)$(uuidgen) | tr -d '-'
```
