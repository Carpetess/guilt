use flate2::{
    read::{ZlibDecoder, ZlibEncoder},
    Compression,
};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, create_dir, File},
    io::{BufReader, Read, Write},
};

use clap::{Arg, ArgAction, Command};

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
        )
        .get_matches();
    match matches.subcommand() {
        Some(("init", _)) => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        Some(("cat-file", sub_matches)) => {
            if sub_matches.get_flag("pretty-print") {
                println!("Pretty-print selected")
            }
            if let Some(object_path) = sub_matches.get_one::<String>("object-path") {
                let file = fs::read(format!(
                    ".git/objects/{}/{}",
                    &object_path[..2],
                    &object_path[2..]
                ))
                .unwrap();
                let mut z = ZlibDecoder::new(&file[..]);
                let mut s = String::new();
                z.read_to_string(&mut s).unwrap();
                let split_s: Vec<&str> = s.split('\0').collect();
                println!("{}", split_s[1]);
            }
        }
        Some(("hash-object", sub_matches)) => {
            if let Some(object_name) = sub_matches.get_one::<String>("object-name") {
                let mut f = File::open(object_name).unwrap();
                let mut buffer = String::new();
                f.read_to_string(&mut buffer).unwrap();
                let blob = format!("blob {}\0{}", buffer.len(), buffer);

                let mut hasher = Sha1::new();
                hasher.update(&blob);
                let hashed_blob: String = format!("{:x}", hasher.finalize());
                println!("{}", &hashed_blob[..]);

                if sub_matches.get_flag("write") {
                    let mut encoder =
                        ZlibEncoder::new(BufReader::new(blob.as_bytes()), Compression::default());
                    let mut buffer = Vec::new();
                    encoder.read_to_end(&mut buffer).unwrap();

                    let file_dir = format!(".git/objects/{}", &hashed_blob[..2]);
                    let file_path = format!("{}/{}", &file_dir, &hashed_blob[2..]);

                    if !file_exists(&file_dir) {
                        create_dir(format!(".git/objects/{}", &hashed_blob[..2])).unwrap();
                    }
                    if !file_exists(&file_path) {
                        let mut f = File::create(file_path).unwrap();
                        f.write_all(buffer.as_slice()).unwrap();
                    }
                }
            }
        }

        _ => (),
    }

    eprintln!("logs from your program will appear here!");
}

pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

