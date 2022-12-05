use std::env;
use std::fs;


fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("Unable to read file");

    most_cal(&content);
    most_top3_cal(&content);
}

fn most_cal(content: &String) {
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


fn most_top3_cal(content: &String) {
    let lines = content.lines();

    let mut cal_stack = [0, 0, 0, 0];

    for line in lines {
        if line == "" {
            cal_stack.sort();
            cal_stack[0] = 0;
            continue;
        }

        let result: u32 = line.parse().expect("Couldn't parse line");
        cal_stack[0] += result;
    }

    let top_three = &cal_stack[1..4];

    println!("Top three cal: {:?}", top_three);
    println!("Sum of top three cal: {:?}", top_three.iter().sum::<u32>());
}
