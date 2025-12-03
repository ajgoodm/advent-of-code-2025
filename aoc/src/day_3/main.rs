use utils::{index_of_max, AocBufReader};

fn main() {
    println!(
        "part 1: {}",
        part_1(parse_banks(AocBufReader::from_string(
            "aoc/src/day_3/data/part_1.txt"
        )))
    );
    println!("part 2: {}", part_2());
}

fn _part_1_joltage(bank: Vec<usize>) -> usize {
    let len = bank.len();
    let (idx, first_digt) = index_of_max(&bank[..(len - 1)]);
    let (_, second_digit) = index_of_max(&bank[(idx + 1)..]);

    10 * *first_digt + *second_digit
}

fn part_1(banks: Vec<Vec<usize>>) -> usize {
    banks.into_iter().map(_part_1_joltage).sum()
}

fn part_2() -> usize {
    0
}

fn parse_banks(iter: impl Iterator<Item = String>) -> Vec<Vec<usize>> {
    iter.map(|x| {
        (0..x.len())
            .map(|idx| x[idx..(idx + 1)].parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .into_iter()
        .map(|x| x.to_string());
        assert_eq!(part_1(parse_banks(input)), 357);
    }
}
