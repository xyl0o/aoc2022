use indexmap::IndexMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, VecDeque},
    io::{Error, ErrorKind},
    str::FromStr,
};

pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!("Part one: {:?}", part_one_solution);

    let part_two_solution = part_two(input);
    println!("Part two: {:?}", part_two_solution);
}

pub fn part_one(input: &str) -> u64 {
    let monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|split| split.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    let mut ka = KeepAway::new(monkeys, 3);

    for _ in 0..20 {
        ka.round();
    }

    ka.inspections
        .values()
        .sorted()
        .rev()
        .take(2)
        .copied()
        .fold(1, |acc, x| acc * x as u64)
}

pub fn part_two(input: &str) -> String {
    todo!();
}

type WorryLevel = u64;
type MonkeyId = u32;

#[derive(PartialEq, Debug)]
enum OpArg {
    Old,
    Value(WorryLevel),
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


#[derive(PartialEq, Debug)]
struct Monkey {
    id: MonkeyId,
    items: VecDeque<WorryLevel>,
    op: InfixOp,
    test_div: WorryLevel,
    true_target: MonkeyId,
    false_target: MonkeyId,
}

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
        let id = caps["id"]
            .parse()
            .map_err(|_| Self::Err::new(ErrorKind::InvalidData, "Invalid monkey id"))?;

        // We now items is there bc. the regex matches
        let items: VecDeque<WorryLevel> = caps["items"]
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
        let test_div = caps["divtest"]
            .parse()
            .map_err(|_| Self::Err::new(ErrorKind::InvalidData, "Invalid divisor"))?;

        // We now truetgt is there bc. the regex matches
        let true_target = caps["truetgt"]
            .parse()
            .map_err(|_| Self::Err::new(ErrorKind::InvalidData, "Invalid true target monkey id"))?;

        // We now falsetgt is there bc. the regex matches
        let false_target = caps["falsetgt"].parse().map_err(|_| {
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

type MonkeyThrow = (MonkeyId, WorryLevel);

impl Monkey {
    fn inspect(&self, item: WorryLevel) -> WorryLevel {
        let left_op = match self.op.left {
            OpArg::Old => item,
            OpArg::Value(val) => val,
        };

        let right_op = match self.op.right {
            OpArg::Old => item,
            OpArg::Value(val) => val,
        };

        match self.op.op {
            Op::Multiply => left_op * right_op,
            Op::Add => left_op + right_op,
        }
    }

    fn throw_target(&self, item: WorryLevel) -> MonkeyId {
        match item % self.test_div {
            0 => self.true_target,
            _ => self.false_target,
        }
    }

    /// Each monkey has several attributes:
    /// - Starting items lists your worry level for each item the monkey
    ///   is currently holding in the order they will be inspected.
    /// - Operation shows how your worry level changes as that monkey inspects
    ///   an item. (An operation like new = old * 5 means that your worry level
    ///   after the monkey inspected the item is five times whatever
    ///   your worry level was before inspection.)
    /// - Test shows how the monkey uses your worry level to decide
    ///   where to throw an item next.
    ///   - If true shows what happens with an item if the Test was true.
    ///   = If false shows what happens with an item if the Test was false.
    ///
    /// After each monkey inspects an item but before it tests your
    /// worry level, your relief that the monkey's inspection didn't
    /// damage the item causes your worry level to be divided by three
    /// and rounded down to the nearest integer.
    fn inspect_and_throw(&mut self) -> Option<MonkeyThrow> {
        self.inspect_and_throw_worried(3)
    }

    fn inspect_and_throw_worried(&mut self, worry_div: WorryLevel) -> Option<MonkeyThrow> {
        let item = self.items.pop_front()?;
        let item = self.inspect(item);
        let item = item / worry_div;

        Some((self.throw_target(item), item))
    }

    fn catch(&mut self, item: WorryLevel) {
        self.items.push_back(item);
    }
}

struct KeepAway {
    monkeys: IndexMap<MonkeyId, Monkey>,
    inspections: HashMap<MonkeyId, u32>,
    worry_div: WorryLevel,
    lcm: WorryLevel,
}

impl KeepAway {
    fn new(monkeys: impl IntoIterator<Item = Monkey>, worry_div: WorryLevel) -> Self {
        let mut idxmap = IndexMap::new();

        for monkey in monkeys {
            idxmap.insert(monkey.id, monkey);
        }

        let lcm = idxmap.values().map(|m| m.test_div).fold(1, |acc, x| acc * x);

        Self {
            monkeys: idxmap,
            inspections: HashMap::default(),
            worry_div,
            lcm,
        }
    }
    /// The monkeys take turns inspecting and throwing items. On a single
    /// monkey's turn, it inspects and throws all of the items it is holding
    /// one at a time and in the order listed. Monkey 0 goes first,
    /// then monkey 1, and so on until each monkey has had one turn.
    /// The process of each monkey taking a single turn is called a round.
    fn round(&mut self) {
        let keys: Vec<_> = self.monkeys.keys().cloned().collect();
        for monkey_id in keys {
            let monkey = self.monkeys.get_mut(&monkey_id).unwrap();

            // collect so monkey drops and second mut borrow is possible
            let thrown: Vec<MonkeyThrow> =
                std::iter::from_fn(|| monkey.inspect_and_throw_worried(self.worry_div)).collect();

            let thrown_len = thrown.len() as u32;
            self.inspections
                .entry(monkey_id)
                .and_modify(|e| *e += thrown_len)
                .or_insert(thrown_len);

            for (monkey_id, item) in thrown {
                self.monkeys
                    .get_mut(&monkey_id)
                    .expect("Invalid target monkey")
                    .catch(item % self.lcm);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const MONKEYS: [&str; 4] = [
        indoc! {"
            Monkey 0:
              Starting items: 79, 98
              Operation: new = old * 19
              Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3"},
        indoc! {"
            Monkey 1:
              Starting items: 54, 65, 75, 74
              Operation: new = old + 6
              Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0"},
        indoc! {"
            Monkey 2:
              Starting items: 79, 60, 97
              Operation: new = old * old
              Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3"},
        indoc! {"
            Monkey 3:
              Starting items: 74
              Operation: new = old + 3
              Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1"},
    ];

    #[test]
    fn parse_monkey() {
        let monkey = dbg!(MONKEYS[0].parse::<Monkey>());
        assert!(monkey.is_ok());
        assert_eq!(
            monkey.unwrap(),
            Monkey {
                id: 0,
                items: vec![79, 98].into(),
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

        let monkey = dbg!(MONKEYS[1].parse::<Monkey>());
        assert!(monkey.is_ok());
        assert_eq!(
            monkey.unwrap(),
            Monkey {
                id: 1,
                items: vec![54, 65, 75, 74].into(),
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

        let monkey = dbg!(MONKEYS[2].parse::<Monkey>());
        assert!(monkey.is_ok());
        assert_eq!(
            monkey.unwrap(),
            Monkey {
                id: 2,
                items: vec![79, 60, 97].into(),
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

    #[test]
    fn inspect() {
        let mut monkey = Monkey {
            id: 0,
            items: vec![79, 98].into(),
            op: InfixOp {
                left: OpArg::Old,
                op: Op::Multiply,
                right: OpArg::Value(19),
            },
            test_div: 23,
            true_target: 2,
            false_target: 3,
        };

        // Monkey inspects an item with a worry level of 79.
        assert_eq!(monkey.items[0], 79);
        //   Worry level is multiplied by 19 to 1501.
        assert_eq!(monkey.inspect(monkey.items[0]), 1501);
        //   Monkey gets bored with item. Worry level is divided by 3 to 500.
        //   Current worry level is not divisible by 23.
        //   Item with worry level 500 is thrown to monkey 3.
        assert_eq!(monkey.inspect_and_throw(), Some((3, 500)));

        // Monkey inspects an item with a worry level of 98.
        assert_eq!(monkey.items[0], 98);
        //   Worry level is multiplied by 19 to 1862.
        assert_eq!(monkey.inspect(monkey.items[0]), 1862);
        //   Monkey gets bored with item. Worry level is divided by 3 to 620.
        //   Current worry level is not divisible by 23.
        //   Item with worry level 620 is thrown to monkey 3.
        assert_eq!(monkey.inspect_and_throw(), Some((3, 620)));

        let mut monkey = Monkey {
            id: 2,
            items: vec![79, 60, 97].into(),
            op: InfixOp {
                left: OpArg::Old,
                op: Op::Multiply,
                right: OpArg::Old,
            },
            test_div: 13,
            true_target: 1,
            false_target: 3,
        };

        // Monkey inspects an item with a worry level of 79.
        assert_eq!(monkey.items[0], 79);
        //   Worry level is multiplied by itself to 6241.
        assert_eq!(monkey.inspect(monkey.items[0]), 6241);
        //   Monkey gets bored with item. Worry level is divided by 3 to 2080.
        //   Current worry level is divisible by 13.
        //   Item with worry level 2080 is thrown to monkey 1.
        assert_eq!(monkey.inspect_and_throw(), Some((1, 2080)));

        // Monkey inspects an item with a worry level of 60.
        assert_eq!(monkey.items[0], 60);
        //   Worry level is multiplied by itself to 3600.
        assert_eq!(monkey.inspect(monkey.items[0]), 3600);
        //   Monkey gets bored with item. Worry level is divided by 3 to 1200.
        //   Current worry level is not divisible by 13.
        //   Item with worry level 1200 is thrown to monkey 3.
        assert_eq!(monkey.inspect_and_throw(), Some((3, 1200)));
    }

    #[test]
    fn keep_away_round() {
        let mut ka = KeepAway::new(
            MONKEYS
                .iter()
                .map(|m| m.parse().unwrap())
                .collect::<Vec<_>>(),
            3,
        );

        ka.round();
        assert_eq!(ka.monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(ka.monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(ka.monkeys[2].items, vec![]);
        assert_eq!(ka.monkeys[3].items, vec![]);

        ka.round();
        assert_eq!(ka.monkeys[0].items, vec![695, 10, 71, 135, 350]);
        assert_eq!(ka.monkeys[1].items, vec![43, 49, 58, 55, 362]);
        assert_eq!(ka.monkeys[2].items, vec![]);
        assert_eq!(ka.monkeys[3].items, vec![]);
    }

    #[test]
    fn keep_away_inspections() {
        let mut ka = KeepAway::new(
            MONKEYS
                .iter()
                .map(|m| m.parse().unwrap())
                .collect::<Vec<_>>(),
            3,
        );

        for _ in 0..20 {
            ka.round();
        }
        assert_eq!(ka.inspections[&0], 101);
        assert_eq!(ka.inspections[&1], 95);
        assert_eq!(ka.inspections[&2], 7);
        assert_eq!(ka.inspections[&3], 105);
    }
}
