#![allow(unused_variables)]
mod args;
mod file_handling;
mod git_objects;
mod subcommands;

use args::{Command, GuiltArgs};
use clap::Parser;

fn main() {
    let args: GuiltArgs = GuiltArgs::parse();
    match args.main_command {
        Command::Init => {
            subcommands::init();
        }
        Command::WriteTree => {
            subcommands::write_tree();
        }
        Command::CatFile(cat_file_args) => {
            subcommands::cat_file(cat_file_args);
        }
        Command::HashObject(hash_object_args) => {
            subcommands::hash_object(hash_object_args);
        }
        Command::UpdateIndex(update_index_args) => {
            subcommands::update_index(update_index_args);
        }
    }
}
