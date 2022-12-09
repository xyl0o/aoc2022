
use aoc2022::day_two::{part_one,part_two};
use indoc::indoc;

#[test]
fn part_one_examples() {
    let puzzle_input = indoc! {"
        A Y
        B X
        C Z
    "};
    assert_eq!(part_one(puzzle_input), 15);
}

#[test]
fn part_two_examples() {
    let puzzle_input = indoc! {"
        A Y
        B X
        C Z
    "};
    assert_eq!(part_two(puzzle_input), 12);
}
