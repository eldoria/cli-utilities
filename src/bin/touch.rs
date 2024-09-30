use std::env;
use std::fs::File;
use std::io::Write;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = &args[1];
    let content = &args[2];

    let mut file = File::create(file)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}