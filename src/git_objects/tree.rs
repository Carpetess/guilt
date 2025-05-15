#![allow(dead_code)]
use flate2::read::ZlibDecoder;

use std::{fs::File, io::Read};

const PATH_TO_INDEX: &str = ".git/index";
const PATH_TO_OBJECTS: &str = ".git/objects";

pub struct Tree {
    content: Vec<u8>
}

impl Tree {
    pub fn load_tree(tree_hash: &str) -> Self {

        let tree_file: File = File::open(format!("{}/{}/{}", PATH_TO_OBJECTS, &tree_hash[..2], &tree_hash[2..])).expect("Error while opening tree file");
        let mut decoder = ZlibDecoder::new(tree_file);
        let mut content = String::new();

        decoder.read_to_string(&mut content).expect("Error while reading the contents of the tree file");

        let content: Vec<&str> = content.split("\0").collect();

        Self { content: content[1].as_bytes().to_vec()}
    }
}


