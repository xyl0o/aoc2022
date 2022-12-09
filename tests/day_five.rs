
use aoc2022::day_five::{part_one,part_two};
use indoc::indoc;

#[test]
fn part_one_examples() {
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

#[test]
fn part_two_examples() {
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
    assert_eq!(part_two(puzzle_input), "MCD");
}
