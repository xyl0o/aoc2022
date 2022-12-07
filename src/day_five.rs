use std::iter::zip;
use regex::Regex;
use lazy_static::lazy_static;

pub fn day_five(input: String) {
    todo!()
}

pub fn part_one(input: &str) -> String {
    todo!()
}

#[derive(Debug)]
struct CargoBay {
    stacks: Vec<Vec<char>>
}

impl CargoBay {
    pub fn new(input: &str) -> Self {
        lazy_static! {
            static ref RE_IDX: Regex = Regex::new(r"^(\s*\d+)+").unwrap();
            static ref RE_FIRST: Regex = Regex::new(r"(\s*)\[\w\]").unwrap();
        }

        let mut lines = input
            .lines()
            .rev()
            .skip_while(|x| RE_IDX.captures(x).is_some())
            .skip(1)
            .peekable();

        let first_line = lines.peek()
            .expect("No stacks found in cargo bay");

        let matches = RE_FIRST.captures(&first_line)
            .expect("Invalid cargo bay");

        let cb = CargoBay {
            stacks: vec![vec![]; matches.len()]
        };

        // pattern_str = ''.join(
        //     "(?:" + m + r"(?:   |\[(\w)\])" for m in matches
        // ) + ")?" * len(matches)
        let re = Regex::new(r"^(\s*\d+)+").unwrap();

        for line in lines {

            let matches = match re.captures(&line) {
                Some(expr) => expr,
                None => return cb,
            };

            for (m, mut s) in zip(matches.iter(), cb.stacks.iter()) {
                if let Some(m) = m {
                    s.push(m);
                }
            }
        }

        cb
    }

    pub fn stack_top(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part_one() {
        let puzzle_input = indoc! {"
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "};
        assert_eq!(part_one(puzzle_input), "CMZ");
    }

    #[test]
    fn test_cargo_bay_creation() {
        let cargo_bay = indoc! {"
                [D]
            [N] [C]
            [Z] [M] [P]
             1   2   3
        "};
        let cb = CargoBay::new(cargo_bay);
        assert_eq!(cb.stack_top(), "NDP");

        let cargo_bay = indoc! {"
            [A]
             1
        "};
        let cb = CargoBay::new(cargo_bay);
        assert_eq!(cb.stack_top(), "A");

        let cargo_bay = indoc! {"
                [B] [T]
            [Z] [M] [P] [X]
             1   2   3   4
        "};
        let cb = CargoBay::new(cargo_bay);
        assert_eq!(cb.stack_top(), "ZBTX");
    }

    #[test]
    fn test_cargo_bay_stack_top() {
        let cb = CargoBay { stacks: vec![
            vec!['A', 'B'],
            vec!['C']
        ]};
        assert_eq!(cb.stack_top(), "BC");

        let cb = CargoBay { stacks: vec![vec!['Z']]};
        assert_eq!(cb.stack_top(), "BC");

        let cb = CargoBay { stacks: vec![
            vec!['D'],
            vec!['C', 'A'],
            vec!['A', 'C'],
            vec!['F']
        ]};
        assert_eq!(cb.stack_top(), "DACF");
    }
}
