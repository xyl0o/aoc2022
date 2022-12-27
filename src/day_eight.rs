use ndarray::{s, Array, Array1, Array2};
use std::cmp;

pub fn both(input: &str) {
    let part_one_solution = part_one(input);
    println!("Part one: {:?}", part_one_solution);

    let part_two_solution = part_two(input);
    println!("Part two: {:?}", part_two_solution);
}

pub fn part_one(input: &str) -> u32 {
    let treemap = parse_map(input);
    let vismap = gen_vismap(&treemap);
    vismap.mapv(|num| if num == 0 { 0 } else { 1 }).sum()
}

pub fn part_two(input: &str) -> u32 {
    let treemap = parse_map(input);
    max_scenic_score(&treemap)
}

fn parse_map(input: &str) -> Array2<u32> {
    let mut line_iter_peek = input.lines().peekable();
    let cols = line_iter_peek.peek().expect("No input").len();
    let treemap: Array1<u32> = line_iter_peek
        .flat_map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Couldn't parse num"))
        })
        .collect();

    let rows = treemap.len() / cols;

    treemap.into_shape((rows, cols)).expect("Invalid shape")
    // .reversed_axes()
}

fn gen_vismap(treemap: &Array2<u32>) -> Array2<u32> {
    let mut vismap: Array2<u32> = Array::zeros(treemap.raw_dim());

    let rows = treemap.nrows();
    let cols = treemap.ncols();

    for row in 0..rows {
        // let row_view = treemap.index_axis(Axis(0), row);

        let mut min_height = -1;
        // for (row, tree) in row_view.indexed_iter() {
        for col in 0..cols {
            let height = treemap[[row, col]] as i32;
            if height > min_height {
                vismap[[row, col]] += 1;
                min_height = height;
            }
            if height >= 9 {
                break;
            }
        }

        let mut min_height = -1;
        for col in (0..cols).rev() {
            let height = treemap[[row, col]] as i32;
            if height > min_height {
                vismap[[row, col]] += 1;
                min_height = height;
            }
            if height >= 9 {
                break;
            }
        }
    }

    for col in 0..cols {
        // let col_view = treemap.index_axis(Axis(0), col);

        let mut min_height = -1;
        // for (row, height) in col_view.indexed_iter() {
        for row in 0..rows {
            let height = treemap[[row, col]] as i32;
            if height > min_height {
                vismap[[row, col]] += 1;
                min_height = height;
            }
            if height >= 9 {
                break;
            }
        }

        let mut min_height = -1;
        for row in (0..rows).rev() {
            let height = treemap[[row, col]] as i32;
            if height > min_height {
                vismap[[row, col]] += 1;
                min_height = height;
            }
            if height >= 9 {
                break;
            }
        }
    }
    vismap
}

fn max_scenic_score(treemap: &Array2<u32>) -> u32 {
    let rows = treemap.nrows();
    let cols = treemap.ncols();

    treemap
        .indexed_iter()
        .map(|((row, col), height)| {
            [
                treemap.slice(s![row, 0..col;-1]),
                treemap.slice(s![row, (col + 1)..cols]),
                treemap.slice(s![0..row;-1, col]),
                treemap.slice(s![(row + 1)..rows, col]),
            ]
            .iter()
            .map(|slice| {
                // if slice.len() == 0 {
                //     0
                // } else {
                //     slice.iter().take_while(|x| *x < height).count() + 1
                // }
                let mut view = 0;
                for tree in slice.iter() {
                    view += 1;
                    if tree >= height {
                        break;
                    }
                }
                view
            })
            .reduce(|acc, x| acc * x)
            .unwrap_or(0)
        })
        .reduce(|acc, x| cmp::max(acc, x))
        .expect("Field empty")
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use ndarray::arr2;

    #[test]
    fn read_input() {
        let map = indoc! {"
            30373
            25512
            65332
            33549
            35390
        "};
        let map = parse_map(map);

        assert_eq!(&map[[1, 2]], &5);
        assert_eq!(&map[[1, 3]], &1);
        assert_eq!(&map[[1, 4]], &2);
        assert_eq!(&map[[2, 1]], &5);
        assert_eq!(&map[[2, 2]], &3);
        assert_eq!(&map[[3, 0]], &3);
        assert_eq!(&map[[4, 0]], &3);

        assert_eq!(
            &map,
            arr2(&[
                [3, 0, 3, 7, 3],
                [2, 5, 5, 1, 2],
                [6, 5, 3, 3, 2],
                [3, 3, 5, 4, 9],
                [3, 5, 3, 9, 0]
            ])
        );
    }

    #[test]
    fn read_input_not_square() {
        let map = indoc! {"
            3037
            2551
        "};

        assert_eq!(parse_map(map), arr2(&[[3, 0, 3, 7], [2, 5, 5, 1],]));
    }

    #[test]
    fn generate_vismap() {
        let map = arr2(&[[3, 0, 3, 7], [2, 5, 5, 1]]);
        let vismap = gen_vismap(&map);
        assert_eq!(vismap, arr2(&[[3, 1, 1, 4], [2, 3, 3, 2],]));

        let map = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        let vismap = gen_vismap(&map);
        assert_eq!(
            vismap,
            arr2(&[
                [2, 1, 1, 3, 2],
                [1, 2, 2, 0, 1],
                [4, 1, 0, 1, 1],
                [1, 0, 2, 0, 4],
                [2, 2, 1, 4, 2],
            ])
        );
    }

    #[test]
    fn most_scenic() {
        let map = arr2(&[
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ]);
        let most_scenic = max_scenic_score(&map);
        assert_eq!(most_scenic, 8);
    }

    #[test]
    fn most_scenic_zero() {
        let map = arr2(&[[3, 0, 3, 7], [2, 5, 5, 1]]);
        let most_scenic = max_scenic_score(&map);
        assert_eq!(most_scenic, 0);
    }

    #[test]
    fn most_scenic_one() {
        let map = arr2(&[[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]]);
        let most_scenic = max_scenic_score(&map);
        assert_eq!(most_scenic, 1);
    }
}
