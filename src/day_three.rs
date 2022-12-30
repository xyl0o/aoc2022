use itertools::Itertools;
use std::collections::HashSet;
use std::convert::TryInto;

pub fn both(input: &str) {
    part_one(input);
    part_two(input);
}

pub fn part_one(input: &str) -> u32 {
    let prio_sum: u32 = input
        .lines()
        .map(|l| priority(Rucksack::new(l).first_duplicate().unwrap()))
        .sum();

    println!("Sum of prios of duplicates: {:?}", prio_sum);
    prio_sum
}

pub fn part_two(input: &str) -> u32 {
    let mut badge_prio_sum = 0;
    for group in input.lines().chunks(3).into_iter() {
        let eg = ElfGroup {
            elves: group
                .map(|l| Rucksack::new(l))
                .collect::<Vec<Rucksack>>()
                .try_into()
                .unwrap(),
        };
        badge_prio_sum += priority(eg.badge().unwrap());
    }

    println!("Sum of prios of badges: {:?}", badge_prio_sum);
    badge_prio_sum
}

#[derive(Debug, Clone)]
struct Rucksack {
    first_comp: String,
    second_comp: String,
}

impl Rucksack {
    pub fn new(input: &str) -> Self {
        let input_chars = input.chars().count();

        let (first, second) = input.split_at(input_chars / 2);

        Self {
            first_comp: first.to_string(),
            second_comp: second.to_string(),
        }
    }

    pub fn first_duplicate(&self) -> Option<char> {
        let mut chars = HashSet::new();

        chars.extend(self.first_comp.chars());

        for c in self.second_comp.chars() {
            if chars.contains(&c) {
                return Some(c);
            }
        }

        None
    }

    // pub fn iter_chars(&self) -> impl Iterator<Item = &'a char> {
    //     let first_chars = self.first_comp.chars();
    //     let second_chars = self.second_comp.chars();

    //     first_chars.chain(second_chars)
    // }
}

#[derive(Debug)]
struct ElfGroup {
    elves: [Rucksack; 3],
}

impl ElfGroup {
    pub fn badge(&self) -> Option<char> {
        let mut sets = [HashSet::new(), HashSet::new(), HashSet::new()];

        sets[0].extend(self.elves[0].first_comp.chars());
        sets[0].extend(self.elves[0].second_comp.chars());

        sets[1].extend(self.elves[1].first_comp.chars());
        sets[1].extend(self.elves[1].second_comp.chars());

        sets[2].extend(self.elves[2].first_comp.chars());
        sets[2].extend(self.elves[2].second_comp.chars());

        let tmp: HashSet<char> =
            sets[0].intersection(&sets[1]).copied().collect();

        let tmp: HashSet<char> = tmp.intersection(&sets[2]).copied().collect();

        tmp.iter().next().copied()
    }
}

fn priority(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        return (c as u32) - 97 + 1;
    }

    if c.is_ascii_uppercase() {
        return (c as u32) - 65 + 27;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack() {
        let r = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(r.first_comp, "vJrwpWtwJgWr");
        assert_eq!(r.second_comp, "hcsFMMfFFhFp");

        let r = Rucksack::new("PmmdzqPrVvPwwTWBwg");
        assert_eq!(r.first_comp, "PmmdzqPrV");
        assert_eq!(r.second_comp, "vPwwTWBwg");
    }

    #[test]
    fn test_first_duplicate() {
        let r = Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp");
        assert_eq!(r.first_duplicate(), Some('p'));

        let r = Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(r.first_duplicate(), Some('L'));

        let r = Rucksack::new("PmmdzqPrVvPwwTWBwg");
        assert_eq!(r.first_duplicate(), Some('P'));

        let r = Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        assert_eq!(r.first_duplicate(), Some('v'));

        let r = Rucksack::new("ttgJtRGJQctTZtZT");
        assert_eq!(r.first_duplicate(), Some('t'));

        let r = Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!(r.first_duplicate(), Some('s'));

        let r = Rucksack::new("aaBB");
        assert_eq!(r.first_duplicate(), None);
    }

    #[test]
    fn test_priority() {
        let prio = priority('a');
        assert_eq!(prio, 1);

        let prio = priority('z');
        assert_eq!(prio, 26);

        let prio = priority('A');
        assert_eq!(prio, 27);

        let prio = priority('Z');
        assert_eq!(prio, 52);
    }

    #[test]
    fn test_badge() {
        let badge = ElfGroup {
            elves: [
                Rucksack::new("aixB"),
                Rucksack::new("ciyD"),
                Rucksack::new("eizF"),
            ],
        }
        .badge()
        .unwrap();
        let badge_prio = priority(badge);
        assert_eq!(badge, 'i');
        assert_eq!(badge_prio, 9);

        let badge = ElfGroup {
            elves: [
                Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
                Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
                Rucksack::new("PmmdzqPrVvPwwTWBwg"),
            ],
        }
        .badge()
        .unwrap();
        let badge_prio = priority(badge);
        assert_eq!(badge, 'r');
        assert_eq!(badge_prio, 18);

        let badge = ElfGroup {
            elves: [
                Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
                Rucksack::new("ttgJtRGJQctTZtZT"),
                Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
            ],
        }
        .badge()
        .unwrap();
        let badge_prio = priority(badge);
        assert_eq!(badge, 'Z');
        assert_eq!(badge_prio, 52);
    }
}
