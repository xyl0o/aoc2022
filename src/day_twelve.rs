use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use ndarray::{array, Array1, Array2};

use crate::Point2D;

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

struct HeightMap {
    map: Array2<u8>,
    start: Point2D<usize>,
    end: Point2D<usize>,
}

impl FromStr for HeightMap {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter_peek = s.lines().peekable();
        let cols = line_iter_peek.peek().expect("No input").len();

        let mut start: Option<Point2D<usize>> = None;
        let mut end: Option<Point2D<usize>> = None;

        let mut char_parse = |(row, col), chr| {
            match chr {
                c if c.is_ascii_lowercase() => Ok((c as u8) - 97),
                'S' => match start {
                    Some(_) => Err(Self::Err::new(
                        ErrorKind::InvalidData,
                        "Duplicate starting point",
                    )),
                    None => {
                        start = Some(Point2D { x: col, y: row });
                        Ok(0) // a
                    }
                },
                'E' => match end {
                    Some(_) => Err(Self::Err::new(
                        ErrorKind::InvalidData,
                        "Duplicate end point",
                    )),
                    None => {
                        end = Some(Point2D { x: col, y: row });
                        Ok(25) // z
                    }
                },
                _ => Err(Self::Err::new(
                    ErrorKind::InvalidData,
                    "Unsupported symbol in map",
                )),
            }
        };
        // char_parse((row, col), chr)

        let flat_map: Array1<u8> = line_iter_peek
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(|(col, chr)| Ok(5))
            })
            .collect::<Result<_, _>>()?;

        let rows = flat_map.len() / cols;
        let map = flat_map.into_shape((rows, cols)).expect("Invalid shape");

        dbg!(&map);
        dbg!(&start);
        dbg!(&end);

        let start = start
            .ok_or(Self::Err::new(ErrorKind::InvalidData, "No start found"))?;
        let end =
            end.ok_or(Self::Err::new(ErrorKind::InvalidData, "No end found"))?;

        // invert y bc. row num is exactly inverse to coord
        let start = Point2D {
            x: start.x,
            y: rows - start.y,
        };
        let end = Point2D {
            x: end.x,
            y: rows - end.y,
        };

        assert_eq!(map[(start.x, start.y)], 0);
        assert_eq!(map[(end.x, end.y)], 25);

        Ok(HeightMap { map, start, end })
    }
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
    fn parse_map() {
        let heightmap: Result<HeightMap, _> = HEIGHTMAP.parse();
        // assert!(heightmap.is_ok());
        let heightmap = heightmap.unwrap();

        assert_eq!(heightmap.start, Point2D { x: 0, y: 4 });
        assert_eq!(heightmap.start, Point2D { x: 5, y: 2 });

        assert_eq!(
            heightmap.map,
            array![
                [0, 0, 1, 16, 15, 14, 13, 12],
                [0, 1, 2, 17, 24, 23, 23, 11],
                [0, 2, 2, 18, 25, 25, 23, 10],
                [0, 2, 2, 19, 20, 21, 22, 9],
                [0, 1, 3, 4, 5, 6, 7, 8],
            ]
        );
    }
}
