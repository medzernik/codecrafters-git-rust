use flate2::bufread::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;
use std::{
    io::{BufRead, Read},
    path::Path,
};

use crate::GitObjectOperations;

#[derive(Debug)]
pub enum FileType {
    Directory = 40000,
    ExecutableFile = 100755,
    RegularFile = 100644,
    SymbolicLink = 120000,
}

#[derive(Debug)]
pub struct Entry {
    filetype: FileType,
    sha: Vec<u8>,
    name: String,
}

#[derive(Debug)]
pub struct Tree {
    contents: Vec<Entry>,
}

impl GitObjectOperations for Tree {
    fn new(hash: &str) -> Self {
        let (dir, file) = Tree::get_hash_path_sha(hash).expect("cannot parse hash file for Tree");
        let file = std::fs::read(Path::new(&format!(".git/objects/{dir}/{file}")))
            .expect("cannot open fs file Tree");
        let data = Tree::decode_reader(&file);
        let split: Vec<String> = data
            .split(|x| *x == 0)
            .map(|slice| String::from_utf8_lossy(slice).to_string())
            .collect();
        println! {"data: {split:#?}"};

        //header
        let header = split
            .first()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        if header[0] != "tree" || header.len() != 2 {
            panic!("this is not a tree file!");
        }
        println!("size of the file is: {}", header[1]);

        todo!()
    }

    fn get_file_contents(&self) -> String {
        todo!()
    }

    fn get_bytes(&self) -> &[u8] {
        todo!()
    }

    fn compute_hash(&self) -> anyhow::Result<String> {
        todo!()
    }
}
