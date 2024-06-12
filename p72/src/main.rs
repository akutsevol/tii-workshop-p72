const TEST_NAME: &str = "./bin/test.sh";
const ENCRYPTION: &str = "ring";

fn main() {
    println!("Running {}...", TEST_NAME);

    use std::process::Command;
    use std::str;

    let output = Command::new(TEST_NAME)
        .arg(ENCRYPTION)
        .output()
        .expect("Failed to execute command");

    // Convert the output to a string
    let output_str = str::from_utf8(&output.stdout).expect("Failed to convert output to string");

    println!("{}", output_str);
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_openssl_encrypting_decrypting_file() {
        let output = Command::new(crate::TEST_NAME)
            .arg(crate::ENCRYPTION)
            .output()
            .expect("Failed to execute command");

        println!("{:?}", output.stdout.as_slice());

        let expected: String = format!("Encryption name: {}\nDone: SUCCESS\n", crate::ENCRYPTION);
        assert_eq!(expected.as_bytes().to_vec(), output.stdout.as_slice());
    }
}
