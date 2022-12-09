use regex::Regex;
use lazy_static::lazy_static;

pub fn day_five(input: String) {
    println!(
        "stack top: {:?}",
        part_one(input.as_ref())
    );
}

pub fn part_one(input: &str) -> String {
    let cb = CargoBay::new(input);
    cb.stack_top()
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
            .skip_while(|x| !RE_IDX.is_match(x))
            .skip(1)
            .peekable();

        let first_line = lines.peek()
            .expect("No stacks found in cargo bay");

        // get array of whitespace between crates
        let caps : Vec<String> = RE_FIRST
            .captures_iter(&first_line)
            .map(|cap| cap[1].to_string())
            .collect();

        let mut cb = CargoBay {
            stacks: vec![Vec::new(); caps.len()]
        };

        // prepare regex with exact whitespace matching
        let re_str = caps.iter().fold(
            String::new(), |acc, cap| {
                acc + &format!("(?:{}(?:   |\\[(\\w)\\])", cap)
            });
        let re_str = re_str + &")?".repeat(caps.len());
        let re = Regex::new(re_str.as_str()).unwrap();

        for line in lines {
            let caps = match re.captures(&line) {
                Some(expr) => expr,
                None => continue,
            };
            for idx in 0..cb.stacks.len() {
                if let Some(cargo) = caps.get(idx + 1) {
                    cb.stacks[idx].push(
                        cargo.as_str().chars().next().unwrap());
                }
            }
        }
        cb
    }

    pub fn stack_top(&self) -> String {
        // self.stacks.iter().fold(
        //     String::new(),
        //     |acc, stack| acc + &stack.last()
        //         .map_or("".to_owned(), |c| c.to_string())
        // )
        self.stacks.iter().fold(
            String::new(),
            |acc, stack| acc + &stack.last().unwrap_or(&' ').to_string()
        )
    }

    pub fn operate_crane(&mut self, line: &str) {
        lazy_static! {
            static ref RE_LINE: Regex = Regex::new(
                r"move\s*(\d+)\s*from\s*(\d+)\s*to\s*(\d+)").unwrap();
        }

        let caps = RE_LINE.captures(line).unwrap();

        let amount : usize = caps.get(1).unwrap()
            .as_str().parse().unwrap();

        let source_stack : usize = caps.get(2).unwrap()
            .as_str().parse().unwrap();
        let target_stack : usize = caps.get(3).unwrap()
            .as_str().parse().unwrap();

        for _ in 0..amount {
            if let Some(cargo) = self.stacks[source_stack - 1].pop() {
                self.stacks[target_stack - 1].push(cargo);
            }
        }
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
        assert_eq!(cb.stack_top(), "Z");

        let cb = CargoBay { stacks: vec![
            vec!['D'],
            vec!['C', 'A'],
            vec!['A', 'C'],
            vec!['F']
        ]};
        assert_eq!(cb.stack_top(), "DACF");
    }
}
