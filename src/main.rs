use anyhow::Error;
use flate2::bufread::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use std::default::Default;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::io::Read;
use std::io::Write;
use std::path::Path;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "init" => {
            std::fs::create_dir(".git").unwrap();
            std::fs::create_dir(".git/objects").unwrap();
            std::fs::create_dir(".git/refs").unwrap();
            std::fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        "cat-file" => {
            let (dir, file) = get_hash_path_sha(&args[3]).unwrap();
            let file = std::fs::read(Path::new(&format!(".git/objects/{dir}/{file}"))).unwrap();

            let decompress = decode_reader(file).unwrap();

            let test: Vec<&str> = decompress.split("\0").collect();
            print!("{}", test[1])
        }
        "hash-object" => match args.len() {
            1 => todo!("just print"),
            2.. => {
                let file = args[3].as_str();

                // Get the file contents
                let contents = std::fs::read(Path::new(&format!("{file}"))).unwrap();
                // Add the necessary header data
                let contents = get_contents(&contents);

                // Calculate the Hash
                let hash = compute_hash(&contents).unwrap();

                // Print the hash to the terminal as per the spec
                println!("{hash}");

                // Get the directory and filename from the hash
                let (dir, file_name) = get_hash_path_sha(&hash).unwrap();
                // Compress the data using zlib
                let contents =
                    encode_writer(contents.as_bytes().to_vec()).expect("invalid writing!!");
                // Create the directory using the name from the hash
                std::fs::create_dir(Path::new(&format!(".git/objects/{dir}"))).unwrap();

                // Create the file and write the data using the compressed data
                std::fs::File::create_new(Path::new(&format!(".git/objects/{dir}/{file_name}")))
                    .expect("cannot somehow get a created file?")
                    .write_all(&contents)
                    .expect("cannot write data to file");
            }
            _ => panic!("incorrect command arguments"),
        },
        _ => panic!("unknown command: {}", args[1]),
    }
}

fn get_hash_path_sha(hash: &str) -> anyhow::Result<(&str, &str)> {
    if hash.len() != 40 {
        return Err(Error::msg(format!(
            "invalid sha length: {} instead of 40\nhash: {hash}",
            hash.len()
        )));
    }
    Ok(hash.split_at(2))
}

fn compute_hash(contents: &str) -> anyhow::Result<String> {
    let mut hash = sha1_smol::Sha1::new();
    hash.update(contents.as_bytes());
    Ok(hash.digest().to_string())
}

fn get_contents(contents: &[u8]) -> String {
    format!(
        "blob {}\0{}",
        contents.len(),
        String::from_utf8_lossy(contents)
    )
}

fn encode_writer(bytes: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let mut z = ZlibEncoder::new(bytes.as_slice(), Compression::best());
    let mut buffer = vec![];
    z.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn decode_reader(bytes: Vec<u8>) -> anyhow::Result<String> {
    let mut gz = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}
