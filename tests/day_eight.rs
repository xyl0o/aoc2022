use aoc2022::day_eight::{part_one, part_two};
use indoc::indoc;

#[test]
fn part_one_examples() {
    let puzzle_input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};
    assert_eq!(part_one(puzzle_input), 21);
}

#[test]
fn part_two_examples() {
    let puzzle_input = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};
    assert_eq!(part_two(puzzle_input), 8);
}
