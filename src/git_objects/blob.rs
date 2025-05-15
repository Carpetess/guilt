#![allow(dead_code)]
use flate2::read::ZlibDecoder;
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

    pub fn load_object(file_hash: &str) -> Self {
        let f = File::open(format!(
            ".git/objects/{}/{}",
            &file_hash[..2],
            &file_hash[2..]
        ))
        .expect("Error opening Blob");

        let mut decoder = ZlibDecoder::new(f);
        let mut content = String::new();
        decoder
            .read_to_string(&mut content)
            .expect("Error decoding the contents of the blob");

        let content: Vec<&str> = content.split("\0").collect();

        Blob {
            content: content[1].as_bytes().to_vec(),
        }
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
