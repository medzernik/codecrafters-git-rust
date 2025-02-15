use std::{fmt::Display, path::Path};

use crate::GitObjectOperations;

const DIRECTORY: isize = 40000;
const EXECUTABLE_FILE: isize = 100755;
const REGULAR_FILE: isize = 100644;
const SYMBOLIC_LINK: isize = 120000;
const HASH_SIZE: usize = 20;

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Directory = DIRECTORY,
    ExecutableFile = EXECUTABLE_FILE,
    RegularFile = REGULAR_FILE,
    SymbolicLink = SYMBOLIC_LINK,
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as isize)
    }
}

impl TryFrom<&str> for FileType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> anyhow::Result<Self, Self::Error> {
        match isize::from_str_radix(&value, 10)? {
            DIRECTORY => Ok(Self::Directory),
            EXECUTABLE_FILE => Ok(Self::ExecutableFile),
            REGULAR_FILE => Ok(Self::RegularFile),
            SYMBOLIC_LINK => Ok(Self::SymbolicLink),
            _ => Err(anyhow::Error::msg("invalid file type header")),
        }
    }
}

pub struct Entry {
    filetype: FileType,
    name: String,
    sha: Vec<u8>,
}

impl Entry {
    pub fn new(filetype: FileType, name: String, sha: Vec<u8>) -> Self {
        Self {
            filetype,
            name,
            sha,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Default)]
pub struct Tree {
    contents: Vec<Entry>,
    size: u32,
    hash: String,
}

impl Tree {
    pub fn print_name_only(&self) {
        let mut output: Vec<&str> = self.contents.iter().map(|x| x.get_name().trim()).collect();
        output.sort();
        output.into_iter().for_each(|item| {
            print!("{item}\n");
        });
    }
}

impl GitObjectOperations for Tree {
    fn new(hash: &str) -> Self {
        let mut tree = Tree::default();
        let (dir, file) = Tree::get_hash_path_sha(hash).expect("cannot parse hash file for Tree");
        let file = std::fs::read(Path::new(&format!(".git/objects/{dir}/{file}")))
            .expect("cannot open fs file Tree");
        let data = Tree::decode_reader_bytes(&file);

        let header_body: Vec<Vec<u8>> = data.splitn(2, |x| *x == 0).map(|x| x.to_vec()).collect();

        let header = String::from_utf8(header_body[0].clone()).unwrap();

        // Get and check the header
        let header: Vec<&str> = header.split_whitespace().collect();

        if header.len() != 2 {
            panic!("invalid header length, cannot spe")
        }

        if *header.first().unwrap() != "tree" {
            panic!("invalid tree file, header does not match tree");
        }

        // Set the tree size
        tree.size = u32::from_str_radix(header[1], 10).unwrap();

        // Split body into lines
        let mut min = 0;

        for (i, char) in header_body[1].iter().enumerate() {
            if *char == 0 {
                // println!("{}", header_body[1].len());
                let line = &header_body[1][min..=i + HASH_SIZE];
                // println!(
                //     "line num: {i}:\n{line:?}\n{:?}",
                //     line.iter().map(|x| *x as char).collect::<String>()
                // );
                let line: Vec<Vec<u8>> = line.split(|x| *x == 0).map(|x| x.to_vec()).collect();
                let file_info = String::from_utf8_lossy(&line[0]).to_string();
                // println!("{file_info}");
                // println!("{}:{:?}", &line[1].len(), &line[1]);
                let (mode, name) = file_info
                    .split_once(' ')
                    .expect("cannot have less than 2 items");
                // println!("{mode},{name}");
                let mode: FileType = mode.trim().try_into().unwrap();

                tree.contents
                    .push(Entry::new(mode, name.to_string(), line[1].clone()));

                min = i + HASH_SIZE + 1;
            }
        }

        tree
    }

    fn get_file_contents(&self) -> String {
        todo!()
    }

    fn get_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn compute_hash(&self) -> anyhow::Result<String> {
        todo!()
    }
}
