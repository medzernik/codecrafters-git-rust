use std::path::Path;

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
    size: u32,
}

const BYTE_HASH_SIZE_SPLIT: usize = 21;

impl GitObjectOperations for Tree {
    fn new(hash: &str) -> Self {
        let (dir, file) = Tree::get_hash_path_sha(hash).expect("cannot parse hash file for Tree");
        let file = std::fs::read(Path::new(&format!(".git/objects/{dir}/{file}")))
            .expect("cannot open fs file Tree");
        let data = Tree::decode_reader(&file);
        let split: Vec<Vec<u8>> = data.splitn(2, |x| *x == 0).map(|x| x.to_vec()).collect();

        //header
        let header = String::from_utf8_lossy(&split[0]).to_string();
        let header: Vec<&str> = header.split_whitespace().collect();

        if header[0] != "tree" || header.len() != 2 {
            panic!("this is not a tree file!");
        }
        println!("size of the file is: {}", header[1]);

        // Get each line
        let mut lines = vec![];
        let mut line = 0;

        for (i, num) in split[1].iter().enumerate() {
            if *num == 0 {
                println!("found the symbol at: {i}, {num}");
                lines.push(&split[1][line..i + BYTE_HASH_SIZE_SPLIT]);
                line = i + BYTE_HASH_SIZE_SPLIT;
                lines.iter().for_each(|x| println!("{:#?}", *x));
            }
        }

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
