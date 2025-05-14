mod args;

use args::{GuiltArgs, Command, CatFileCommand, HashObjectCommand, UpdateIndexCommand};
use clap::Parser;
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

}

fn cat_file(cat_file_args: CatFileCommand) {

}

fn hash_object(hash_object_args: HashObjectCommand) {

}

fn update_index(update_index_args: UpdateIndexCommand) {

}

fn write_tree() {

}
