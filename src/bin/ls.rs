use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let directory = &args[1];

    for file in fs::read_dir(directory).unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}