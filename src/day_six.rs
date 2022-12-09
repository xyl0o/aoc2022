use itertools::Itertools;
use std::collections::HashSet;

pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!(
        "Part one: {:?}",
        part_one_solution
    );

    let part_two_solution = part_two(input);
    println!(
        "Part two: {:?}",
        part_two_solution
    );
}

pub fn part_one(input: &str) -> u32 {
    packet_start_pos(input).unwrap()
}

pub fn part_two(input: &str) -> u32 {
    todo!();
}

fn packet_start_pos(stream: &str) -> Option<u32> {
    stream
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .map(|(a, b, c, d)| pairwise_distinct(&a, &b, &c, &d))
        .enumerate()
        .skip_while(|(idx, unique)| !unique)
        .map(|(idx, unique)| (idx + 4) as u32) // change when TryFrom is stable
        .next()
}

fn pairwise_distinct(a: &char, b: &char, c: &char, d: &char) -> bool {
    !(a == b || a == c || a == d || b == c || b == d || c == d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_all_same() {
        assert_eq!(pairwise_distinct(&'a', &'a', &'a', &'a'), false);
        assert_eq!(pairwise_distinct(&'0', &'0', &'0', &'0'), false);
    }

    #[test]
    fn test_distinct_some_same() {
        assert_eq!(pairwise_distinct(&'a', &'b', &'a', &'d'), false);
        assert_eq!(pairwise_distinct(&'x', &'A', &'0', &'x'), false);
        assert_eq!(pairwise_distinct(&'a', &'a', &'a', &'x'), false);
    }

    #[test]
    fn test_distinct_none_same() {
        assert_eq!(pairwise_distinct(&'a', &'b', &'c', &'d'), true);
        assert_eq!(pairwise_distinct(&'x', &'y', &'z', &'#'), true);
    }
}
