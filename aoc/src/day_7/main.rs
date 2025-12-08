use std::collections::{HashMap, HashSet};

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
    println!(
        "part 1: {}",
        TachyonManifold::new(Grid::from_line_iter(AocBufReader::from_string(
            "aoc/src/day_7/data/part_1.txt"
        )))
        .part_2()
    );
}

struct TachyonManifold {
    grid: Grid<char>,
}

impl TachyonManifold {
    fn new(grid: Grid<char>) -> Self {
        Self { grid }
    }

    fn part_1(&self) -> usize {
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

    fn part_2(&self) -> usize {
        let start = self.grid.find_one('S');

        // in classic Advent of Code fashion, this is too slow to do recursively
        // without memoization
        let mut cache: HashMap<(usize, usize), usize> = HashMap::new();
        self.count_timelines(1usize, start.col, &mut cache)
    }

    fn count_timelines(
        &self,
        row_idx: usize,
        col_idx: usize,
        cache: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(val) = cache.get(&(row_idx, col_idx)) {
            return *val;
        }

        if row_idx == self.grid.n_rows - 1 {
            return 1usize;
        }

        let is_splitter = self.grid.get(&Coord2D::new(row_idx, col_idx)).unwrap() == '^';
        let mut n_paths = 0usize;
        if is_splitter {
            if col_idx > 0 {
                n_paths += self.count_timelines(row_idx + 1, col_idx - 1, cache)
            }
            if col_idx < self.grid.n_cols - 1 {
                n_paths += self.count_timelines(row_idx + 1, col_idx + 1, cache)
            }
        } else {
            n_paths += self.count_timelines(row_idx + 1, col_idx, cache)
        }

        cache.insert((row_idx, col_idx), n_paths);
        n_paths
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

    #[test]
    fn test_part2() {
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
        assert_eq!(TachyonManifold::new(grid).part_2(), 40)
    }
}
