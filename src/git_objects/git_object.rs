#![allow(unused)]
use flate2::{
    read::{ZlibDecoder, ZlibEncoder},
    Compression,
};

use std::{
    fs::{create_dir_all, File},
    io::{BufReader, Read, Write},
};

pub trait GitObject {
    fn pretty_print(&self);
    fn hash_object(&self) -> String;
    fn raw_content(&self) -> Vec<u8>;
    fn store_oject(&self) {
        let object_content: Vec<u8> = self.raw_content();
        let mut encoder = ZlibEncoder::new(
            BufReader::new(object_content.as_slice()),
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
}
