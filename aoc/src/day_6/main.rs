use grid::Grid;
use utils::AocBufReader;

fn main() {
    println!(
        "part 1: {}",
        reduce(parse_input_part_1(AocBufReader::from_string(
            "aoc/src/day_6/data/part_1.txt"
        )))
    );
    println!(
        "part 2: {}",
        reduce(parse_input_part_2(AocBufReader::from_string(
            "aoc/src/day_6/data/part_1.txt"
        )))
    );
}

fn reduce(cols: Vec<(Vec<usize>, char)>) -> usize {
    cols.into_iter()
        .map(|(vals, c)| match c {
            '+' => vals.into_iter().sum::<usize>(),
            '*' => vals.into_iter().product::<usize>(),
            _ => panic!(),
        })
        .sum()
}

fn parse_input_part_1(iter: impl Iterator<Item = String>) -> Vec<(Vec<usize>, char)> {
    let mut lines: Vec<String> = iter.collect();
    let last = lines.pop().unwrap();
    let chars: Vec<char> = last
        .split_whitespace()
        .map(|item| item.chars().next().unwrap())
        .collect();

    let numbers: Vec<Vec<usize>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    let grid = Grid::new(numbers);

    grid.cols().zip(chars).collect()
}

fn parse_input_part_2(iter: impl Iterator<Item = String>) -> Vec<(Vec<usize>, char)> {
    let mut lines: Vec<String> = iter.collect();
    let last = lines.pop().unwrap();
    let chars: Vec<char> = last
        .split_whitespace()
        .map(|item| item.chars().next().unwrap())
        .collect();

    let grid: Grid<char> = Grid::from_line_iter(lines.into_iter());
    let mut numbers: Vec<Vec<usize>> = vec![];
    let mut problem_numbers: Vec<usize> = vec![];
    for col in grid.cols() {
        let col_str = col.into_iter().collect::<String>();
        if col_str.trim().is_empty() {
            numbers.push(std::mem::take(&mut problem_numbers));
        } else {
            problem_numbers.push(col_str.trim().parse().unwrap())
        }
    }
    numbers.push(problem_numbers);

    numbers.into_iter().zip(chars).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            reduce(parse_input_part_1(
                [
                    "123 328  51 64 ",
                    " 45 64  387 23 ",
                    "  6 98  215 314",
                    "*   +   *   +  ",
                ]
                .into_iter()
                .map(|x| x.to_string())
            )),
            4277556
        )
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            reduce(parse_input_part_2(
                [
                    "123 328  51 64 ",
                    " 45 64  387 23 ",
                    "  6 98  215 314",
                    "*   +   *   +  ",
                ]
                .into_iter()
                .map(|x| x.to_string())
            )),
            3263827
        )
    }
}
