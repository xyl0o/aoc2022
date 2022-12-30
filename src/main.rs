use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let day: u32 = args[1].parse().expect("Not a valid day");
            let file_path = format!("./input/day{}.txt", day);
            single_day(day, file_path.as_ref());
        }
        3 => {
            let day: u32 = args[1].parse().expect("Not a valid day");
            let file_path = &args[2];
            single_day(day, file_path);
        }
        _ => panic!("Unsupported number of arguments"),
    }
}

fn single_day(day: u32, file_path: &str) {
    let input = fs::read_to_string(file_path).expect("Unable to read file");

    match day {
        1 => aoc2022::day_one::both(&input),
        2 => aoc2022::day_two::both(&input),
        3 => aoc2022::day_three::both(&input),
        4 => aoc2022::day_four::both(&input),
        5 => aoc2022::day_five::both(&input),
        6 => aoc2022::day_six::both(&input),
        7 => aoc2022::day_seven::both(&input),
        8 => aoc2022::day_eight::both(&input),
        9 => aoc2022::day_nine::both(&input),
        10 => aoc2022::day_ten::both(&input),
        11 => aoc2022::day_eleven::both(&input),
        12 => aoc2022::day_twelve::both(&input),
        _ => println!("Unknown day"),
    }
}
