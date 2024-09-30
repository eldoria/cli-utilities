use std::env;
use std::fs;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];

    fs::remove_file(file)?;
    Ok(())
}
