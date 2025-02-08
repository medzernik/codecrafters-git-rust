use std::path::Path;

use anyhow::Error;

use crate::GitObjectOperations;

#[derive(Debug)]
pub struct Blob {
    pub contents: Vec<u8>,
}

impl GitObjectOperations for Blob {
    fn new(path: &str) -> Self {
        Self {
            contents: std::fs::read(Path::new(path)).unwrap(),
        }
    }
    fn get_file_contents(&self) -> String {
        format!(
            "blob {}\0{}",
            self.contents.len(),
            String::from_utf8_lossy(&self.contents)
        )
    }

    fn get_bytes(&self) -> &[u8] {
        &self.contents.as_slice()
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

    fn compute_hash(&self) -> anyhow::Result<String> {
        let mut hash = sha1_smol::Sha1::new();
        let contents = self.get_file_contents();
        hash.update(contents.as_bytes());
        Ok(hash.digest().to_string())
    }
}
