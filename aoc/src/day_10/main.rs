use std::collections::HashSet;

use utils::{shortest_path_length, AocBufReader, DijkstraSearchable};

fn main() {
    println!(
        "part 1: {}",
        part_1(AocBufReader::from_string("aoc/src/day_10/data/part_1.txt"))
    );
    println!("part 2: {}", part_2());
}

fn part_1(iter: impl Iterator<Item = String>) -> usize {
    iter.map(|line| {
        let (target, machine) = Machine::from_string(line);
        let mut targets: HashSet<Vec<char>> = HashSet::new();
        let target_len = target.len();
        targets.insert(target);

        let start: Vec<char> = (0..target_len).map(|_| '.').collect();
        shortest_path_length(machine, start, targets).unwrap()
    })
    .sum()
}

fn part_2() -> usize {
    0
}

#[derive(Debug)]
struct Machine {
    buttons: Vec<Vec<usize>>,
    _joltage: Vec<usize>,
}

impl Machine {
    fn from_string(s: String) -> (Vec<char>, Self) {
        let mut split = s.split_whitespace();
        let indicator_lights: Vec<char> = split
            .next()
            .map(|x| x[1..(x.len() - 1)].chars().collect())
            .unwrap();

        let mut buttons: Vec<Vec<usize>> = vec![];
        let mut joltage: Vec<usize> = vec![];
        for string in split {
            match string.chars().next().unwrap() {
                '(' => buttons.push(
                    string[1..(string.len() - 1)]
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect(),
                ),
                '{' => {
                    joltage = string[1..(string.len() - 1)]
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect()
                }
                _ => panic!(),
            }
        }

        (
            indicator_lights,
            Self {
                buttons,
                _joltage: joltage,
            },
        )
    }

    fn push_button(&self, lights: &[char], button_idx: usize) -> Vec<char> {
        let mut result = lights.to_owned();
        for light_idx in self.buttons[button_idx].iter() {
            let new_char = match lights[*light_idx] {
                '.' => '#',
                '#' => '.',
                _ => panic!(),
            };
            result[*light_idx] = new_char;
        }

        result
    }
}

impl DijkstraSearchable for Machine {
    type Node = Vec<char>;
    type Cost = usize;

    fn neighbors(
        &self,
        previous: &Self::Node,
        previous_cost: Self::Cost,
    ) -> Vec<(Self::Node, Self::Cost)> {
        (0..(self.buttons.len()))
            .map(|button_idx| (self.push_button(previous, button_idx), previous_cost + 1))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_bits() {
        let (lights, machine) =
            Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());
        let next = machine.push_button(&lights, 1);
        assert_eq!(next, "..##".chars().collect::<Vec<char>>())
    }

    #[test]
    fn part_1_djikstra() {
        let (target, machine) =
            Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());
        let mut targets: HashSet<Vec<char>> = HashSet::new();
        targets.insert(lights);

        assert_eq!(
            shortest_path_length(machine, Vec::from_iter("....".chars()), targets),
            Some(2)
        )
    }
}
