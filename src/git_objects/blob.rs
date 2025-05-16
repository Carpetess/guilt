#![allow(dead_code)]
use crate::file_handling::file_handler;
use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct Blob {
    content: Vec<u8>,
}

impl Blob {
    pub fn new(path_to_file: &String) -> Self {
        let mut f: File = File::open(path_to_file).expect("Failed to open file.");
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        Self { content: buffer }
    }

    pub fn load_object(blob_hash: &str) -> Self {
        let content: Vec<u8> = file_handler::read_encrypted_file(blob_hash);
        Blob { content }
    }

    pub fn get_content(&self) -> &Vec<u8> {
        self.content.as_ref()
    }

    pub fn get_formated_content(&self) -> String {
        format!(
            "blob {}\0{}",
            self.content.len(),
            String::from_utf8(self.content.clone()).unwrap()
        )
    }
}
