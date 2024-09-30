use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    let attr = fs::read_to_string(file);
    match attr {
        Ok(content) => println!("{}", content),
        Err(e) => println!("{}", e)
    }
}
