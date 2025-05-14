#![allow(unused_imports)]
use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct GuiltArgs{
    
    #[clap(subcommand)]
    pub main_command: Command

}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initiate the git repository.
    Init,
    /// Write the contents of an hashed file.
    CatFile(CatFileCommand),
    /// Hash an object.
    HashObject(HashObjectCommand),
    /// Update the index by adding a file to it.
    UpdateIndex(UpdateIndexCommand),
    /// Write out the tree object in the index file into a commit object.
    WriteTree
}

#[derive(Debug, Args)]
pub struct UpdateIndexCommand {
    /// Adds a file that is not yet in your staging area, by hashing it as a blob and then adding
    /// it to the index
    #[arg(long)]
    pub add: String,
    /// Should be formated as [MODE] \ [SHA-1] [FILENAME].
    #[arg(long)]
    pub cacheinfo: String
}

#[derive(Debug, Args)]
pub struct CatFileCommand {
    /// Print the contents of the file as specified by the file type.
    #[arg(short)]
    pub pretty_print: bool,
    /// Hash of the serialized object.
    pub object_hash: String,
}

#[derive(Debug, Args)]
pub struct HashObjectCommand {
    /// Write the object to the git database.
    #[arg(short)]
    pub write: bool,
    /// Path to the object you want to serialize.
    pub path_to_object: String
}



