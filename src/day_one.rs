
pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!(
        "Elf with most cal: {:?}",
        part_one_solution
    );

    let part_two_solution = part_two(input);
    println!(
        "Sum of top three cal: {:?}",
        part_two_solution
    );
}

pub fn part_one(input: &str) -> u32 {
    let lines = input.lines();

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

    most_cal
}


pub fn part_two(input: &str) -> u32 {
    let lines = input.lines();

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

    cal_stack.sort();
    cal_stack[0] = 0;

    let top_three = &cal_stack[1..4];
    let top_three_sum = top_three.iter().sum::<u32>();

    top_three_sum
}
