use utils::{index_of_max, AocBufReader};

fn main() {
    println!(
        "part 1: {}",
        part_1(parse_banks(AocBufReader::from_string(
            "aoc/src/day_3/data/part_1.txt"
        )))
    );
    println!(
        "part 2: {}",
        part_2(parse_banks(AocBufReader::from_string(
            "aoc/src/day_3/data/part_1.txt"
        )))
    );
}

fn _joltage(bank: &[usize], n_digits: usize) -> usize {
    if n_digits == 1 {
        let (_, max) = index_of_max(bank);
        return *max;
    }

    let len = bank.len();
    let (idx, digit) = index_of_max(&bank[..(len - (n_digits - 1))]);
    10usize.pow((n_digits - 1).try_into().unwrap()) * digit
        + _joltage(&bank[(idx + 1)..], n_digits - 1)
}

fn part_1(banks: Vec<Vec<usize>>) -> usize {
    banks.into_iter().map(|bank| _joltage(&bank[..], 2)).sum()
}

fn part_2(banks: Vec<Vec<usize>>) -> usize {
    banks.into_iter().map(|bank| _joltage(&bank[..], 12)).sum()
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

    #[test]
    fn test_part_2() {
        let input = [
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .into_iter()
        .map(|x| x.to_string());
        assert_eq!(part_2(parse_banks(input)), 3121910778619);
    }
}
