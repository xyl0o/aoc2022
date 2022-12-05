use std::collections::HashSet;
use std::iter::zip;


pub fn day_three(input: String) {
    let prio_sum: u32 = input
        .lines()
        .map(|l| {
            priority(
                Rucksack::new(l)
                .first_duplicate()
                .unwrap()
            ) })
        .sum();

    println!("Sum of prios of duplicates: {:?}", prio_sum);
}


#[derive(Debug)]
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

    pub fn first_duplicate(&self) -> Option<char>{
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
    pub fn badge(&self) -> Option<char>{
        let sets = [
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ];

        for (elf, mut set) in zip(self.elves.iter(), sets) {
            set.extend(elf.first_comp.chars());
            set.extend(elf.second_comp.chars());

            // for c in first_chars.chain(second_chars) {
            //     let c = c.clone();
            //     if chars.insert(c) {
            //         return Some(c);
            //     }
            // }
        }

        // let intersection: HashSet<char> = sets[0]
        //     .intersection(&sets[1])
        //     .collect()

        // let intersection: HashSet<char> =
        //     intersection.intersection(&sets[2])
        //     .collect();

        // intersection.iter().next().copied()

        // let intersection = sets
        //     .iter()
        //     .skip(1)
        //     .fold(&sets[0].clone(), |acc, hs| {
        //         acc.intersection(hs).cloned().collect()
        //     });
        // intersection.iter().next().copied()

        let tmp = sets[0]
            .intersection(&sets[1])
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&sets[2])
            .copied()
            .collect::<HashSet<char>>();

        tmp.iter().next().copied()
    }
}

fn priority(c: char) -> u32{

    if c.is_ascii_lowercase() {
        return (c as u32) - 97 + 1
    }

    if c.is_ascii_uppercase() {
        return (c as u32) - 65 + 27
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
        let badge = ElfGroup { elves: [
            Rucksack::new("aixB"),
            Rucksack::new("ciyD"),
            Rucksack::new("eizF"),
        ]}.badge().unwrap();
        let badge_prio = priority(badge);
        assert_eq!(badge, 'i');
        assert_eq!(badge_prio, 18);

        let badge = ElfGroup { elves: [
            Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Rucksack::new("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Rucksack::new("PmmdzqPrVvPwwTWBwg"),
        ]}.badge().unwrap();
        let badge_prio = priority(badge);
        assert_eq!(badge, 'r');
        assert_eq!(badge_prio, 18);

        let badge = ElfGroup { elves: [
            Rucksack::new("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Rucksack::new("ttgJtRGJQctTZtZT"),
            Rucksack::new("CrZsJsPPZsGzwwsLwLmpwMDw"),
        ]}.badge().unwrap();
        let badge_prio = priority(badge);
        assert_eq!(badge, 'Z');
        assert_eq!(badge_prio, 52);
    }
}