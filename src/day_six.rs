use itertools::Itertools;
use std::collections::HashSet;

pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!(
        "Part one: {:?}",
        part_one_solution
    );

    let part_two_solution = part_two(input);
    println!(
        "Part two: {:?}",
        part_two_solution
    );
}

pub fn part_one(input: &str) -> String {
    todo!();
}

pub fn part_two(input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    // use indoc::indoc;

    #[test]
    fn test_part_one() {
        todo!();
        // let puzzle_input = indoc! {""};
        // assert_eq!(part_one(puzzle_input), "");
    }

    #[test]
    fn test_packet_start_pos_example() {
        let puzzle_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(packet_start_pos(puzzle_input), Some(7));

        let puzzle_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(packet_start_pos(puzzle_input), Some(5));

        let puzzle_input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(packet_start_pos(puzzle_input), Some(6));

        let puzzle_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(packet_start_pos(puzzle_input), Some(10));

        let puzzle_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(packet_start_pos(puzzle_input), Some(11));
    }

    #[test]
    fn test_part_two() {
        todo!();
        // let puzzle_input = indoc! {""};
        // assert_eq!(part_one(puzzle_input), "");
    }
}
