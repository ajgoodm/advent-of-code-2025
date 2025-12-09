use std::str::FromStr;

use itertools::Itertools;

use coord_2d::Coord2D;
use utils::AocBufReader;

fn main() {
    println!(
        "part 1: {}",
        part_1(
            AocBufReader::from_string("aoc/src/day_9/data/part_1.txt")
                .map(|x| Coord2D::from_str(&x).unwrap())
                .collect()
        )
    );
    println!("part 2: {}", part_2());
}

fn part_1(tiles: Vec<Coord2D<usize>>) -> usize {
    tiles
        .into_iter()
        .combinations(2)
        .map(|x_y| x_y[0].rectangle_area(&x_y[1]))
        .max()
        .unwrap()
}
fn part_2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3",]
                    .into_iter()
                    .map(|x| Coord2D::from_str(x).unwrap())
                    .collect()
            ),
            50
        )
    }
}
