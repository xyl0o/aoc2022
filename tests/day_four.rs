
use aoc2022::day_four::{part_one,part_two};
use indoc::indoc;

#[test]
fn part_one_examples() {
    let puzzle_input = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};
    assert_eq!(part_one(puzzle_input), 2);
}

#[test]
fn part_two_examples() {
    let puzzle_input = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};
    assert_eq!(part_two(puzzle_input), 4);
}
