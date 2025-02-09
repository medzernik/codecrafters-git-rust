use std::{ascii::AsciiExt, io::BufRead, path::Path};

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

#[derive(Debug, Default)]
pub struct Tree {
    contents: Vec<Entry>,
    size: u32,
}

const BYTE_HASH_SIZE_SPLIT: usize = 21;

impl GitObjectOperations for Tree {
    fn new(hash: &str) -> Self {
        let mut tree = Tree::default();
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
                // lines.iter().for_each(|x| println!("{:#?}", *x));
            }
        }

        // Parse each line
        for item in lines.into_iter() {
            let item: Vec<Vec<u8>> = item.split(|x| *x == 0).map(|x| x.to_vec()).collect();
            let (file_perm, file_name): (Vec<u8>, Vec<u8>) =
                item[0].iter().partition(|x| x.is_ascii_whitespace());
            let file_perm = String::from_utf8_lossy(&file_perm).to_string();
            let file_name = String::from_utf8_lossy(&file_name).to_string();
            println!(" file names and perms:  {file_name},{file_perm}");
        }
        //TODO: why is the space missing between file perm and file name??
        todo!();

        tree
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
