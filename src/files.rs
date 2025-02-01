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
    fn get_contents(contents: &[u8]) -> String {
        format!(
            "blob {}\0{}",
            contents.len(),
            String::from_utf8_lossy(contents)
        )
    }
}
