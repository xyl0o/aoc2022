use indoc::indoc;

pub fn day_four(input: String) {
    todo!()
}

pub fn day_four_part_one(input: &str) -> u32 {
    todo!()
}

fn fully_contained(assignment: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests {
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
}
