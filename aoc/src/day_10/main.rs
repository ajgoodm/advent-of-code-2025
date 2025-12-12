use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use utils::{shortest_path_length, AocBufReader, DijkstraSearchable};

fn main() {
    println!(
        "part 1: {}",
        part_1(AocBufReader::from_string("aoc/src/day_10/data/part_1.txt"))
    );
    println!(
        "part 2: {}",
        part_2(AocBufReader::from_string("aoc/src/day_10/data/part_1.txt"))
    );
}

fn part_1(iter: impl Iterator<Item = String>) -> usize {
    iter.map(|line| {
        let (target, _, machine) = Machine::from_string(line);
        let mut targets: HashSet<Vec<char>> = HashSet::new();
        let target_len = target.len();
        targets.insert(target);

        let start: Vec<char> = (0..target_len).map(|_| '.').collect();
        shortest_path_length(machine, start, targets).unwrap()
    })
    .sum()
}

/// For the first button, iterate through its possible values
/// For each value, iterate through the second button's possible values
/// and so on...
fn part_2_inner(machine: &Machine, target: &[usize]) -> Option<usize> {
    if target.iter().all(|x| *x == 0) {
        return Some(0);
    }

    let target_indicators = joltage_to_indicators(target);
    match machine.combinations_for_indicators.get(&target_indicators) {
        Some(combinations) => combinations
            .iter()
            .filter(|button_set| {
                let mut to_subtract = new_zeros(machine.n_lights);
                for button in button_set.iter() {
                    for joltage_idx in button.iter() {
                        to_subtract[*joltage_idx] += 1;
                    }
                }
                target
                    .iter()
                    .zip(to_subtract.iter())
                    .all(|(t, minus)| *t >= *minus)
            })
            .filter_map(|button_set| {
                let mut new_target = target.to_owned();
                for button in button_set.iter() {
                    for joltage_idx in button.iter() {
                        new_target[*joltage_idx] -= 1;
                    }
                }

                for joltage in new_target.iter_mut() {
                    *joltage /= 2;
                }

                part_2_inner(machine, &new_target).map(|value| 2 * value + button_set.len())
            })
            .min(),
        None => None,
    }
}

/// Sigh this is a sneaky linear system optimization problem, and most folks
/// on the internet have used solvers in their language of choice. I'll
/// try to find a Rust solver and see if I can use it here. I've re-coded
/// different flavors of this solution like 4 times and they all time-out
/// on the list of data
fn part_2(iter: impl Iterator<Item = String>) -> usize {
    iter.map(|line| {
        let (_, target, machine) = Machine::from_string(line.clone());
        part_2_inner(&machine, &target).unwrap()
    })
    .sum()
}

type Button = Vec<usize>;
type ButtonCollection = Vec<Button>;

#[derive(Debug, Clone, PartialEq)]
struct Machine {
    n_lights: usize,
    buttons: Vec<Button>,
    combinations_for_indicators: HashMap<Vec<char>, Vec<ButtonCollection>>,
}

fn new_zeros(len: usize) -> Vec<usize> {
    (0..len).map(|_| 0).collect()
}

fn joltage_to_indicators(joltage: &[usize]) -> Vec<char> {
    joltage
        .iter()
        .map(|x| match *x % 2 == 0 {
            true => '.',
            false => '#',
        })
        .collect()
}

impl Machine {
    fn from_string(s: String) -> (Vec<char>, Vec<usize>, Self) {
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

        let button_powerset: Vec<ButtonCollection> = (0usize..=(buttons.len()))
            .flat_map(|n_buttons| buttons.iter().cloned().combinations(n_buttons))
            .collect();

        let n_lights = indicator_lights.len();
        let mut combinations_for_indicators: HashMap<Vec<char>, Vec<ButtonCollection>> =
            HashMap::new();
        for button_collection in button_powerset.into_iter() {
            let mut joltage = new_zeros(n_lights);
            for button in button_collection.iter() {
                for joltage_idx in button.iter() {
                    joltage[*joltage_idx] += 1;
                }
            }
            let indicators = joltage_to_indicators(&joltage);
            combinations_for_indicators
                .entry(indicators)
                .or_default()
                .push(button_collection)
        }

        (
            indicator_lights,
            joltage,
            Self {
                n_lights,
                buttons,
                combinations_for_indicators,
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
    fn test_part_1_bits() {
        let (lights, _, machine) =
            Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());
        let next = machine.push_button(&lights, 1);
        assert_eq!(next, "..##".chars().collect::<Vec<char>>())
    }

    #[test]
    fn test_part_1_djikstra() {
        let (target, _, machine) =
            Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());
        let mut targets: HashSet<Vec<char>> = HashSet::new();
        targets.insert(target);

        assert_eq!(
            shortest_path_length(machine, Vec::from_iter("....".chars()), targets),
            Some(2)
        )
    }

    #[test]
    fn test_part_2_inner() {
        let (_, target, machine) =
            Machine::from_string("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string());
        assert_eq!(part_2_inner(&machine, &target), Some(10));
    }

    #[test]
    fn test_debug_panic() {
        let (_, target, machine) = Machine::from_string(
            "[####.] (0,2,3,4) (0,1,2,3) (1,4) {19,156,19,19,149}".to_string(),
        );
        part_2_inner(&machine, &target).unwrap();
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                [
                    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
                    "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
                    "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
                ]
                .into_iter()
                .map(|x| x.to_string())
            ),
            33
        )
    }
}
