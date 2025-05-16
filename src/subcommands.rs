use std::{fs, io, process, u8};

use crate::{
    args::{CatFileCommand, HashObjectCommand, UpdateIndexCommand},
    file_handling::file_handler::{self, hash_object},
};

const GIT_REPO: &str = ".git";
const OBJECTS: &str = ".git/objects";
const REFS: &str = ".git/refs";
const HEAD: &str = ".git/HEAD";

const ERROR_MSG_INIT: &str = "Failed to create the repository";

pub fn init() {
    let dir_res: io::Result<()> = fs::create_dir(GIT_REPO);
    if dir_res.is_err() {
        println!("{}", ERROR_MSG_INIT);
        process::exit(1)
    }
    fs::create_dir(OBJECTS).unwrap();
    fs::create_dir(REFS).unwrap();
    fs::write(HEAD, "ref: refs/heads/main\n").unwrap();
    println!("Git repository created!");
}

pub fn cat_file(cat_file_args: CatFileCommand) {
    let mut content_to_print: Vec<u8>;
    let file_content: Vec<u8> = file_handler::read_encrypted_file(&cat_file_args.object_hash);
    if cat_file_args.pretty_print {
        let content = file_handler::pretty_print_git_object(file_content);
        println!("{}", content);
    } else {
        println!("{}", std::str::from_utf8(file_content.as_slice()).unwrap());
    }
}

pub fn hash_object(hash_object_args: HashObjectCommand) {

}

pub fn update_index(update_index_args: UpdateIndexCommand) {}

pub fn write_tree() {}
