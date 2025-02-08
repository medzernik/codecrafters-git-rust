use std::io::Read;

use flate2::bufread::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

#[derive(Debug)]
pub enum FileType {
    Directory = 40000,
    ExecutableFile = 100755,
    RegularFile = 100644,
    SymbolicLink = 120000,
}

pub fn get_contents(contents: &[u8]) -> String {
    format!(
        "tree {}\0{}",
        contents.len(),
        String::from_utf8_lossy(contents)
    )
}

pub fn encode_writer(bytes: Vec<u8>) -> anyhow::Result<Vec<u8>> {
    let mut z = ZlibEncoder::new(bytes.as_slice(), Compression::best());
    let mut buffer = vec![];
    z.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn decode_reader(bytes: Vec<u8>) -> anyhow::Result<String> {
    let mut gz = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;
    Ok(s)
}
