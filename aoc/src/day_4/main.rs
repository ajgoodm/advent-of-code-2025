use std::collections::HashSet;

use coord_2d::Coord2D;
use grid::Grid;
use utils::AocBufReader;

fn main() {
    println!(
        "part 1: {}",
        part_1(AocBufReader::from_string("aoc/src/day_4/data/part_1.txt"))
    );
    println!(
        "part 2: {}",
        part_2(AocBufReader::from_string("aoc/src/day_4/data/part_1.txt"))
    );
}

fn count_neighbors(x: &Coord2D<usize>, all: &HashSet<Coord2D<usize>>) -> usize {
    x.neighbors()
        .into_iter()
        .collect::<HashSet<_>>()
        .intersection(all)
        .collect::<Vec<_>>()
        .len()
}

fn part_1(reader: impl Iterator<Item = String>) -> usize {
    let grid: Grid<char> = Grid::from_line_iter(reader);
    let paper_rolls = grid.find('@');
    paper_rolls
        .iter()
        .filter(|x| count_neighbors(x, &paper_rolls) < 4)
        .count()
}

fn part_2(reader: impl Iterator<Item = String>) -> usize {
    let grid: Grid<char> = Grid::from_line_iter(reader);
    let mut paper_rolls = grid.find('@');
    let mut result = 0;

    loop {
        let to_remove = paper_rolls
            .iter()
            .filter(|x| count_neighbors(x, &paper_rolls) < 4)
            .cloned()
            .collect::<HashSet<_>>();
        if to_remove.is_empty() {
            break;
        }
        result += to_remove.len();
        paper_rolls.retain(|x| !to_remove.contains(x));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(
                [
                    "..@@.@@@@.",
                    "@@@.@.@.@@",
                    "@@@@@.@.@@",
                    "@.@@@@..@.",
                    "@@.@@@@.@@",
                    ".@@@@@@@.@",
                    ".@.@.@.@@@",
                    "@.@@@.@@@@",
                    ".@@@@@@@@.",
                    "@.@.@@@.@.",
                ]
                .into_iter()
                .map(|x| x.to_string())
            ),
            13
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                [
                    "..@@.@@@@.",
                    "@@@.@.@.@@",
                    "@@@@@.@.@@",
                    "@.@@@@..@.",
                    "@@.@@@@.@@",
                    ".@@@@@@@.@",
                    ".@.@.@.@@@",
                    "@.@@@.@@@@",
                    ".@@@@@@@@.",
                    "@.@.@@@.@.",
                ]
                .into_iter()
                .map(|x| x.to_string())
            ),
            43
        )
    }
}
