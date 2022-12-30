use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashSet,
    fmt,
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
    let mut field = Field::new(10);
    let movements = parse_input(input);

    for ref m in movements {
        field.apply_move(m);
    }

    field.visited.len().try_into().unwrap()
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
    rope: Vec<Point>,
    visited: HashSet<Point>,
}

impl Default for Field {
    fn default() -> Self {
        Field::new(2)
    }
}

impl fmt::Display for Field {
    //     ---
    // fld 321012345678901
    // str 012345678901234 str  fld  neg +dif +min
    //     3...........4..  0    4   -4    2    0
    //  d  ...............  1    3   -3    3    1
    //  i  ...............  2    2   -2    4    2
    //  f  ....H1.........  3    1   -1    5    3
    //     ...s...........  4    0    0    6    4
    //  y  ...............  5   -1    1    7    5
    //     .2.............  6   -2    2    8    6
    //          dif x
    //
    // field_to_string:
    //   str_x = fld_x - min_x
    //   str_y = dif_y + min_y - fld_y
    //
    //   pos = str_x + str_y * dif_x
    //
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_x = self.rope.iter().map(|p| p.x).min().unwrap();
        let max_x = self.rope.iter().map(|p| p.x).max().unwrap();
        let min_y = self.rope.iter().map(|p| p.y).min().unwrap();
        let max_y = self.rope.iter().map(|p| p.y).max().unwrap();
        let delta_x = max_x - min_x;
        let delta_y = max_y - min_y;

        let line = ".".repeat((delta_x + 1).try_into().unwrap()) + "\n";
        let mut field = line.repeat((delta_y + 1).try_into().unwrap());

        for r in self.rope.iter().rev() {
            let str_x = r.x - min_x;
            let str_y = delta_y + min_y - r.y;
            let str_idx: usize =
                (str_x + str_y * (delta_x + 2)).try_into().unwrap();
            field.replace_range(str_idx..(str_idx + 1), "x");
        }
        write!(f, "{}", field.trim())
    }
}

impl Field {
    fn new(rope_len: usize) -> Self {
        Field {
            rope: vec![Point::default(); rope_len],
            visited: HashSet::from([Point::default()]),
        }
    }

    fn apply_move(&mut self, m: &Move) {
        for _ in 0..m.distance {
            self.single_step(&m.direction);
        }
    }

    fn single_step(&mut self, direction: &MoveDirection) {
        match direction {
            MoveDirection::Up => self.rope[0].y += 1,
            MoveDirection::Right => self.rope[0].x += 1,
            MoveDirection::Down => self.rope[0].y -= 1,
            MoveDirection::Left => self.rope[0].x -= 1,
        }

        let mut prev = self.rope[0];

        for r in self.rope.iter_mut().skip(1) {
            if !prev.is_moore(&r) {
                //   -21012
                //  2 ppppp
                //  1 p...p
                //  0 p.r.p
                // -1 p...p
                // -2 ppppp
                // corners are only possible when rope.len() > 2
                match prev {
                    Point { x, y } if x == r.x + 0 && y == r.y + 2 => {
                        r.y += 1;
                    }
                    Point { x, y } if x == r.x + 1 && y == r.y + 2 => {
                        r.y += 1;
                        r.x += 1;
                    }
                    Point { x, y } if x == r.x + 2 && y == r.y + 2 => {
                        r.y += 1;
                        r.x += 1;
                    }
                    Point { x, y } if x == r.x + 2 && y == r.y + 1 => {
                        r.y += 1;
                        r.x += 1;
                    }
                    Point { x, y } if x == r.x + 2 && y == r.y + 0 => {
                        r.x += 1;
                    }
                    Point { x, y } if x == r.x + 2 && y == r.y - 1 => {
                        r.y -= 1;
                        r.x += 1;
                    }
                    Point { x, y } if x == r.x + 2 && y == r.y - 2 => {
                        r.y -= 1;
                        r.x += 1;
                    }
                    Point { x, y } if x == r.x + 1 && y == r.y - 2 => {
                        r.y -= 1;
                        r.x += 1;
                    }
                    Point { x, y } if x == r.x + 0 && y == r.y - 2 => {
                        r.y -= 1;
                    }
                    Point { x, y } if x == r.x - 1 && y == r.y - 2 => {
                        r.y -= 1;
                        r.x -= 1;
                    }
                    Point { x, y } if x == r.x - 2 && y == r.y - 2 => {
                        r.y -= 1;
                        r.x -= 1;
                    }
                    Point { x, y } if x == r.x - 2 && y == r.y - 1 => {
                        r.y -= 1;
                        r.x -= 1;
                    }
                    Point { x, y } if x == r.x - 2 && y == r.y + 0 => {
                        r.x -= 1;
                    }
                    Point { x, y } if x == r.x - 2 && y == r.y + 1 => {
                        r.y += 1;
                        r.x -= 1;
                    }
                    Point { x, y } if x == r.x - 2 && y == r.y + 2 => {
                        r.y += 1;
                        r.x -= 1;
                    }
                    Point { x, y } if x == r.x - 1 && y == r.y + 2 => {
                        r.y += 1;
                        r.x -= 1;
                    }
                    _ => unreachable!("Prev to far away"),
                };
            }
            prev = *r;
        }
        self.visited.insert(*self.rope.last().unwrap());
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
                Self::Err::new(
                    ErrorKind::InvalidData,
                    "Couldn't parse move distance",
                )
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
        assert_eq!(field.rope[0], Point { x: 4, y: 0 });
        assert_eq!(field.rope[1], Point { x: 3, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Up,
            distance: 1,
        });
        assert_eq!(field.rope[0], Point { x: 4, y: 1 });
        assert_eq!(field.rope[1], Point { x: 3, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Up,
            distance: 1,
        });
        assert_eq!(field.rope[0], Point { x: 4, y: 2 });
        assert_eq!(field.rope[1], Point { x: 4, y: 1 });

        field.apply_move(&Move {
            direction: MoveDirection::Left,
            distance: 2,
        });
        assert_eq!(field.rope[0], Point { x: 2, y: 2 });
        assert_eq!(field.rope[1], Point { x: 3, y: 2 });

        field.apply_move(&Move {
            direction: MoveDirection::Up,
            distance: 1,
        });
        assert_eq!(field.rope[0], Point { x: 2, y: 3 });
        assert_eq!(field.rope[1], Point { x: 3, y: 2 });

        field.apply_move(&Move {
            direction: MoveDirection::Down,
            distance: 2,
        });
        assert_eq!(field.rope[0], Point { x: 2, y: 1 });
        assert_eq!(field.rope[1], Point { x: 3, y: 2 });
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

        assert_eq!(field.rope[0], Point { x: 2, y: 2 });
        assert_eq!(field.rope[1], Point { x: 1, y: 2 });
    }

    #[test]
    fn movement_long() {
        let mut field = Field::new(10);

        field.apply_move(&Move {
            direction: MoveDirection::Right,
            distance: 4,
        });

        assert_eq!(field.rope[0], Point { x: 4, y: 0 });
        assert_eq!(field.rope[1], Point { x: 3, y: 0 });
        assert_eq!(field.rope[2], Point { x: 2, y: 0 });
        assert_eq!(field.rope[3], Point { x: 1, y: 0 });
        assert_eq!(field.rope[4], Point { x: 0, y: 0 });
        assert_eq!(field.rope[5], Point { x: 0, y: 0 });
        assert_eq!(field.rope[6], Point { x: 0, y: 0 });
        assert_eq!(field.rope[7], Point { x: 0, y: 0 });
        assert_eq!(field.rope[8], Point { x: 0, y: 0 });
        assert_eq!(field.rope[9], Point { x: 0, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Up,
            distance: 4,
        });

        assert_eq!(field.rope[0], Point { x: 4, y: 4 });
        assert_eq!(field.rope[1], Point { x: 4, y: 3 });
        assert_eq!(field.rope[2], Point { x: 4, y: 2 });
        assert_eq!(field.rope[3], Point { x: 3, y: 2 });
        assert_eq!(field.rope[4], Point { x: 2, y: 2 });
        assert_eq!(field.rope[5], Point { x: 1, y: 1 });
        assert_eq!(field.rope[6], Point { x: 0, y: 0 });
        assert_eq!(field.rope[7], Point { x: 0, y: 0 });
        assert_eq!(field.rope[8], Point { x: 0, y: 0 });
        assert_eq!(field.rope[9], Point { x: 0, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Left,
            distance: 3,
        });

        assert_eq!(field.rope[0], Point { x: 1, y: 4 });
        assert_eq!(field.rope[1], Point { x: 2, y: 4 });
        assert_eq!(field.rope[2], Point { x: 3, y: 3 });
        assert_eq!(field.rope[3], Point { x: 3, y: 2 });
        assert_eq!(field.rope[4], Point { x: 2, y: 2 });
        assert_eq!(field.rope[5], Point { x: 1, y: 1 });
        assert_eq!(field.rope[6], Point { x: 0, y: 0 });
        assert_eq!(field.rope[7], Point { x: 0, y: 0 });
        assert_eq!(field.rope[8], Point { x: 0, y: 0 });
        assert_eq!(field.rope[9], Point { x: 0, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Down,
            distance: 1,
        });

        assert_eq!(field.rope[0], Point { x: 1, y: 3 });
        assert_eq!(field.rope[1], Point { x: 2, y: 4 });
        assert_eq!(field.rope[2], Point { x: 3, y: 3 });
        assert_eq!(field.rope[3], Point { x: 3, y: 2 });
        assert_eq!(field.rope[4], Point { x: 2, y: 2 });
        assert_eq!(field.rope[5], Point { x: 1, y: 1 });
        assert_eq!(field.rope[6], Point { x: 0, y: 0 });
        assert_eq!(field.rope[7], Point { x: 0, y: 0 });
        assert_eq!(field.rope[8], Point { x: 0, y: 0 });
        assert_eq!(field.rope[9], Point { x: 0, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Right,
            distance: 4,
        });

        assert_eq!(field.rope[0], Point { x: 5, y: 3 });
        assert_eq!(field.rope[1], Point { x: 4, y: 3 });
        assert_eq!(field.rope[2], Point { x: 3, y: 3 });
        assert_eq!(field.rope[3], Point { x: 3, y: 2 });
        assert_eq!(field.rope[4], Point { x: 2, y: 2 });
        assert_eq!(field.rope[5], Point { x: 1, y: 1 });
        assert_eq!(field.rope[6], Point { x: 0, y: 0 });
        assert_eq!(field.rope[7], Point { x: 0, y: 0 });
        assert_eq!(field.rope[8], Point { x: 0, y: 0 });
        assert_eq!(field.rope[9], Point { x: 0, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Down,
            distance: 1,
        });

        assert_eq!(field.rope[0], Point { x: 5, y: 2 });
        assert_eq!(field.rope[1], Point { x: 4, y: 3 });
        assert_eq!(field.rope[2], Point { x: 3, y: 3 });
        assert_eq!(field.rope[3], Point { x: 3, y: 2 });
        assert_eq!(field.rope[4], Point { x: 2, y: 2 });
        assert_eq!(field.rope[5], Point { x: 1, y: 1 });
        assert_eq!(field.rope[6], Point { x: 0, y: 0 });
        assert_eq!(field.rope[7], Point { x: 0, y: 0 });
        assert_eq!(field.rope[8], Point { x: 0, y: 0 });
        assert_eq!(field.rope[9], Point { x: 0, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Left,
            distance: 5,
        });

        assert_eq!(field.rope[0], Point { x: 0, y: 2 });
        assert_eq!(field.rope[1], Point { x: 1, y: 2 });
        assert_eq!(field.rope[2], Point { x: 2, y: 2 });
        assert_eq!(field.rope[3], Point { x: 3, y: 2 });
        assert_eq!(field.rope[4], Point { x: 2, y: 2 });
        assert_eq!(field.rope[5], Point { x: 1, y: 1 });
        assert_eq!(field.rope[6], Point { x: 0, y: 0 });
        assert_eq!(field.rope[7], Point { x: 0, y: 0 });
        assert_eq!(field.rope[8], Point { x: 0, y: 0 });
        assert_eq!(field.rope[9], Point { x: 0, y: 0 });

        field.apply_move(&Move {
            direction: MoveDirection::Right,
            distance: 2,
        });

        assert_eq!(field.rope[0], Point { x: 2, y: 2 });
        assert_eq!(field.rope[1], Point { x: 1, y: 2 });
        assert_eq!(field.rope[2], Point { x: 2, y: 2 });
        assert_eq!(field.rope[3], Point { x: 3, y: 2 });
        assert_eq!(field.rope[4], Point { x: 2, y: 2 });
        assert_eq!(field.rope[5], Point { x: 1, y: 1 });
        assert_eq!(field.rope[6], Point { x: 0, y: 0 });
        assert_eq!(field.rope[7], Point { x: 0, y: 0 });
        assert_eq!(field.rope[8], Point { x: 0, y: 0 });
        assert_eq!(field.rope[9], Point { x: 0, y: 0 });
    }

    #[test]
    fn field_to_string() {
        let field = Field {
            rope: vec![Point { x: 0, y: 0 }],
            visited: HashSet::new(),
        };
        assert_eq!(field.to_string(), "x");

        let field = Field {
            rope: vec![Point { x: 1, y: 1 }, Point { x: 0, y: 0 }],
            visited: HashSet::new(),
        };
        assert_eq!(
            field.to_string(),
            indoc! {"
            .x
            x.
        "}
            .trim()
        );

        let field = Field {
            rope: vec![Point { x: 5, y: 0 }, Point { x: 0, y: 0 }],
            visited: HashSet::new(),
        };
        assert_eq!(
            field.to_string(),
            indoc! {"
            x....x
        "}
            .trim()
        );

        let field = Field {
            rope: vec![
                Point { x: -2, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 3, y: -1 },
                Point { x: 0, y: 0 },
            ],
            visited: HashSet::new(),
        };
        assert_eq!(
            field.to_string(),
            indoc! {"
            ..x...
            x.x...
            .....x
        "}
            .trim()
        );

        let field = Field {
            rope: vec![
                Point { x: 4, y: 4 },
                Point { x: 4, y: 3 },
                Point { x: 4, y: 2 },
                Point { x: 3, y: 2 },
                Point { x: 2, y: 2 },
                Point { x: 1, y: 1 },
                Point { x: 0, y: 0 },
                Point { x: 0, y: 0 },
                Point { x: 0, y: 0 },
                Point { x: 0, y: 0 },
            ],
            visited: HashSet::new(),
        };
        assert_eq!(
            field.to_string(),
            indoc! {"
            ....x
            ....x
            ..xxx
            .x...
            x....
        "}
            .trim()
        );
    }
}
