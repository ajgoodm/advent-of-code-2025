use grid::Grid;
use utils::AocBufReader;

fn main() {
    println!(
        "part 1: {}",
        part_1(AocBufReader::from_string("aoc/src/day_12/data/part_1.txt"))
    );
}

fn part_1(iter: impl Iterator<Item = String>) -> usize {
    let (gifts, trees) = parse_input(iter);
    trees
        .iter()
        .map(|tree| {
            if tree.not_at_all_possible(&gifts) {
                0
            } else if tree.trivially_possible() {
                1
            } else {
                panic!()
            }
        })
        .sum()
}

fn parse_input(mut lines: impl Iterator<Item = String>) -> (Vec<Gift>, Vec<Tree>) {
    let mut gifts = vec![];
    for _ in 0..6 {
        lines.next();
        let gift_lines = vec![
            lines.next().unwrap(),
            lines.next().unwrap(),
            lines.next().unwrap(),
        ];
        let grid: Grid<char> = Grid::from_line_iter(gift_lines.into_iter());
        gifts.push(Gift { grid });

        lines.next();
    }

    let trees = lines.map(Tree::from_line).collect();
    (gifts, trees)
}

struct Gift {
    grid: Grid<char>,
}

impl Gift {
    fn n_units(&self) -> usize {
        self.grid.find('#').len()
    }
}

struct Tree {
    n_cols: usize,
    n_rows: usize,
    gift_counts: Vec<usize>,
}

impl Tree {
    fn total_area(&self) -> usize {
        self.n_cols * self.n_rows
    }

    fn n_3x3_cells(&self) -> usize {
        (self.n_cols / 3) * (self.n_rows / 3)
    }

    fn not_at_all_possible(&self, gifts: &[Gift]) -> bool {
        self.gift_counts
            .iter()
            .enumerate()
            .map(|(gift_idx, n_gifts)| gifts[gift_idx].n_units() * n_gifts)
            .sum::<usize>()
            > self.total_area()
    }

    fn trivially_possible(&self) -> bool {
        self.gift_counts.iter().sum::<usize>() <= self.n_3x3_cells()
    }

    fn from_line(line: String) -> Self {
        let mut plot_and_cts = line.split(": ");
        let plot = plot_and_cts.next().unwrap();

        let mut cols_rows = plot.split('x');
        let n_cols = cols_rows.next().unwrap().parse().unwrap();
        let n_rows = cols_rows.next().unwrap().parse().unwrap();

        let gift_counts = plot_and_cts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect();

        Self {
            n_cols,
            n_rows,
            gift_counts,
        }
    }
}
