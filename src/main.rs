mod git_objects;
use flate2::{
    read::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use git_objects::{blob::Blob, git_object::GitObject};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, create_dir, File},
    io::{BufReader, Read, Write},
};

use clap::{Arg, ArgAction, ArgMatches, Command};

fn main() {
    let matches = Command::new("Guilt")
        .version("0.1")
        .about("Git made by a bad rust programmer")
        .subcommand(
            Command::new("init")
                .about("Initiates the git repository")
        )
        .subcommand(
            Command::new("cat-file")
                .about("Prints the contents of a blob")
                .arg(Arg::new("pretty-print")
                    .short('p')
                    .action(ArgAction::SetTrue)
                    .help("pretty-print the contents of <object> based on its type (does nothing right now)"))
                .arg(Arg::new("type")
                    .short('t')
                    .action(ArgAction::SetTrue)
                    .help("if this flag is used, cat-file will tell you the type of this object")
                )
                .arg(Arg::new("object-path")
                    .help("The path to the <object> you want to read").required(true))

        )
        .subcommand(
            Command::new("hash-object")
                .about("Used to compute the SHA of an <object>")
                .arg(Arg::new("write")
                    .short('w')
                    .action(ArgAction::SetTrue)
                    .help("Actually write the object into the object database."))
                .arg(Arg::new("object-name")
                    .required(true)
                    .help("The name of the file corresponding to the <object>"))
        ).subcommand(
            Command::new("update-index")
            .about("Used to update the state of the staging area, also known as the index file")
                .arg(Arg::new("file-information")
                    .help("Information regarding the file")
                )
                .arg(Arg::new("add")
                    .long("add")
                    .num_args(0..=1)
                    .default_missing_value("CACHEINFO")
                    .help("Adds the object if its not yet in your staging area")
                ).arg(Arg::new("cacheinfo")
                    .long("cacheinfo")
                    .trailing_var_arg(true)
                    .num_args(4)
                    .requires("add")
                    .help(r"Used to add an object thats already in the .git/objects folder --cacheinfo [FILE PERMISSON] \ [SHA-1] [FILE NAME]")
                )
        )
        .get_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            init();
        }
        Some(("cat-file", sub_matches)) => {
            cat_file(sub_matches);
        }
        Some(("hash-object", sub_matches)) => {
            hash_object(sub_matches);
        }
        Some(("update-index", sub_matches)) => {
            update_index(sub_matches);
        }

        _ => (),
    }

    eprintln!("logs from your program will appear here!");
}
/// Initiates the git repository creating all the required directories.
fn init() {
    match fs::create_dir(".git") {
        Ok(_) => (),
        Err(_) => {
            println!("A git repository has already been created");
            println!("Use guilt help for more information about guilt");
            return;
        }
    }
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}
/// Reads the contents of the file corresponding to hash given to the program, decrypts it using
/// Zlib and then prints the contents of the object.
fn cat_file(sub_matches: &ArgMatches) {
    if let Some(object_path) = sub_matches.get_one::<String>("object-path") {
        let kind = kind(object_path);
        let object: Box<dyn GitObject> = match kind.as_str() {
            "blob" => Box::new(Blob::load_object(object_path)),
            "tree" => todo!(),
            "commit" => todo!(),
            _ => panic!("Object type is not valid"),
        };
        if sub_matches.get_flag("type") {
            println!("{}", kind);
        }
        if sub_matches.get_flag("pretty-print") {
            object.pretty_print();
        }
    }
}
/// Reads the contents of the file to hash, places it in front of an header with information
/// regarding the contents of the file, hashes them using SHA1 and prints them to the terminal.
///
/// # Contents of the result file (decrypted)
///
/// (type) (size)\0(content)
///
/// # Exmaple
///
/// blob 12\0Hello World!
///
/// If the w flag is used, then contents of the file are encrypted using Zlib and stored inside the
/// objects folder in a folder corresponding to the objects SHA1 hash calculated previously
fn hash_object(sub_matches: &ArgMatches) {
    if let Some(object_name) = sub_matches.get_one::<String>("object-name") {
        let blob: Blob = Blob::new(object_name);
        println!("{}", blob.hash_object());
        if sub_matches.get_flag("write") {
            blob.store_oject();
        }
    }
}

fn update_index(sub_matches: &ArgMatches) {
    if let Some(file_info) = sub_matches.get_one::<String>("add") {
            println!("{}", file_info); 
        if let Some(cache_info) = sub_matches.get_many::<String>("cacheinfo") {
            let tmp : Vec<_> = cache_info.collect();
            println!("{:?}", tmp )
        }
    }
}



fn kind(file_hash: &str) -> String {
    let file_name = &file_hash[2..];
    let file_dir = &file_hash[..2];
    let file = File::open(format!(".git/objects/{}/{}", file_dir, file_name))
        .expect("Failed to open git object");

    let mut decoder = ZlibDecoder::new(file);
    let mut buffer = String::new();

    decoder
        .read_to_string(&mut buffer)
        .expect("Error while decoding the file");
    let split: Vec<&str> = buffer.split(" ").collect();
    split[0].to_string()
}
