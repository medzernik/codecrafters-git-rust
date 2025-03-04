use std::{
    fmt::Display,
    path::{Path, PathBuf},
    str::FromStr,
};

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

#[derive(Debug)]
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

#[derive(Default, Debug)]
pub struct Tree {
    header: Vec<Vec<u8>>,
    contents: Vec<Entry>,
    size: u32,
    hash: String,
}

impl Tree {
    fn calculate_data_hash(data: &[u8]) -> anyhow::Result<String> {
        let mut hash = sha1_smol::Sha1::new();
        hash.update(data);
        Ok(hash.digest().to_string())
    }

    fn parse_header(&mut self, data: &[u8]) -> anyhow::Result<()> {
        self.header = data.splitn(2, |x| *x == 0).map(|x| x.to_vec()).collect();

        // Get and check the header
        let parsed_header = String::from_utf8(self.header[0].clone()).unwrap();
        let parsed_header: Vec<String> = parsed_header
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();

        if parsed_header.len() != 2 {
            return Err(anyhow::anyhow!(
                "invalid header length, cannot split filetype from name",
            ));
        }

        if *parsed_header.first().unwrap() != "tree" {
            return Err(anyhow::anyhow!(
                "invalid tree file, header does not match tree"
            ));
        }
        // Set the tree size
        self.size = u32::from_str_radix(&parsed_header[1], 10).unwrap();

        Ok(())
    }

    fn parse_body(&mut self) -> anyhow::Result<()> {
        // Split body into lines
        let mut min = 0;
        for (i, char) in self.header[1].iter().enumerate() {
            if *char == 0 {
                let line = &self.header[1][min..=i + HASH_SIZE];
                let line: Vec<Vec<u8>> = line.split(|x| *x == 0).map(|x| x.to_vec()).collect();

                // This is a problem since paths dont have to be UTF8.
                let file_info = String::from_utf8_lossy(&line[0]).to_string();
                let (mode, name) = file_info
                    .split_once(' ')
                    .expect("cannot have less than 2 items");
                let mode: FileType = mode.trim().try_into()?;

                self.contents
                    .push(Entry::new(mode, name.to_string(), line[1].clone()));

                min = i + HASH_SIZE + 1;
            }
        }
        Ok(())
    }

    fn parse_directory(&mut self, path: PathBuf) -> Option<Entry> {
        for entry in std::fs::read_dir(&path).unwrap() {
            let entry = entry.unwrap();
            if let Ok(file_metadata) = entry.metadata() {
                if file_metadata.file_type().is_dir() {
                    print!("-\t",);
                    self.parse_directory(entry.path());
                    return Some(Entry {
                        filetype: FileType::Directory,
                        name: entry.file_name().into_string().unwrap(),
                        sha: vec![],
                    });
                } else {
                    // println!("FILE PATH: {}", path.to_str().unwrap());
                    // println!("FILE INFO: {}", file_metadata.is_file());
                    let file = match std::fs::read(&path) {
                        Ok(val) => {
                            println!("Filename: {}", entry.file_name().to_str().unwrap());
                            val
                        }
                        Err(_) => {
                            println!(
                                "cannot do stuff {}, {}",
                                entry.file_name().to_str().unwrap(),
                                file_metadata.is_dir()
                            );
                            continue;
                        }
                    };
                    return Some(Entry {
                        filetype: FileType::RegularFile,
                        name: entry.file_name().into_string().unwrap(),
                        sha: Tree::calculate_data_hash(&file).unwrap().into(),
                    });
                }
            }
        }
        None
    }
}

impl GitObjectOperations for Tree {
    fn new_read(hash: &str) -> Self {
        let mut tree = Tree::default();
        let (dir, file) = Tree::get_hash_path_sha(hash).expect("cannot parse hash file for Tree");
        let file = std::fs::read(Path::new(&format!(".git/objects/{dir}/{file}")))
            .expect("cannot open fs file Tree");
        let data = Tree::decode_reader_bytes(&file);

        // Parse the header
        tree.parse_header(&data)
            .expect("failed to parse the header");

        // Parse the body
        tree.parse_body().expect("failed to parse the body");

        tree.hash = tree.compute_hash().expect("cannot compute hash");

        tree
    }

    fn new_create() -> Self {
        let mut tree = Tree::default();
        if let Some(item) = tree.parse_directory(PathBuf::from_str(".").unwrap()) {
            tree.contents.push(item);
        }

        tree
    }

    fn get_file_contents(&self) -> String {
        let mut output: Vec<&str> = self.contents.iter().map(|x| x.get_name().trim()).collect();
        // We need to sort according to the spec
        output.sort();

        let mut result = String::default();
        output.into_iter().for_each(|item| {
            result.push_str(&format!("{item}\n"));
        });
        result
    }

    fn get_bytes(&self) -> Vec<u8> {
        [
            "tree ".as_bytes(),
            &self.size.to_ne_bytes(),
            "\0".as_bytes(),
            self.get_file_contents().as_bytes(),
        ]
        .concat()
    }

    fn compute_hash(&self) -> anyhow::Result<String> {
        let mut hash = sha1_smol::Sha1::new();
        let contents = self.get_bytes();

        hash.update(&contents);
        Ok(hash.digest().to_string())
    }
}
