#![allow(dead_code)]


use crate::file_handling::file_handler;

const PATH_TO_INDEX: &str = ".git/index";
const PATH_TO_OBJECTS: &str = ".git/objects";

pub struct Tree {
    content: String
}

impl Tree {
    pub fn new(tree_content: &str) -> Self {
        Self { content: tree_content.to_owned()}
    }

    pub fn load_tree(tree_hash: &str) -> Self {

        let content: Vec<u8> = file_handler::read_encrypted_file(tree_hash);

        let content = String::from_utf8(content).unwrap();

        Self { content }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_formatted_content(&self) -> String {
         format!(
            "tree {}\0{}",
            self.content.len(),
            &self.content
        )
    }
        

}


