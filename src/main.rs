use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("Unable to read file");

    let lines = content.lines();

    let mut most_cal: u32 = 0;
    let mut most_cal_elf: u32 = 1;

    let mut curr_cal: u32 = 0;
    let mut curr_cal_elf: u32 = 1;

    for line in lines {
        if line == "" {

            if curr_cal > most_cal {
                most_cal = curr_cal;
                most_cal_elf = curr_cal_elf;
            }

            curr_cal = 0;
            curr_cal_elf += 1;

            continue;
        }

        let result: u32 = line.parse().expect("Couldn't parse line");
        curr_cal += result;
    }
    println!("Most cal: {most_cal}");
    println!("Elf with most cal: {most_cal_elf}");
}
