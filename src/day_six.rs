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
    packet_start_pos_long(input, 14).unwrap()
}

fn packet_start_pos(stream: &str) -> Option<u32> {
    stream
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .map(|(a, b, c, d)| pairwise_distinct(&a, &b, &c, &d))
        .enumerate()
        .skip_while(|(_, unique)| !unique)
        .map(|(idx, _)| (idx + 4) as u32) // change when TryFrom is stable
        .next()
}

fn packet_start_pos_long(stream: &str, window_size: usize) -> Option<u32> {
    let mut stream_chars = stream.chars();
    let mut window : Vec<char> = Vec::new();

    while window.len() < (window_size - 1) {
        window.push(match stream_chars.next() {
            Some(c) => c,
            None => return None,
        });
    }

    for (idx, c) in stream_chars.enumerate() {
        window.push(c);

        if pairwise_distinct_long(&window) {
            return Some((idx + window_size) as u32) // change when TryFrom is stable
        }
        // quite inefficient
        window.remove(0);
    }
    None
}

fn pairwise_distinct(a: &char, b: &char, c: &char, d: &char) -> bool {
    !(a == b || a == c || a == d || b == c || b == d || c == d)
}

fn pairwise_distinct_long(v: &Vec<char>) -> bool {
    // quite inefficient
    let mut set: HashSet<char> = HashSet::new();
    set.extend(v.iter());
    set.len() == v.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_start_window_too_long() {
        let stream = "abc";
        assert_eq!(packet_start_pos_long(stream, 4), None);
        assert_eq!(packet_start_pos_long(stream, 20), None);
    }

    #[test]
    fn test_packet_start_none_found() {
        let stream = "abcabc";
        assert_eq!(packet_start_pos_long(stream, 4), None);
        let stream = "abababababababababababababababab";
        assert_eq!(packet_start_pos_long(stream, 20), None);
    }

    #[test]
    fn test_packet_start_found() {
        let stream = "abcabc";
        assert_eq!(packet_start_pos_long(stream, 3), Some(3));
        let stream = "aaabcdefghijklmnopqrstuvwxyz";
        assert_eq!(packet_start_pos_long(stream, 20), Some(22));
    }

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

    #[test]
    fn test_distinct_long_all_same() {
        assert_eq!(pairwise_distinct_long(&vec!['a', 'a', 'a', 'a']), false);
        assert_eq!(pairwise_distinct_long(&vec!['0', '0', '0', '0']), false);
    }

    #[test]
    fn test_distinct_long_some_same() {
        assert_eq!(pairwise_distinct_long(&vec!['a', 'b', 'a', 'd']), false);
        assert_eq!(pairwise_distinct_long(&vec!['x', 'A', '0', 'x']), false);
        assert_eq!(pairwise_distinct_long(&vec!['a', 'a', 'a', 'x']), false);
    }

    #[test]
    fn test_distinct_long_none_same() {
        assert_eq!(pairwise_distinct_long(&vec!['a', 'b', 'c', 'd']), true);
        assert_eq!(pairwise_distinct_long(&vec!['x', 'y', 'z', '#']), true);
    }

    #[test]
    fn test_distinct_long_variable_length_true() {
        assert_eq!(pairwise_distinct_long(&vec!['a']), true);
        assert_eq!(pairwise_distinct_long(
            &vec!['x', 'y', 'z', 'a', 'b', 'c']), true);
    }

    #[test]
    fn test_distinct_long_variable_length_false() {
        assert_eq!(pairwise_distinct_long(&vec!['a', 'a']), false);
        assert_eq!(pairwise_distinct_long(&vec!['a', 'b']), true);
        assert_eq!(pairwise_distinct_long(
            &vec!['x', 'y', 'z', 'a', 'x', 'c']), false);
        assert_eq!(pairwise_distinct_long(
            &vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a']), false);
    }
}
