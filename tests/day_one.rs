
use aoc2022::day_one::{part_one,part_two};
use indoc::indoc;

#[test]
fn part_one_examples() {
    let puzzle_input = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};
    assert_eq!(part_one(puzzle_input), 24000);
}

#[test]
fn part_two_examples() {
    let puzzle_input = indoc! {"
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "};
    assert_eq!(part_two(puzzle_input), 45000);
}
