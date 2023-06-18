use ndarray::{Array1, Array2};
use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct Point2D {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
struct Field {
    heights: Array2<u8>,
    point: Point2D,
}

impl FromStr for Field {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let cols = lines.peek().ok_or("No input")?.len();

        let mut point: Option<Point2D> = None;

        let mut char_parse = |(row, col), chr| match chr {
            c if c.is_ascii_lowercase() => Ok((c as u8) - 97),
            'P' => match point {
                Some(_) => Err("Duplicate point"),
                None => {
                    point = Some(Point2D { x: col, y: row });
                    Ok(0)
                }
            },
            _ => Err("Unsupported symbol"),
        };

        // This does not compile:
        let flat_heights: Array1<u8> = lines
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(col, chr)| char_parse((row, col), chr))
            })
            .collect::<Result<_, _>>()?;

        let rows = flat_heights.len() / cols;

        let heights = flat_heights
            .into_shape((rows, cols))
            .map_err(|_| "Invalid shape")?;
        let point = point.ok_or("No point found")?;

        Ok(Field { heights, point })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn parse_heights() {
        let field: Result<Field, _> = "cab\nabP\nacc".parse();
        assert_eq!(
            field,
            Ok(Field {
                heights: array![[2, 0, 1], [0, 1, 0], [0, 2, 2],],
                point: Point2D { x: 2, y: 1 },
            })
        );
    }
}
