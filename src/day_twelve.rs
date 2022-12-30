pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!("Part one: {:?}", part_one_solution);

    let part_two_solution = part_two(input);
    println!("Part two: {:?}", part_two_solution);
}

pub fn part_one(input: &str) -> u32 {
    todo!();
}

pub fn part_two(input: &str) -> u32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const HEIGHTMAP: &str = indoc! {"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "};

    #[test]
    fn foo() {
        todo!();
    }
}
