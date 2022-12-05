
use std::collections::HashSet;


pub fn day_three(input: String) {
    println!("Rucksack: {:?}", Rucksack::new("vJrwpWtwJgWrhcsFMMfFFhFp"));
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

}
