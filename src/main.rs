use files::Blob;
use flate2::bufread::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use git_tree::Tree;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::io::Read;
use std::io::Write;
use std::path::Path;

mod files;
mod git_tree;

pub trait GitObjectOperations {
    fn new(path: &str) -> Self;
    fn get_file_contents(&self) -> String;
    fn get_bytes(&self) -> Vec<u8>;
    fn encode_writer(&self) -> anyhow::Result<Vec<u8>> {
        let bytes = self.get_bytes();
        let mut z = ZlibEncoder::new(bytes.as_slice(), Compression::best());
        let mut buffer = vec![];
        z.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    fn compute_hash(&self) -> anyhow::Result<String>;
    fn get_hash_path_sha(hash: &str) -> anyhow::Result<(&str, &str)> {
        if hash.len() != 40 {
            return Err(anyhow::Error::msg(format!(
                "invalid sha length: {} instead of 40\nhash: {hash}",
                hash.len()
            )));
        }
        Ok(hash.split_at(2))
    }
    fn decode_reader_bytes(data: &[u8]) -> Vec<u8> {
        let mut gz = ZlibDecoder::new(data);
        let mut buffer = vec![];
        gz.read_to_end(&mut buffer).unwrap();
        buffer
    }
    fn decode_reader_string(data: &[u8]) -> String {
        let mut gz = ZlibDecoder::new(data);
        let mut buffer = String::default();
        gz.read_to_string(&mut buffer).unwrap();
        buffer
    }
}

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
            let (dir, file) = Blob::get_hash_path_sha(&args[3]).unwrap();
            let file = Blob::new(&format!(".git/objects/{dir}/{file}"));

            let decompress = Blob::decode_reader_string(&file.contents);
            let test: Vec<&str> = decompress.split("\0").collect();
            print!("{}", test[1])
        }
        "hash-object" => match args.len() {
            2.. => {
                let file_path = args[3].as_str();

                // Get the file contents
                let blob = Blob::new(file_path);

                // Calculate the Hash
                let hash = blob.compute_hash().unwrap();

                // Print the hash to the terminal as per the spec
                println!("{hash}");

                // Get the directory and filename from the hash
                let (dir, file_name) = Blob::get_hash_path_sha(&hash).unwrap();
                // Compress the data using zlib
                let contents = blob.encode_writer().expect("invalid writing!!");
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
        "ls-tree" => match args.len() {
            2.. => {
                let file_path = args[3].as_str();
                // Get the file contents
                let tree = Tree::new(file_path);
                if let Some(mode_type) = args.get(2) {
                    match mode_type.as_str() {
                        "--name-only" => {
                            println!("{}", tree.get_file_contents());
                        }
                        _ => panic!("invalid argument"),
                    }
                }
            }
            _ => panic!("incorrect command arguments"),
        },
        "write-tree" => {}

        _ => panic!("unknown command: {}", args[1]),
    }
}
