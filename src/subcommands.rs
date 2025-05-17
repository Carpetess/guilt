use regex::Regex;
use std::{
    fs,
    io::{self, BufRead},
    process,
};

use crate::{
    args::{CatFileCommand, HashObjectCommand, MakeTreeCommand},
    file_handling::file_handler::{self},
    git_objects::{blob::Blob, tree::Tree},
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
        let content = file_handler::parse_content(file_content);
        println!("{}", content);
    } else {
        println!("{}", std::str::from_utf8(file_content.as_slice()).unwrap());
    }
}

pub fn hash_object(hash_object_args: HashObjectCommand) {
    let blob = Blob::new(&hash_object_args.path_to_object);

    let formatted_content = blob.get_formated_content();

    let hash = file_handler::get_hash(&formatted_content);

    if hash_object_args.write {
        file_handler::store_oject(formatted_content, &hash);
    }
    println!("{}", hash);
}

pub fn mk_tree(make_tree_args: MakeTreeCommand) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut current_tree = String::new();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() && !current_tree.is_empty(){
            let tree: Tree = Tree::new(&current_tree);
            let tree_formatted_content = tree.get_formatted_content();
            let tree_hash = file_handler::get_hash(&tree_formatted_content);
            file_handler::store_oject(tree_formatted_content, &tree_hash);
            current_tree = String::new();
            continue
        } 
        if !match_tree_structure(&line) {
            println!("Wrong branch format.");
            println!("Correct format: [100644|100755|120000|040000] [blob|tree] [GitObjectHash] <tab> [FileName]");
            process::exit(1)
        }
        if !make_tree_args.missing || is_real_object(&line) {
            println!("This object does not exist. If you want to ignore this condition use the --missing flag");
            process::exit(1)
        }
        current_tree.push_str(&line);
    }
}

fn match_tree_structure(branch_line: &str) -> bool {
    let re = Regex::new(r"^(100644|100755|120000|040000) (blob|tree) [0-9a-f]{40}\t.+$").unwrap();
    re.is_match(branch_line)
}

fn is_real_object(branch_line: &str) -> bool {
    false
}

pub fn write_tree() {}
