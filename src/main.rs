use anyhow::Error;
use flate2::read::ZlibDecoder;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

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
        "cat-file" => {
            let (dir, file) = get_hash_path_sha(&args[3]).unwrap();
            let file = std::fs::read(Path::new(&format!(".git/objects/{dir}/{file}"))).unwrap();

            let decompress = decode_reader(file).unwrap();

            let test: Vec<&str> = decompress.split("\0").collect();
            print!("{}", test[1])
        }
        _ => panic!("unknown command: {}", args[1]),
    }
}

fn get_hash_path_sha(hash: &str) -> anyhow::Result<(&str, &str)> {
    if hash.len() != 40 {
        return Err(Error::msg("invalid sha length"));
    }
    Ok(hash.split_at(2))
}

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut gz = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}
