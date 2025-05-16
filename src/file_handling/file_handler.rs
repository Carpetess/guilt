#![allow(dead_code)]

use std::{
    fs::{create_dir_all, File},
    io::{BufReader, Read, Write},
};

use flate2::{read::{ZlibDecoder, ZlibEncoder}, Compression};
use sha1::{Digest, Sha1};

const GIT_OBJECTS_DIR: &str = ".git/objects/";

pub fn get_hash(formatted_content: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(formatted_content);
    let hashed_object_content = format!("{:x}", hasher.finalize());
    hashed_object_content
}

pub fn store_oject(formatted_content: String, object_hash: &str) {
    let mut encoder = ZlibEncoder::new(
        BufReader::new(formatted_content.as_bytes()),
        Compression::default(),
    );
    let mut buffer = Vec::new();
    encoder
        .read_to_end(&mut buffer)
        .expect("Error encoding object");

    create_dir_all(format!("{}/{}", GIT_OBJECTS_DIR, &object_hash[..2]))
        .expect("Error while trying to initialize the blob's directory");
    let mut f: File = File::create(format!(
        "{}/{}/{}",
        GIT_OBJECTS_DIR,
        &object_hash[..2],
        &object_hash[2..]
    ))
    .expect("Error creating the object File");
    
    f.write_all(buffer.as_slice())
        .expect("Error writting to the object file");
}

pub fn read_encrypted_file(file_hash: &str) -> Vec<u8> {
        let f = File::open(format!(
            "{}/{}/{}",
            GIT_OBJECTS_DIR,
            &file_hash[..2],
            &file_hash[2..]
        ))
        .expect("Error opening Blob");

        let mut decoder = ZlibDecoder::new(f);
        let mut content = String::new();
        decoder
            .read_to_string(&mut content)
            .expect("Error decoding the contents of the blob");

        content.as_bytes().to_vec()
}

pub fn parse_content(file_content: Vec<u8>) -> String {
    let content_string = String::from_utf8(file_content).unwrap();
    let content_split: Vec<&str> = content_string.split("\0").collect();
    content_split[1].to_owned()
    
}
