#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;

use anyhow::Error;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        _ => println!("unknown command: {}", args[1]),
    }
}

fn get_hash_path_sha(hash: &str) -> anyhow::Result<(&str, &str)> {
    if hash.len() != 40 {
        return Err(Error::msg("invalid sha length"));
    }
    Ok(hash.split_at(2))
}
