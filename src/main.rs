use std::env;
use std::fs;

mod day_one;
mod day_two;
mod day_three;
mod day_four;
mod day_five;
mod day_six;
mod day_seven;
mod day_eight;
mod day_nine;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let day : u32 = args[1].parse().expect("Not a valid day");
            let file_path = format!("./input/day{}.txt", day);
            single_day(day, file_path.as_ref());
        },
        3 => {
            let day : u32 = args[1].parse().expect("Not a valid day");
            let file_path = &args[2];
            single_day(day, file_path);
        },
        _ => panic!("Unsupported number of arguments"),
    }
}

fn single_day(day: u32, file_path: &str) {
    let input = fs::read_to_string(file_path)
        .expect("Unable to read file");

    match day {
        1 => day_one::both(&input),
        2 => day_two::both(&input),
        3 => day_three::both(&input),
        4 => day_four::both(&input),
        5 => day_five::both(&input),
        6 => day_six::both(&input),
        7 => day_seven::both(&input),
        8 => day_eight::both(&input),
        9 => day_nine::both(&input),
        _ => println!("Unknown day"),
    }
}
