use std::collections::HashSet;

use coord_2d::Coord2D;
use grid::Grid;
use utils::AocBufReader;

fn main() {
    println!(
        "part 1: {}",
        TachyonManifold::new(Grid::from_line_iter(AocBufReader::from_string(
            "aoc/src/day_7/data/part_1.txt"
        )))
        .part_1()
    );
    println!("part 2: {}", part_2());
}

fn part_2() -> usize {
    0
}

struct TachyonManifold {
    grid: Grid<char>,
}

impl TachyonManifold {
    fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }

    fn part_1(self) -> usize {
        let splitters = self.grid.find('^');
        let mut result = 0usize;

        let mut current_row_beams = HashSet::from([self.grid.find_one('S')]);
        for _ in 1..self.grid.n_rows {
            let mut next_row_beams: HashSet<Coord2D<usize>> = HashSet::new();
            for beam in current_row_beams.into_iter() {
                if splitters.contains(&beam) {
                    result += 1;
                    if let Some(left) = beam.west() {
                        next_row_beams.insert(left.south());
                    }
                    let right = beam.east();
                    if right.col < self.grid.n_cols {
                        next_row_beams.insert(right.south());
                    }
                } else {
                    next_row_beams.insert(beam.south());
                }
            }

            current_row_beams = next_row_beams;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let grid: Grid<char> = Grid::from_line_iter(
            [
                ".......S.......",
                "...............",
                ".......^.......",
                "...............",
                "......^.^......",
                "...............",
                ".....^.^.^.....",
                "...............",
                "....^.^...^....",
                "...............",
                "...^.^...^.^...",
                "...............",
                "..^...^.....^..",
                "...............",
                ".^.^.^.^.^...^.",
                "...............",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(TachyonManifold::new(grid).part_1(), 21)
    }
}
