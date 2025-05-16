#![allow(dead_code)]
use flate2::read::ZlibDecoder;

use std::{fs::File, io::Read};

use crate::file_handling::{self, file_handler};

const PATH_TO_INDEX: &str = ".git/index";
const PATH_TO_OBJECTS: &str = ".git/objects";

pub struct Tree {
    content: Vec<u8>
}

impl Tree {
    pub fn load_tree(tree_hash: &str) -> Self {

        let content: Vec<u8> = file_handler::read_encrypted_file(tree_hash);

        Self { content }
    }
}


