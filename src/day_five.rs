use std::collections::HashSet;
use std::iter::FromIterator;

pub fn day_five(input: String) {
    todo!()
}

pub fn part_one(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        let puzzle_input = indoc! {"
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "};
        assert_eq!(part_one(puzzle_input), "CMZ");
    }
}
