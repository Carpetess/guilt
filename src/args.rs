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
    MakeTree(MakeTreeCommand),
    /// Write out the tree object in the index file into a commit object.
    WriteTree
}

#[derive(Debug, Args)]
pub struct MakeTreeCommand{
    // Do not check if the arguments are valid objects.
    #[arg(long)]
    pub missing: bool,
    // Make several trees in one input, by separating each tree with two new lines: \n\n
    #[arg(long)]
    pub batch: bool,
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



