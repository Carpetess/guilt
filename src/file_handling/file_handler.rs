#![allow(dead_code)]

use std::{
    fs::{create_dir_all, File},
    io::{BufReader, Read, Write},
};

use flate2::{bufread::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

const GIT_OBJECTS_DIR: &str = ".git/objects/";

fn hash_object(formatted_content: &str) -> String {
    let mut hasher = Sha1::new();

    hasher.update(formatted_content);
    let hashed_object_content = format!("{:x}", hasher.finalize());
    hashed_object_content
}

fn store_oject(raw_content: &Vec<u8>, object_hash: &str) {
    let mut encoder = ZlibEncoder::new(
        BufReader::new(raw_content.as_slice()),
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
