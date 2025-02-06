use super::git_object::GitObject;
use flate2::{
    read::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use sha1::{Digest, Sha1};
use std::{
    fmt::format,
    fs::{self, create_dir, create_dir_all, File},
    io::{BufReader, Read, Write},
};

#[derive(Debug)]
pub struct Blob {
    kind: &'static str,
    content: Vec<u8>,
}

impl Blob {
    pub fn new(path_to_file: &String) -> Self {
        let mut f: File = File::open(path_to_file).expect("Failed to open file.");
        let mut buffer: Vec<u8> = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        Self {
            kind: "blob",
            content: buffer,
        }
    }
    fn format_blob(&self) -> String {
        format!(
            "blob {}\0{}",
            self.content.len(),
            String::from_utf8(self.content.clone()).unwrap()
        )
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
            kind: "blob",
            content: content[1].as_bytes().to_vec(),
        }
    }
}

impl GitObject for Blob {
    fn hash_object(&self) -> String {
        let mut hasher = Sha1::new();
        let object_content = self.format_blob();
        hasher.update(&object_content);
        let hashed_object_content = format!("{:x}", hasher.finalize());
        hashed_object_content
    }

    fn pretty_print(&self) {
        println! {"{}",  String::from_utf8(self.content.clone()).unwrap()
        }
    }

    fn store_oject(&self) {
        let object_content = self.format_blob();
        let mut encoder = ZlibEncoder::new(
            BufReader::new(object_content.as_bytes()),
            Compression::default(),
        );
        let mut buffer = Vec::new();
        encoder
            .read_to_end(&mut buffer)
            .expect("Error encoding blob");

        let object_hash = self.hash_object();
        create_dir_all(format!(".git/objects/{}", &object_hash[..2]))
            .expect("Error while trying to initialize the blob's directory");
        let mut f: File = File::create(format!(
            ".git/objects/{}/{}",
            &object_hash[..2],
            &object_hash[2..]
        ))
        .expect("Error creating the blob File");
        f.write_all(buffer.as_slice())
            .expect("Error writting to the blob file");
    }

    fn raw_content(&self) -> Vec<u8> {
        self.format_blob().as_bytes().to_vec()
    }
}
