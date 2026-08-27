use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("usage: solana-keypair-base58 <keypair.json>");
            return ExitCode::from(2);
        }
    };

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(err) => {
            eprintln!("error: cannot read {path}: {err}");
            return ExitCode::from(1);
        }
    };

    let bytes: Vec<u8> = match serde_json::from_str(&content) {
        Ok(b) => b,
        Err(err) => {
            eprintln!("error: invalid keypair JSON in {path}: {err}");
            return ExitCode::from(1);
        }
    };

    if bytes.len() != 64 {
        eprintln!(
            "error: expected 64 bytes, got {} — not a valid Solana keypair",
            bytes.len()
        );
        return ExitCode::from(1);
    }

    println!("{}", bs58::encode(&bytes).into_string());
    ExitCode::SUCCESS
}
