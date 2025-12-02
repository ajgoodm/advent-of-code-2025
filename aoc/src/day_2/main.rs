use span_1d::Span1D;
use utils::{factors, AocBufReader};

fn main() {
    println!(
        "part 1: {}",
        part_1(parse_input(
            AocBufReader::from_string("aoc/src/day_2/data/part_1.txt")
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.to_string())
        ))
    );
    println!(
        "part 2: {}",
        part_2(parse_input(
            AocBufReader::from_string("aoc/src/day_2/data/part_1.txt")
                .next()
                .unwrap()
                .split(',')
                .map(|x| x.to_string())
        ))
    );
}

fn part_1(spans: Vec<Span1D<usize>>) -> usize {
    spans
        .into_iter()
        .map(|span| span.iter().filter(|x| is_invalid_part_1(*x)).sum::<usize>())
        .sum()
}

fn part_2(spans: Vec<Span1D<usize>>) -> usize {
    spans
        .into_iter()
        .map(|span| span.iter().filter(|x| is_invalid_part_2(*x)).sum::<usize>())
        .sum()
}

fn is_invalid_part_1(id: usize) -> bool {
    let s = id.to_string();
    let n_chars = s.len();

    if n_chars % 2 == 1 {
        false
    } else {
        let midpoint = n_chars / 2;
        s[..midpoint] == s[midpoint..]
    }
}

fn is_repeated_substring(s: &str, len: usize) -> bool {
    let mut start = 0usize;
    let mut end = len;

    let to_match = &s[start..end];

    while end <= s.len() {
        if &s[start..end] != to_match {
            return false;
        }
        start += len;
        end += len;
    }
    true
}

fn is_invalid_part_2(id: usize) -> bool {
    let mut divisors = factors(id.to_string().len());
    divisors.pop(); // don't consider a single repetition invalid

    let s = id.to_string();
    for divisor in divisors.into_iter() {
        if is_repeated_substring(&s[..], divisor) {
            return true;
        }
    }
    false
}

fn parse_span(s: String) -> Span1D<usize> {
    let mut split = s.split('-');
    Span1D::from_start_end_inclusive(
        split.next().unwrap().parse::<usize>().unwrap(),
        split.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn parse_input(lines: impl Iterator<Item = String>) -> Vec<Span1D<usize>> {
    lines.into_iter().map(parse_span).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_span() {
        assert_eq!(
            parse_span("11-22".to_string()),
            Span1D::from_start_end_inclusive(11usize, 22usize)
        )
    }

    #[test]
    fn test_parse_input() {
        let example = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        parse_input(example.split(',').map(|x| x.to_string()));
    }

    #[test]
    fn test_is_invalid_part_1() {
        assert!(is_invalid_part_1(1010));
        assert!(is_invalid_part_1(1188511885));
        assert!(!is_invalid_part_1(101));
    }

    #[test]
    fn test_is_invalid_part_2() {
        assert!(is_invalid_part_2(1188511885));
        assert!(is_invalid_part_2(824824824));
        assert!(!is_invalid_part_2(101));
    }
}
