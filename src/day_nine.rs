use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
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
    let mut field = Field::default();
    let movements = parse_input(input);

    for ref m in movements {
        field.apply_move(m);
    }

    field.visited.len().try_into().unwrap()
}

pub fn part_two(input: &str) -> u32 {
    todo!();
}

#[derive(PartialEq, Eq, Debug)]
enum MoveDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Debug)]
struct Move {
    direction: MoveDirection,
    distance: u32,
}

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_moore(&self, other: &Point) -> bool {
        self.x <= other.x + 1
            && self.x >= other.x - 1
            && self.y <= other.y + 1
            && self.y >= other.y - 1
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Field {
    head: Point,
    tail: Point,
    visited: HashSet<Point>,
}

impl Default for Field {
    fn default() -> Self {
        Field {
            head: Point::default(),
            tail: Point::default(),
            visited: HashSet::from([Point::default()]),
        }
    }
}

impl Field {
    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.distance {
            self.single_step(&m.direction);
        }
    }

    fn single_step(&mut self, direction: &MoveDirection) {
        match direction {
            MoveDirection::Up => self.head.y += 1,
            MoveDirection::Right => self.head.x += 1,
            MoveDirection::Down => self.head.y -= 1,
            MoveDirection::Left => self.head.x -= 1,
        }

        if self.head.is_moore(&self.tail) {
            return;
        }

        self.tail = self.head;

        match direction {
            MoveDirection::Up => {
                self.tail.y -= 1;
            }
            MoveDirection::Right => {
                self.tail.x -= 1;
            }
            MoveDirection::Down => {
                self.tail.y += 1;
            }
            MoveDirection::Left => {
                self.tail.x += 1;
            }
        }

        self.visited.insert(self.tail);
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w)\s*(\d+)").unwrap();
        }

        let caps = RE.captures(s).ok_or(Self::Err::new(
            ErrorKind::InvalidData,
            "Couldn't parse move",
        ))?;

        // We now these groups are there bc. the regex matches
        let direction = caps.get(1).unwrap().as_str();
        let distance =
            caps.get(2).unwrap().as_str().parse().map_err(|_| {
                Self::Err::new(ErrorKind::InvalidData, "Couldn't parse move distance")
            })?;

        Ok(Self {
            direction: match direction {
                "U" => MoveDirection::Up,
                "R" => MoveDirection::Right,
                "D" => MoveDirection::Down,
                "L" => MoveDirection::Left,
                _ => {
                    return Err(Self::Err::new(
                        ErrorKind::InvalidData,
                        "Couldn't parse move direction",
                    ))
                }
            },
            distance,
        })
    }
}

fn parse_input(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| line.parse().expect("Can't parse input"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse() {
        let movements = indoc! {"
            R 4
            D 100
            U 1
            L 3
        "};
        assert_eq!(
            parse_input(movements),
            vec![
                Move {
                    direction: MoveDirection::Right,
                    distance: 4
                },
                Move {
                    direction: MoveDirection::Down,
                    distance: 100
                },
                Move {
                    direction: MoveDirection::Up,
                    distance: 1
                },
                Move {
                    direction: MoveDirection::Left,
                    distance: 3
                }
            ]
        );
    }

    #[test]
    fn points_moore() {
        assert!(Point { x: 0, y: 0 }.is_moore(&Point { x: 0, y: 0 }));
        assert!(Point { x: 10, y: 10 }.is_moore(&Point { x: 11, y: 11 }));
        assert!(Point { x: -3, y: 1 }.is_moore(&Point { x: -4, y: 0 }));
        assert!(Point { x: 1, y: 1 }.is_moore(&Point { x: 0, y: 1 }));
        assert!(Point { x: 1, y: 1 }.is_moore(&Point { x: 1, y: 0 }));
    }

    #[test]
    fn points_not_moore() {
        assert!(!Point { x: 0, y: 0 }.is_moore(&Point { x: 0, y: 2 }));
        assert!(!Point { x: 10, y: 10 }.is_moore(&Point { x: 8, y: 8 }));
        assert!(!Point { x: -3, y: 1 }.is_moore(&Point { x: 0, y: 0 }));
        assert!(!Point { x: 1, y: 1 }.is_moore(&Point { x: 10, y: -10 }));
        assert!(!Point { x: 1, y: 1 }.is_moore(&Point { x: -1, y: -1 }));
    }

    #[test]
    fn movement() {
        let mut field = Field::default();

        field.apply_move(&Move {
            direction: MoveDirection::Right,
            distance: 4,
        });
        assert_eq!(field.head, Point { x: 4, y: 0 });
        assert_eq!(field.tail, Point { x: 3, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Up,
            distance: 1,
        });
        assert_eq!(field.head, Point { x: 4, y: 1 });
        assert_eq!(field.tail, Point { x: 3, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Up,
            distance: 1,
        });
        assert_eq!(field.head, Point { x: 4, y: 2 });
        assert_eq!(field.tail, Point { x: 4, y: 1 });

        field.apply_move(&Move {
            direction: MoveDirection::Left,
            distance: 2,
        });
        assert_eq!(field.head, Point { x: 2, y: 2 });
        assert_eq!(field.tail, Point { x: 3, y: 2 });

        field.apply_move(&Move {
            direction: MoveDirection::Up,
            distance: 1,
        });
        assert_eq!(field.head, Point { x: 2, y: 3 });
        assert_eq!(field.tail, Point { x: 3, y: 2 });

        field.apply_move(&Move {
            direction: MoveDirection::Down,
            distance: 2,
        });
        assert_eq!(field.head, Point { x: 2, y: 1 });
        assert_eq!(field.tail, Point { x: 3, y: 2 });
    }

    #[test]
    fn movement_example() {
        let mut field = Field::default();
        let movements = parse_input(indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "});

        for ref m in movements {
            field.apply_move(m);
        }

        assert_eq!(field.head, Point { x: 2, y: 2 });
        assert_eq!(field.tail, Point { x: 1, y: 2 });
    }
}
