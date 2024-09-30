use std::env;
use std::fs;
use std::collections::BinaryHeap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let file_to_search = &args[1];
    let directory = &args[2];

    if !file_to_search.contains(".") {
        panic!("You can only search for files, '{}' is not allowed", file_to_search)
    }

    let mut heap = BinaryHeap::new();
    heap.push(directory.clone());

    while let Some(dir) = heap.pop() {
        for element in fs::read_dir(dir).unwrap() {
            let path: String = element.unwrap().path().display().to_string();
            if path.contains(file_to_search) {
                println!("{}", &path)
            }
            else {
                let metadata = fs::metadata(&path)?;
                let file_type = metadata.file_type();

                if file_type.is_dir() {
                    heap.push(path)
                }
            }
        }
    }
    Ok(())
}
