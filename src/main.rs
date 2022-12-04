use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("Unable to read file");

    let mut lines = content.lines();

    for line in lines {
        println!("Line: {line}");
    }
}
