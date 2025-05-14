#![allow(dead_code)]
use flate2::{
    read::{ZlibEncoder},
    Compression,
};
use sha1::{Digest, Sha1};

use std::{
    fs::{create_dir_all, File},
    io::{BufReader, Read, Write},
};


pub trait GitObject {
    fn pretty_print(&self);

    fn format_object(&self) -> String;

    fn raw_content(&self) -> Vec<u8> {
        self.format_object().as_bytes().to_vec()
    }
    fn hash_object(&self) -> String {
        let mut hasher = Sha1::new();
        let object_content = self.format_object();
        hasher.update(&object_content);
        let hashed_object_content = format!("{:x}", hasher.finalize());
        hashed_object_content
    }

    fn store_oject(&self) {
        let object_content: Vec<u8> = self.raw_content();
        let mut encoder = ZlibEncoder::new(
            BufReader::new(object_content.as_slice()),
            Compression::default(),
        );
        let mut buffer = Vec::new();
        encoder
            .read_to_end(&mut buffer)
            .expect("Error encoding object");

        let object_hash: String = self.hash_object();
        create_dir_all(format!(".git/objects/{}", &object_hash[..2]))
            .expect("Error while trying to initialize the blob's directory");
        let mut f: File = File::create(format!(
            ".git/objects/{}/{}",
            &object_hash[..2],
            &object_hash[2..]
        ))
        .expect("Error creating the object File");
        f.write_all(buffer.as_slice())
            .expect("Error writting to the object file");
    }
}
