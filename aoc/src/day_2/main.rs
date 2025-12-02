use span_1d::Span1D;
use utils::AocBufReader;

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
    println!("part 2: {}", part_2());
}

fn part_1(_spans: Vec<Span1D<usize>>) -> usize {
    0
}

fn part_2() -> usize {
    0
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
}
