use aoc2022::day_nine::{part_one, part_two};
use indoc::indoc;

#[test]
fn part_one_examples() {
    let puzzle_input = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};
    assert_eq!(part_one(puzzle_input), 13);
}

#[test]
fn part_two_examples() {
    let puzzle_input = indoc! {"
        R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2
    "};
    assert_eq!(part_two(puzzle_input), 13);
}
