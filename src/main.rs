use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Unable to read file");

    println!("With text:\n{contents}");
}
