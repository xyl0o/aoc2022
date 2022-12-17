use aoc2022::day_six::{part_one, part_two};

#[test]
fn part_one_examples() {
    let puzzle_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(part_one(puzzle_input), 7);

    let puzzle_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part_one(puzzle_input), 5);

    let puzzle_input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part_one(puzzle_input), 6);

    let puzzle_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part_one(puzzle_input), 10);

    let puzzle_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part_one(puzzle_input), 11);
}

#[test]
fn part_two_examples() {
    let puzzle_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(part_two(puzzle_input), 19);

    let puzzle_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(part_two(puzzle_input), 23);

    let puzzle_input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(part_two(puzzle_input), 23);

    let puzzle_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(part_two(puzzle_input), 29);

    let puzzle_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(part_two(puzzle_input), 26);
}
