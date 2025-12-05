use span_1d::Span1D;
use utils::AocBufReader;

fn main() {
    let (spans, ids) = parse_input(AocBufReader::from_string("aoc/src/day_5/data/part_1.txt"));
    println!("part 1: {}", part_1(spans.clone(), ids));
    println!("part 2: {}", part_2(spans));
}

fn part_1(spans: Vec<Span1D<usize>>, ids: Vec<usize>) -> usize {
    ids.into_iter()
        .filter(|id| spans.iter().any(|span| span.contains(*id)))
        .count()
}

fn part_2(spans: Vec<Span1D<usize>>) -> usize {
    let merged_spans = Span1D::melt(spans);
    merged_spans.into_iter().map(|span| span.len).sum()
}

fn parse_input(mut iter: impl Iterator<Item = String>) -> (Vec<Span1D<usize>>, Vec<usize>) {
    let mut spans = vec![];
    let mut ids = vec![];
    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }

        let mut split = line.split('-');
        let (start, end) = (
            split.next().unwrap().parse::<usize>().unwrap(),
            split.next().unwrap().parse::<usize>().unwrap(),
        );
        spans.push(Span1D::from_start_end_inclusive(start, end));
    }

    for line in iter {
        ids.push(line.parse::<usize>().unwrap());
    }

    (spans, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let (spans, ids) = parse_input(
            [
                "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(spans.len(), 4);
        assert_eq!(ids.len(), 6);
        assert_eq!(spans[0], Span1D::from_start_end_inclusive(3, 5));
    }

    #[test]
    fn test_part_1() {
        let (spans, ids) = parse_input(
            [
                "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
            ]
            .into_iter()
            .map(|x| x.to_string()),
        );
        assert_eq!(part_1(spans, ids), 3);
    }
}
