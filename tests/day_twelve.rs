use aoc2022::day_twelve::{part_one, part_two};
use indoc::indoc;

#[test]
fn part_one_examples() {
    let puzzle_input = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};
    assert_eq!(part_one(puzzle_input), 31);
}

#[test]
fn part_two_examples() {
    let puzzle_input = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};
    assert_eq!(part_two(puzzle_input), 2713310158);
}
