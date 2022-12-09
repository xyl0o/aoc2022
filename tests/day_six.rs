
use aoc2022::day_six;

#[test]
fn part_one_examples() {
    let puzzle_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    assert_eq!(day_six::part_one(puzzle_input), 7);

    let puzzle_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    assert_eq!(day_six::part_one(puzzle_input), 5);

    let puzzle_input = "nppdvjthqldpwncqszvftbrmjlhg";
    assert_eq!(day_six::part_one(puzzle_input), 6);

    let puzzle_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    assert_eq!(day_six::part_one(puzzle_input), 10);

    let puzzle_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    assert_eq!(day_six::part_one(puzzle_input), 11);
}
