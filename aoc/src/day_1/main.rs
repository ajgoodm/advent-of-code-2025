use utils::AocBufReader;

fn main() {
    println!(
        "part 1: {}",
        part_1(AocBufReader::from_string("aoc/src/day_1/data/part_1.txt"))
    );
}

fn part_1(input: AocBufReader) -> usize {
    let instructions = parse_instructions(input);
    let mut dial = Dial::new(99, 50);
    dial.count_zeros(instructions)
}

#[derive(Debug, PartialEq, Eq)]
struct Dial {
    max_val: isize,
    current: isize,
}

impl Dial {
    fn new(max_val: isize, current: isize) -> Self {
        Self { max_val, current }
    }

    fn turn_dial(&mut self, left_or_right: char, n: isize) {
        let mut next = (match left_or_right {
            'L' => self.current - n,
            'R' => self.current + n,
            _ => panic!(),
        } % (self.max_val + 1));
        if next < 0 {
            next += self.max_val + 1;
        }

        self.current = next
    }

    fn count_zeros(&mut self, instructions: Vec<(char, isize)>) -> usize {
        let mut result = 0usize;
        for (left_or_right, n) in instructions {
            self.turn_dial(left_or_right, n);
            if self.current == 0 {
                result += 1;
            }
        }
        result
    }
}

fn parse_instructions(input: impl Iterator<Item = String>) -> Vec<(char, isize)> {
    input
        .map(|line| {
            let left_or_right = line.chars().next().unwrap();
            let clicks: isize = line[1..].parse().unwrap();
            (left_or_right, clicks)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instructions() {
        let input = ["L68".to_string(), "L30".to_string(), "R48".to_string()];
        parse_instructions(input.into_iter());
    }

    #[test]
    fn test_turn_dial() {
        let mut dial = Dial::new(99, 5);
        dial.turn_dial('L', 10);
        assert_eq!(dial, Dial::new(99, 95));
    }

    #[test]
    fn test_example_part_1() {
        let mut dial = Dial::new(99, 50);
        let instructions = parse_instructions(
            [
                "L68".to_string(),
                "L30".to_string(),
                "R48".to_string(),
                "L5".to_string(),
                "R60".to_string(),
                "L55".to_string(),
                "L1".to_string(),
                "L99".to_string(),
                "R14".to_string(),
                "L82".to_string(),
            ]
            .into_iter(),
        );

        assert_eq!(dial.count_zeros(instructions), 3)
    }
}
