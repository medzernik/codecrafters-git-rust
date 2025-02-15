use std::path::Path;

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

    fn get_bytes(&self) -> Vec<u8> {
        format!(
            "blob {}\0{}",
            self.contents.len(),
            String::from_utf8_lossy(&self.contents)
        )
        .into_bytes()
    }

    fn compute_hash(&self) -> anyhow::Result<String> {
        let mut hash = sha1_smol::Sha1::new();
        let contents = self.get_file_contents();
        hash.update(contents.as_bytes());
        Ok(hash.digest().to_string())
    }
}
