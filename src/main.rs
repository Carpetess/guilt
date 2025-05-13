mod args;

use args::GuiltArgs;
use clap::Parser;
fn main() {
    let args: GuiltArgs  = GuiltArgs::parse();
    println!("{:?}", args);
}
