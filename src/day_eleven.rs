use lazy_static::lazy_static;
use regex::Regex;
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!("Part one: {:?}", part_one_solution);

    let part_two_solution = part_two(input);
    println!("Part two: {:?}", part_two_solution);
}

pub fn part_one(input: &str) -> u32 {
    todo!();
}

pub fn part_two(input: &str) -> String {
    todo!();
}

#[derive(PartialEq, Debug)]
enum OpArg {
    Old,
    Value(u32),
}

#[derive(PartialEq, Debug)]
enum Op {
    Multiply,
    Add,
}

#[derive(PartialEq, Debug)]
struct InfixOp {
    left: OpArg,
    op: Op,
    right: OpArg,
}

type Item = u32;

#[derive(PartialEq, Debug)]
struct Monkey {
    id: u32,
    items: Vec<Item>,
    op: InfixOp,
    test_div: u32,
    true_target: u32,
    false_target: u32,
}

impl Monkey {}

impl FromStr for Monkey {
    type Err = Error;

    /// Parses a monkey from text - format:
    /// Monkey 0:
    ///   Starting items: 79, 98
    ///   Operation: new = old * 19
    ///   Test: divisible by 23
    ///     If true: throw to monkey 2
    ///     If false: throw to monkey 3
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(concat!(
                r"Monkey (?P<id>\d+):\n",
                r"  Starting items: (?P<items>(?:\d+, )*\d+)\n",
                r"  Operation: new = (?P<oparg1>old|\d+) (?P<op>[[:punct:]]) (?P<oparg2>old|\d+)\n",
                r"  Test: divisible by (?P<divtest>\d+)\n",
                r"    If true: throw to monkey (?P<truetgt>\d+)\n",
                r"    If false: throw to monkey (?P<falsetgt>\d+)",
            ))
            .unwrap();
        }

        let caps = RE.captures(s).ok_or(Self::Err::new(
            ErrorKind::InvalidData,
            "Couldn't parse monkey",
        ))?;

        // We now id is there bc. the regex matches
        let id: u32 = caps["id"]
            .parse()
            .map_err(|_| Self::Err::new(ErrorKind::InvalidData, "Invalid monkey id"))?;

        // We now items is there bc. the regex matches
        let items: Vec<Item> = caps["items"]
            .split(", ")
            .map(|item| item.parse())
            .collect::<Result<_, _>>()
            .map_err(|_| Self::Err::new(ErrorKind::InvalidData, "Couldn't parse items"))?;

        // We now oparg1 is there bc. the regex matches
        let oparg1 =
            match &caps["oparg1"] {
                "old" => OpArg::Old,
                val => OpArg::Value(val.parse().map_err(|_| {
                    Self::Err::new(ErrorKind::InvalidData, "Couldn't parse op arg 1")
                })?),
            };

        // We now oparg2 is there bc. the regex matches
        let oparg2 =
            match &caps["oparg2"] {
                "old" => OpArg::Old,
                val => OpArg::Value(val.parse().map_err(|_| {
                    Self::Err::new(ErrorKind::InvalidData, "Couldn't parse op arg 2")
                })?),
            };

        // We now op is there bc. the regex matches
        let op = match &caps["op"] {
            "*" => Ok(Op::Multiply),
            "+" => Ok(Op::Add),
            _ => Err(Self::Err::new(ErrorKind::InvalidData, "Unsupported Op")),
        }?;

        // We now divtest is there bc. the regex matches
        let test_div: u32 = caps["divtest"]
            .parse()
            .map_err(|_| Self::Err::new(ErrorKind::InvalidData, "Invalid divisor"))?;

        // We now truetgt is there bc. the regex matches
        let true_target: u32 = caps["truetgt"]
            .parse()
            .map_err(|_| Self::Err::new(ErrorKind::InvalidData, "Invalid true target monkey id"))?;

        // We now falsetgt is there bc. the regex matches
        let false_target: u32 = caps["falsetgt"].parse().map_err(|_| {
            Self::Err::new(ErrorKind::InvalidData, "Invalid false target monkey id")
        })?;

        Ok(Monkey {
            id,
            items,
            op: InfixOp {
                left: oparg1,
                op,
                right: oparg2,
            },
            test_div,
            true_target,
            false_target,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse_monkey() {
        let monkey = indoc! {"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3"};
        let monkey = dbg!(monkey.parse::<Monkey>());
        assert!(monkey.is_ok());
        assert_eq!(
            monkey.unwrap(),
            Monkey {
                id: 0,
                items: vec![79, 98],
                op: InfixOp {
                    left: OpArg::Old,
                    op: Op::Multiply,
                    right: OpArg::Value(19)
                },
                test_div: 23,
                true_target: 2,
                false_target: 3
            }
        );

        let monkey = indoc! {"
        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0"};
        let monkey = dbg!(monkey.parse::<Monkey>());
        assert!(monkey.is_ok());
        assert_eq!(
            monkey.unwrap(),
            Monkey {
                id: 1,
                items: vec![54, 65, 75, 74],
                op: InfixOp {
                    left: OpArg::Old,
                    op: Op::Add,
                    right: OpArg::Value(6)
                },
                test_div: 19,
                true_target: 2,
                false_target: 0
            }
        );

        let monkey = indoc! {"
        Monkey 2:
          Starting items: 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3"};
        let monkey = dbg!(monkey.parse::<Monkey>());
        assert!(monkey.is_ok());
        assert_eq!(
            monkey.unwrap(),
            Monkey {
                id: 2,
                items: vec![97],
                op: InfixOp {
                    left: OpArg::Old,
                    op: Op::Multiply,
                    right: OpArg::Old
                },
                test_div: 13,
                true_target: 1,
                false_target: 3
            }
        );
    }
}
