#![allow(unused_variables)]
mod args;
mod git_objects;

use args::{GuiltArgs, Command, CatFileCommand, HashObjectCommand, UpdateIndexCommand};
use std::{fs, io::Error};
use clap::{ Parser};

const GIT_REPO: &str = ".git";
const OBJECTS: &str = ".git/objects";
const REFS: &str = ".git/refs";
const HEAD: &str = ".git/HEAD";


fn main() {
    let args: GuiltArgs  = GuiltArgs::parse();
    match args.main_command {
        Command::Init => {init();}
        Command::WriteTree => {write_tree();}
        Command::CatFile(cat_file_args)=> {cat_file(cat_file_args);}
        Command::HashObject(hash_object_args) => { hash_object(hash_object_args);}
        Command::UpdateIndex(update_index_args) => { update_index(update_index_args);}
    }
}

fn init() {
    let dir_res: Result<(), Error> = fs::create_dir(GIT_REPO);
    if dir_res.is_err() {
        panic!("Error while trying to create the git repository!")
    }
    fs::create_dir(OBJECTS).unwrap();
    fs::create_dir(REFS).unwrap();
    fs::write(HEAD, "ref: refs/heads/main\n").unwrap();
    println!("Git repository created!");
}

fn cat_file(cat_file_args: CatFileCommand) {

}

fn hash_object(hash_object_args: HashObjectCommand) {

}

fn update_index(update_index_args: UpdateIndexCommand) {


}

fn write_tree() {

}
