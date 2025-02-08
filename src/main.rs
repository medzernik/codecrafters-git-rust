use files::Blob;
use flate2::bufread::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
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
    fn get_bytes(&self) -> &[u8];
    //TODO: make this &self instead of associated?
    fn encode_writer(&self) -> anyhow::Result<Vec<u8>> {
        let mut z = ZlibEncoder::new(self.get_bytes(), Compression::best());
        let mut buffer = vec![];
        z.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
    fn decode_reader(&self) -> anyhow::Result<String> {
        let mut gz = ZlibDecoder::new(self.get_bytes());
        let mut s = String::new();
        gz.read_to_string(&mut s)?;
        Ok(s)
    }
    fn compute_hash(&self) -> anyhow::Result<String>;
    fn get_hash_path_sha(hash: &str) -> anyhow::Result<(&str, &str)>;
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

            let decompress = file.decode_reader().unwrap();

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
        "ls-tree" => {}

        _ => panic!("unknown command: {}", args[1]),
    }
}
