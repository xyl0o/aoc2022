use std::env;
use std::fs;

mod day_one;
mod day_two;
mod day_three;
mod day_four;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = &args[1].parse().expect("Not a valid day");
    let file_path = &args[2];

    let input = fs::read_to_string(file_path)
        .expect("Unable to read file");

    match day {
        1 => day_one::day_one(input),
        2 => day_two::day_two(input),
        3 => day_three::day_three(input),
        4 => day_four::day_four(input),
        _ => println!("Unknown day"),
    }
}
