use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::VecDeque,
    io::{Error, ErrorKind},
    str::FromStr,
};

pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!("Part one: {:?}", part_one_solution);

    let part_two_solution = part_two(input);
    println!("Part two: {:?}", part_two_solution);
}

pub fn part_one(input: &str) -> i32 {
    let program = parse_program(input);
    let mut cpu = CPU::default();
    cpu.load(&program);

    let stops = [20, 60, 100, 140, 180, 220];
    let mut signal_strengths = 0;

    for stop in stops {
        for _ in cpu.cycles..stop {
            cpu.tick();
        }
        signal_strengths += cpu.reg_x * stop as i32;
    }
    signal_strengths
}

pub fn part_two(input: &str) -> u32 {
    todo!()
}

#[derive(PartialEq, Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

type Program = Vec<Instruction>;

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\w+)(?:\s*(-?\d+))?").unwrap();
        }

        let caps = RE.captures(s).ok_or(Self::Err::new(
            std::io::ErrorKind::InvalidData,
            "Couldn't parse instruction",
        ))?;

        // We now that the first group is there bc. the regex matches
        Ok(match caps.get(1).unwrap().as_str() {
            "addx" => Instruction::Addx(
                caps.get(2)
                    .ok_or(Self::Err::new(ErrorKind::InvalidData, "Addx needs a value"))?
                    .as_str()
                    .parse()
                    .map_err(|_| {
                        Self::Err::new(ErrorKind::InvalidData, "Couldn't parse addx value")
                    })?,
            ),
            "noop" => Instruction::Noop,
            _ => {
                return Err(Self::Err::new(
                    ErrorKind::InvalidData,
                    "Unknown instruction",
                ))
            }
        })
    }
}

fn parse_program(input: &str) -> Program {
    input
        .lines()
        .map(|line| line.parse().expect("Can't parse input"))
        .collect()
}

#[derive(Debug)]
struct CPU {
    program: VecDeque<Instruction>,
    reg_x: i32,
    cycles: u32,
    current_inst: Option<Instruction>,
    current_cycles: u32,
}

impl CPU {
    fn tick(&mut self) {
        match self.current_inst {
            Some(Instruction::Noop) => {
                self.current_inst = None;
            }
            Some(Instruction::Addx(val)) => {
                if self.current_cycles == 2 {
                    self.reg_x += val;
                    self.current_inst = None;
                }
            }
            None => { /* no instructions to exec -> loop forever */ }
        }
        if let None = self.current_inst {
            self.current_inst = self.program.pop_front();
            self.current_cycles = 0;
        }
        self.cycles += 1;
        self.current_cycles += 1;
    }

    fn load(&mut self, program: &Program) {
        self.program = program.clone().into();
        self.reg_x = 1;
        self.cycles = 0;
        self.current_inst = None;
        self.current_cycles = 0;
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            program: VecDeque::new(),
            reg_x: 1,
            cycles: 0,
            current_inst: None,
            current_cycles: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn parse() {
        let program = indoc! {"
            noop
            addx 3
            addx -5
        "};
        assert_eq!(
            parse_program(program),
            vec![
                Instruction::Noop,
                Instruction::Addx(3),
                Instruction::Addx(-5)
            ]
        );
    }

    #[test]
    fn tick() {
        let program = vec![
            Instruction::Noop,
            Instruction::Addx(3),
            Instruction::Addx(-5),
        ];
        let mut cpu = CPU::default();

        cpu.load(&program);
        assert_eq!(cpu.cycles, 0);
        assert_eq!(cpu.reg_x, 1);
        assert_eq!(cpu.program.len(), 3);
        assert_eq!(cpu.current_inst, None);

        cpu.tick();
        assert_eq!(cpu.cycles, 1);
        assert_eq!(cpu.reg_x, 1);
        assert_eq!(cpu.program.len(), 2);
        assert_eq!(cpu.current_inst, Some(Instruction::Noop));

        cpu.tick();
        assert_eq!(cpu.cycles, 2);
        assert_eq!(cpu.reg_x, 1);
        assert_eq!(cpu.program.len(), 1);
        assert_eq!(cpu.current_inst, Some(Instruction::Addx(3)));

        cpu.tick();
        assert_eq!(cpu.cycles, 3);
        assert_eq!(cpu.reg_x, 1);
        assert_eq!(cpu.program.len(), 1);
        assert_eq!(cpu.current_inst, Some(Instruction::Addx(3)));

        cpu.tick();
        assert_eq!(cpu.cycles, 4);
        assert_eq!(cpu.reg_x, 4);
        assert_eq!(cpu.program.len(), 0);
        assert_eq!(cpu.current_inst, Some(Instruction::Addx(-5)));

        cpu.tick();
        assert_eq!(cpu.cycles, 5);
        assert_eq!(cpu.reg_x, 4);
        assert_eq!(cpu.program.len(), 0);
        assert_eq!(cpu.current_inst, Some(Instruction::Addx(-5)));

        cpu.tick();
        assert_eq!(cpu.cycles, 6);
        assert_eq!(cpu.reg_x, -1);
        assert_eq!(cpu.program.len(), 0);
        assert_eq!(cpu.current_inst, None);

        cpu.tick();
        assert_eq!(cpu.cycles, 7);
        assert_eq!(cpu.reg_x, -1);
        assert_eq!(cpu.program.len(), 0);
        assert_eq!(cpu.current_inst, None);
    }
}
