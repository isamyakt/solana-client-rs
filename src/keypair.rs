use solana_sdk::signer::keypair::Keypair;
use std::path::Path;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;

pub async fn initialize_keypair() -> Keypair {
    let file_path = ".env";

    // Check if the file exists
    let file_exists = Path::new(file_path).exists();

    if !file_exists {
        File::create(file_path).unwrap();
    }

    match env::var("PRIVATE_KEY") {
        Ok(_) => {
            let env_private_key = env::var("PRIVATE_KEY").unwrap();
            let trim_private_key = env_private_key
                .trim_matches(|c| c == '[' || c == ']' || c == '"');

            let vec_private_key: Vec<u8> = trim_private_key
                .split(",")
                .map(| s| s.trim().parse().unwrap())
                .collect();

            let bytes_private_key: [u8; 64] = vec_private_key
                .try_into()
                .expect("Expected a Vec of length 64");

            let keypair = Keypair::from_bytes(&bytes_private_key).unwrap();

            keypair
        },
        Err(_) => {
            let keypair = Keypair::new();
            let keypair_bytes = keypair.to_bytes();

            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(".env")
                .unwrap();
            
            writeln!(file, "PRIVATE_KEY=\"{:?}\"", keypair_bytes).unwrap();

            keypair
        }
    }
}