use utils::AocBufReader;

fn main() {
    println!(
        "part 1: {}",
        part_1(AocBufReader::from_string("aoc/src/day_1/data/part_1.txt"))
    );
    println!(
        "part 2: {}",
        part_2(AocBufReader::from_string("aoc/src/day_1/data/part_1.txt"))
    );
}

fn part_1(input: AocBufReader) -> usize {
    let instructions = parse_instructions(input);
    let mut dial = Dial::new(100, 50);
    dial.count_zeros_part_1(instructions)
}

fn part_2(input: AocBufReader) -> usize {
    let instructions = parse_instructions(input);
    let mut dial = Dial::new(100, 50);
    dial.count_zeros_part_2(instructions)
}

#[derive(Debug, PartialEq, Eq)]
struct Dial {
    modulus: isize,
    current: isize,
}

impl Dial {
    fn new(modulus: isize, current: isize) -> Self {
        Self { modulus, current }
    }

    fn turn_dial(&mut self, left_or_right: char, n: isize) -> usize {
        let mut next = (match left_or_right {
            'L' => self.current - n,
            'R' => self.current + n,
            _ => panic!(),
        } % self.modulus);
        if next < 0 {
            next += self.modulus;
        }
        let mut clicks_to_zero = match left_or_right {
            'L' => self.current,
            'R' => self.modulus - self.current,
            _ => panic!(),
        };
        if clicks_to_zero == 0 {
            // if we're starting on zero, this zero-crossing was counted
            // in the previous instruction
            clicks_to_zero += self.modulus
        }
        self.current = next;

        let remaining_clicks = n - clicks_to_zero;
        if remaining_clicks >= 0 {
            (remaining_clicks / self.modulus + 1) as usize
        } else {
            0
        }
    }

    fn count_zeros_part_1(&mut self, instructions: Vec<(char, isize)>) -> usize {
        let mut result = 0usize;
        for (left_or_right, n) in instructions {
            self.turn_dial(left_or_right, n);
            if self.current == 0 {
                result += 1;
            }
        }
        result
    }

    fn count_zeros_part_2(&mut self, instructions: Vec<(char, isize)>) -> usize {
        instructions
            .into_iter()
            .map(|(left_or_right, n)| self.turn_dial(left_or_right, n))
            .sum()
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
        let mut dial = Dial::new(100, 5);
        dial.turn_dial('L', 10);
        assert_eq!(dial, Dial::new(100, 95));
    }

    #[test]
    fn test_example_part_1() {
        let mut dial = Dial::new(100, 50);
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

        assert_eq!(dial.count_zeros_part_1(instructions), 3)
    }

    #[test]
    fn test_example_part_2() {
        let mut dial = Dial::new(100, 50);
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

        assert_eq!(dial.count_zeros_part_2(instructions), 6)
    }
}
