use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day_four(input: String) {
    println!(
        "Number of fully contained assignments: {:?}",
        day_four_part_one(input.as_ref())
    );

    println!(
        "Number of overlapping assignments: {:?}",
        day_four_part_two(input.as_ref())
    );
}

pub fn day_four_part_one(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        if fully_contained(line) { acc + 1 } else { acc }
    })
}

pub fn day_four_part_two(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        if overlapping(line) { acc + 1 } else { acc }
    })
}

fn construct_sets(assignment: &str) -> (HashSet<u32>, HashSet<u32>){
    let (one, two) = assignment.split_once(',').unwrap();

    let (one_start, one_end) = one.split_once('-').unwrap();
    let one_start : u32 = one_start.parse().unwrap();
    let one_end : u32 = one_end.parse().unwrap();
    let one : HashSet<u32> = HashSet::from_iter(one_start..one_end + 1);

    let (two_start, two_end) = two.split_once('-').unwrap();
    let two_start : u32 = two_start.parse().unwrap();
    let two_end : u32 = two_end.parse().unwrap();
    let two : HashSet<u32> = HashSet::from_iter(two_start..two_end + 1);

    (one, two)
}

fn fully_contained(assignment: &str) -> bool {
    let (one, two) = construct_sets(assignment);

    one.is_superset(&two) || two.is_superset(&one)
}

fn overlapping(assignment: &str) -> bool {
    let (one, two) = construct_sets(assignment);
    one.intersection(&two).count() != 0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_fully_contained_false() {
        assert_eq!(fully_contained("2-4,6-8"), false);
        assert_eq!(fully_contained("2-3,4-5"), false);
        assert_eq!(fully_contained("5-7,7-9"), false);
        assert_eq!(fully_contained("2-6,4-8"), false);
    }

    #[test]
    fn test_fully_contained_true() {
        assert_eq!(fully_contained("2-8,3-7"), true);
        assert_eq!(fully_contained("6-6,4-6"), true);
    }

    #[test]
    fn test_day_four_part_one_with_example_input() {
        let puzzle_input = indoc! {"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "};
        assert_eq!(day_four_part_one(puzzle_input), 2);
    }

    #[test]
    fn test_overlapping_false() {
        assert_eq!(overlapping("2-4,6-8"), false);
        assert_eq!(overlapping("2-3,4-5"), false);
    }

    #[test]
    fn test_overlapping_true() {
        assert_eq!(overlapping("5-7,7-9"), true);
        assert_eq!(overlapping("2-8,3-7"), true);
        assert_eq!(overlapping("6-6,4-6"), true);
        assert_eq!(overlapping("2-6,4-8"), true);
    }

    #[test]
    fn test_day_four_part_two_with_example_input() {
        let puzzle_input = indoc! {"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "};
        assert_eq!(day_four_part_two(puzzle_input), 4);
    }
}
